package com.pockethle.app

import android.annotation.SuppressLint
import android.content.res.Configuration
import android.opengl.GLES20
import android.opengl.GLSurfaceView
import android.media.AudioAttributes
import android.media.AudioFormat
import android.media.AudioTrack
import android.os.Bundle
import android.os.Handler
import android.os.Looper
import android.os.SystemClock
import android.view.InputDevice
import android.view.MotionEvent
import android.view.View
import android.widget.ImageButton
import android.widget.ProgressBar
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity
import androidx.appcompat.widget.Toolbar
import java.nio.ByteBuffer
import java.nio.ByteOrder
import org.json.JSONObject
import android.content.pm.ActivityInfo
import android.view.KeyEvent
import android.content.Intent
import android.view.Gravity
import android.widget.FrameLayout
import android.widget.Button
import android.widget.EditText
import android.widget.SeekBar
import androidx.appcompat.app.AlertDialog
import androidx.activity.result.contract.ActivityResultContracts

/**
 * Hosts the emulator output for one game.
 *
 * The implementation drives the emulator session-style (see
 * `pocket-android-jni::runner`): a Rust worker thread runs the
 * emulator and the activity polls the latest framebuffer on the UI
 * thread roughly every 33 ms (~30 Hz), feeds touches and virtual
 * gamepad presses straight back into the kernel, and asks the
 * worker to stop on Back / `onDestroy`. The previous single-shot
 * `NativeBridge.runGame` API blocked until the emulator exited and
 * never streamed intermediate frames, which looked like an infinite
 * loading spinner once the real Unicorn backend was wired up.
 */
class GameActivity : AppCompatActivity() {
    private var controlConfig = JSONObject()
    private val pauseReasons = HashSet<String>()
    private var menuDialog: AlertDialog? = null
    private var menuPauseAcknowledged:Boolean?=null
    private lateinit var floatingMenu: Button
    private val deviceListener = object : android.hardware.input.InputManager.InputDeviceListener {
        override fun onInputDeviceAdded(id: Int) { updateGamepadVisibility() }
        override fun onInputDeviceChanged(id: Int) { onInputDeviceRemoved(id);updateGamepadVisibility() }
        override fun onInputDeviceRemoved(id: Int) {
            if(session!=0L) {
                heldPhysicalKeys.filterKeys { it.first==id }.forEach { (source,vk) -> releaseGuestKey(session,vk);heldPhysicalKeys.remove(source) }
                heldAxisKeys.filterKeys { it.first==id }.forEach { (source,vk) -> releaseGuestKey(session,vk);heldAxisKeys.remove(source) }
            }
            updateGamepadVisibility()
        }
    }
    private fun updateGamepadVisibility() {
        if(!::gameControls.isInitialized) return
        val connected=InputDevice.getDeviceIds().any { id -> InputDevice.getDevice(id)?.let { it.sources and InputDevice.SOURCE_GAMEPAD == InputDevice.SOURCE_GAMEPAD || it.sources and InputDevice.SOURCE_JOYSTICK == InputDevice.SOURCE_JOYSTICK } ?: false }
        val empty=(controlConfig.optJSONArray("keys")?.length() ?: 0)==0 && !controlConfig.optBoolean("keyboard")
        gameControls.visibility=if(empty || (fullscreen && fullscreenMode=="without_controls") || (connected && controlConfig.optBoolean("auto_hide_gamepad",true))) View.GONE else View.VISIBLE
        updateControlsLayout()
    }
    private var inputEpoch=0
    private var activeStylusId = -1
    private val backupProgress = registerForActivityResult(ActivityResultContracts.CreateDocument("application/zip")) { uri ->
        if (uri != null) {
            val id = intent.getStringExtra(EXTRA_GAME_ID).orEmpty()
            val file = java.io.File(cacheDir, "progress-export.zip")
            val result = JSONObject(NativeBridge.saveAction(LibraryPaths.root(this), id, "export", file.absolutePath))
            if (result.optBoolean("ok") && file.exists()) {
                try { requireNotNull(contentResolver.openOutputStream(uri)) { "Cannot open backup destination" }.use { out -> file.inputStream().use { it.copyTo(out) } }; android.widget.Toast.makeText(this, "Progress backup exported", android.widget.Toast.LENGTH_LONG).show() }
                catch (e: Exception) { android.widget.Toast.makeText(this, e.message, android.widget.Toast.LENGTH_LONG).show() }
                finally { file.delete() }
            } else android.widget.Toast.makeText(this, result.optString("error", "No written progress"), android.widget.Toast.LENGTH_LONG).show()
        }
        setPauseReason("backup", false)
    }

    private fun setPauseReason(reason: String, paused: Boolean) {
        if (paused) { releaseHeldInput(); pauseReasons.add(reason) } else pauseReasons.remove(reason)
        if (session != 0L) NativeBridge.nativeSetPaused(session, if (pauseReasons.isEmpty()) 0 else 1)
    }

    private fun openGameMenu() {
        if (session == 0L || menuDialog?.isShowing == true) return
        setPauseReason("menu", true)
        menuPauseAcknowledged=null
        val options = arrayOf("繼續 / Resume", "操作方式 / Controls", "文字輸入 / Text", "旋轉 / Rotate", "放大 / Zoom", "音量 / Volume", "備份進度 / Backup", "全螢幕 / Fullscreen", "返回遊戲庫 / Library")
        menuDialog = AlertDialog.Builder(this).setTitle("正在暫停 / Pausing…")
            .setItems(options) { _, which -> when (which) {
                1 -> { setPauseReason("settings", true); startActivity(Intent(this, ControlsActivity::class.java).putExtra("game_id", intent.getStringExtra(EXTRA_GAME_ID)).putExtra("game_name", title.toString())) }
                2 -> openTextInput()
                3 -> { rotationDegrees = (rotationDegrees + 90) % 360; glRenderer.setRotationDegrees(rotationDegrees); lastFrame?.let { glRenderer.submit(it); surface.requestRender() }; saveRotation() }
                4 -> openDisplaySettings()
                5 -> {
                    setPauseReason("volume",true)
                    val bar = SeekBar(this).apply { max=100; progress=(controlConfig.optDouble("volume",1.0)*100).toInt() }
                    bar.setOnSeekBarChangeListener(object: SeekBar.OnSeekBarChangeListener {
                        override fun onStartTrackingTouch(b: SeekBar?) {}
                        override fun onStopTrackingTouch(b: SeekBar?) {}
                        override fun onProgressChanged(b: SeekBar?, p: Int, user: Boolean) { controlConfig.put("volume",p/100.0); if(session!=0L) NativeBridge.nativeSetVolume(session,p*10) }
                    })
                    AlertDialog.Builder(this).setTitle("Volume").setView(bar).setPositiveButton(android.R.string.ok) { _,_->saveControls() }.setOnDismissListener { setPauseReason("volume",false) }.show()
                }
                6 -> { setPauseReason("backup",true); backupProgress.launch("${intent.getStringExtra(EXTRA_GAME_ID)}-progress.zip") }
                7 -> toggleFullscreenWithControls()
                8 -> { if(session!=0L) NativeBridge.nativeRequestStop(session); finish() }
            } }.setOnDismissListener { menuDialog=null; setPauseReason("menu",false) }.create()
        menuDialog?.show()
        menuDialog?.listView?.isEnabled = NativeBridge.nativeIsPaused(session)==1
    }

    private fun openTextInput() {
        setPauseReason("text",true)
        val input = EditText(this).apply { hint="輸入文字 / Enter text"; maxLines=4 }
        val content=android.widget.LinearLayout(this).apply { orientation=android.widget.LinearLayout.VERTICAL;addView(input) }
        val backspace=Button(this).apply { text="排入客體退格 / Queue guest backspace";setOnClickListener { if(session!=0L) NativeBridge.nativeSendInput(session,NativeBridge.INPUT_TEXT,8,0) } };content.addView(backspace)
        AlertDialog.Builder(this).setTitle("文字輸入 / Committed text").setView(content)
            .setPositiveButton("送出 / Send") { _,_->
                // Deliver committed UTF-16 text after Resume; this is separate
                // from game key mappings and supports Android IME composition.
                setPauseReason("text",false)
                val value=input.text.toString().take(4096)
                if(session!=0L) for(c in value) NativeBridge.nativeSendInput(session,NativeBridge.INPUT_TEXT,c.code,0)
            }.setNegativeButton(android.R.string.cancel,null).setOnDismissListener { setPauseReason("text",false) }.show()
    }

    private fun saveControls() {
        val id=intent.getStringExtra(EXTRA_GAME_ID) ?: return
        val reply=JSONObject(NativeBridge.writeControls(LibraryPaths.root(this),id,controlConfig.toString()))
        if(!reply.optBoolean("ok")) android.widget.Toast.makeText(this,reply.optString("error"),android.widget.Toast.LENGTH_LONG).show()
    }

    private fun saveRotation() {
        val id=intent.getStringExtra(EXTRA_GAME_ID) ?: return
        val current=JSONObject(NativeBridge.readGameSettings(LibraryPaths.root(this),id))
        current.put("rotation",when(rotationDegrees){90->"cw90";180->"half";270->"ccw90";else->"none"})
        NativeBridge.writeGameSettings(LibraryPaths.root(this),id,current.toString())
    }

    private fun openDisplaySettings() {
        setPauseReason("display",true)
        val content=android.widget.LinearLayout(this).apply { orientation=android.widget.LinearLayout.VERTICAL;setPadding(24,8,24,8) }
        for((key,min,max) in listOf(Triple("zoom",1.0,4.0),Triple("zoom_x",0.0,1.0),Triple("zoom_y",0.0,1.0))) {
            val label=android.widget.TextView(this).apply { text=key };content.addView(label)
            val bar=SeekBar(this).apply { this.max=100;progress=((controlConfig.optDouble(key,if(key=="zoom")1.0 else .5)-min)/(max-min)*100).toInt() }
            bar.setOnSeekBarChangeListener(object:SeekBar.OnSeekBarChangeListener {
                override fun onStartTrackingTouch(b:SeekBar?) {}
                override fun onStopTrackingTouch(b:SeekBar?) {}
                override fun onProgressChanged(b:SeekBar?,p:Int,user:Boolean) { val value=min+(max-min)*p/100;controlConfig.put(key,value);label.text="$key: ${"%.2f".format(value)}";applyZoom() }
            });content.addView(bar)
        }
        AlertDialog.Builder(this).setTitle("顯示與放大 / Display and zoom").setView(content).setPositiveButton(android.R.string.ok){_,_->saveControls()}.setOnDismissListener{setPauseReason("display",false)}.show()
    }
    private fun applyZoom() {
        if(!::surface.isInitialized) return
        glRenderer.setView(controlConfig.optDouble("zoom",1.0).toFloat(),controlConfig.optDouble("zoom_x",.5).toFloat(),controlConfig.optDouble("zoom_y",.5).toFloat())
        surface.requestRender()
    }

    private fun reloadControls() {
        val id=intent.getStringExtra(EXTRA_GAME_ID) ?: return
        val reply=JSONObject(NativeBridge.readControls(LibraryPaths.root(this),id))
        if(!reply.optBoolean("ok")) { android.widget.Toast.makeText(this,reply.optString("error"),android.widget.Toast.LENGTH_LONG).show(); return }
        controlConfig=reply.getJSONObject("value")
        releaseHeldInput()
        val old=gameControls
        val parent=old.parent as? FrameLayout ?: return
        parent.removeView(old)
        val panel=FrameLayout(this)
        val density=resources.displayMetrics.density
        panel.layoutParams=FrameLayout.LayoutParams(-1,(210*density).toInt(),Gravity.BOTTOM)
        parent.addView(panel); gameControls=panel; panel.alpha=controlConfig.optDouble("opacity",.7).toFloat()
        val keys=controlConfig.optJSONArray("keys") ?: org.json.JSONArray()
        val landscape=resources.configuration.orientation==Configuration.ORIENTATION_LANDSCAPE
        val layout=controlConfig.optJSONArray(if(landscape) "landscape" else "portrait") ?: org.json.JSONArray()
        val ppcPanel=controlConfig.optString("profile")=="ppc2003"
        if(ppcPanel) {
            panel.setBackgroundColor(android.graphics.Color.rgb(27,44,58))
            val title=android.widget.TextView(this).apply { text="POCKET PC 2003";textSize=10f;setTextColor(android.graphics.Color.rgb(164,191,211));gravity=Gravity.CENTER }
            panel.addView(title,FrameLayout.LayoutParams(-1,(20*density).toInt(),Gravity.BOTTOM))
        }
        val buttons=mutableListOf<Pair<Button,JSONObject>>()
        for(i in 0 until keys.length()) {
            val vk=keys.getInt(i)
            var position:JSONObject?=null
            for(j in 0 until layout.length()) if(layout.getJSONObject(j).optInt("vk")==vk) position=layout.getJSONObject(j)
            val reference=if(ppcPanel) Ppc2003Panel.button(vk) else null
            val point=position ?: reference ?: JSONObject().put("x",.12+(i%5)*.19).put("y",.2+(i/5)*.26).put("size",.17)
            val button=Button(this).apply { text=ControlNames.label(vk); contentDescription=ControlNames.label(vk); textSize=12f; setPadding(0,0,0,0) }
            if(ppcPanel) {
                button.text=reference?.optString("label") ?: ControlNames.label(vk)
                button.setTextColor(android.graphics.Color.rgb(227,237,245))
                button.backgroundTintList=android.content.res.ColorStateList.valueOf(android.graphics.Color.rgb(52,78,98))
                button.stateListAnimator=null
            }
            panel.addView(button); bindControlView(button,vk,reference?.optString("mode")=="click"); buttons.add(button to point)
        }
        panel.addOnLayoutChangeListener { _,_,_,_,_,_,_,_,_->
            for((button,position) in buttons) {
                val size=(position.optDouble("size",.17)*minOf(panel.width,panel.height)).toInt().coerceAtLeast((44*density).toInt())
                val params=FrameLayout.LayoutParams(size,size).apply { leftMargin=(position.optDouble("x")*panel.width-size/2).toInt().coerceIn(0,(panel.width-size).coerceAtLeast(0)); topMargin=(position.optDouble("y")*panel.height-size/2).toInt().coerceIn(0,(panel.height-size).coerceAtLeast(0)) }
                val oldParams=button.layoutParams as? FrameLayout.LayoutParams
                if(oldParams==null || oldParams.width!=size || oldParams.height!=size || oldParams.leftMargin!=params.leftMargin || oldParams.topMargin!=params.topMargin) button.layoutParams=params
            }
            updateControlsLayout()
        }
        if(controlConfig.optBoolean("keyboard")) {
            val keyboard=Button(this).apply { text="⌨"; contentDescription="文字輸入 / Text input"; setOnClickListener { openTextInput() } }
            panel.addView(keyboard); buttons.add(keyboard to JSONObject().put("x",.9).put("y",.82).put("size",.18))
        }
        panel.visibility=if((keys.length()==0&&!controlConfig.optBoolean("keyboard")) || (fullscreen&&fullscreenMode=="without_controls")) View.GONE else View.VISIBLE
        updateGamepadVisibility(); surface.post { applyZoom() }
        if(session!=0L) NativeBridge.nativeSetVolume(session,(controlConfig.optDouble("volume",1.0)*1000).toInt())
    }

    private lateinit var surface: GLSurfaceView
    private lateinit var progress: ProgressBar
    private lateinit var status: TextView
    private lateinit var fpsOverlay: TextView
    private lateinit var toolbar: Toolbar
    private lateinit var fullscreenButton: ImageButton
    private lateinit var gameControls: View
    private lateinit var gameArea: View
    private lateinit var glRenderer: FrameRenderer

    /** Cached handle from `nativeStartGame` (`0` once we've finished). */
    @Volatile private var session: Long = 0

    /** Most recent framebuffer the worker produced — held so we can
     * repaint after `surfaceChanged` resizes the SurfaceView even if
     * the worker has not produced a new frame yet. */
    private var lastFrame: FrameSnapshot? = null

    /**
     * j2me-loader-style FPS counter. Counts frames painted to the
     * SurfaceView in a 1-second sliding window and exposes the
     * latest value as the `displayed` text drawn in [paintFrame].
     * Toggleable via the global "Show FPS counter" preference; when
     * disabled the overlay is skipped entirely so it costs nothing.
     */
    private val fpsCounter = FpsCounter()
    @Volatile private var audioRunning = false
    private var audioThread: Thread? = null
    private var audioTrack: AudioTrack? = null

    /** Mirrors `LauncherConfig::show_fps`. Read once at activity
     * start; toggling the global preference mid-game does not
     * affect an already-running session, the same way j2me-loader
     * applies its `Settings.showFps` snapshot at MIDlet start. */
    private var showFps: Boolean = false

    private var fullscreen: Boolean = false
    private var fullscreenMode: String = "with_controls"

    /** Mirrors `LauncherConfig::show_backend_log`. The status panel is
     * a developer aid ("Backend: Unicorn (ARM)"), so it has to be
     * possible to get it out of the picture. */
    private var showBackendLog: Boolean = false

    /** Mirrors `LauncherConfig::controls_opacity`, applied as the
     * control strip's alpha. In landscape the buttons float over the
     * framebuffer, so a solid pad would cover the bottom of the game;
     * dropping the alpha lets the picture through. */
    private var controlsOpacity: Float = 1f

    /**
     * Presentation-only quarter turn from `GameSettings::rotation`,
     * in degrees clockwise.
     *
     * JumpyBall (and Asphalt 2's Motorola Q build) is laid out for a
     * landscape screen but only renders correctly while it believes it
     * is on a 240x320 portrait panel — forcing `screen` to 320x240
     * breaks it. Keeping the guest portrait and rotating the *picture*
     * gives the intended landscape view, which is what the desktop
     * launcher's "Rotate display" combo already does.
     */
    private var rotationDegrees: Int = 0

    /** Android keycode -> guest virtual key currently held. */
    private val heldPhysicalKeys = HashMap<Pair<Int, Int>, Int>()
    /** Guest virtual keys held by either physical or on-screen controls. */
    private val heldGuestKeys = HashMap<Int, Int>()
    private val heldVirtualKeys = HashSet<Int>()
    /** Guest keys currently asserted by a gamepad stick or hat. */
    private val heldAxisKeys = HashMap<Pair<Int, Int>, Int>()
    private var surfacePointerDown = false
    private var lastPointerX = 0
    private var lastPointerY = 0

    private val mainHandler = Handler(Looper.getMainLooper())

    /** Polling tick. ~30 Hz keeps the SurfaceView smooth without
     * burning the CPU on a phone. */
    private val pollTick = object : Runnable {
        override fun run() {
            if (session == 0L) return
            menuDialog?.let { dialog ->
                val acknowledged=NativeBridge.nativeIsPaused(session)!=0
                if(menuPauseAcknowledged!=acknowledged){menuPauseAcknowledged=acknowledged;dialog.setTitle(if(acknowledged) "已暫停 / Paused" else "正在暫停 / Pausing…");dialog.listView?.isEnabled=acknowledged}
            }
            val raw = NativeBridge.nativePollFrame(session)
            if (raw != null) {
                decodeFrame(raw)?.let { frame ->
                    lastFrame = frame
                    paintFrame(frame)
                }
            }
            if (NativeBridge.nativeIsRunning(session) == 0) {
                // Worker exited on its own (game called ExitProcess
                // / hit max_slices / errored out). Reap it so we
                // surface the summary in the status panel.
                finishSession()
                return
            }
            mainHandler.postDelayed(this, POLL_INTERVAL_MS)
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val config = readLauncherConfig()
        fullscreen = config.fullscreen
        fullscreenMode = config.fullscreenMode
        showBackendLog = config.showBackendLog
        controlsOpacity = config.controlsOpacity
        requestedOrientation = orientationFor(config.orientation)
        setContentView(R.layout.activity_game)
        toolbar = findViewById(R.id.toolbar)
        setSupportActionBar(toolbar)
        fullscreenButton = findViewById(R.id.btn_fullscreen)
        fullscreenButton.setOnClickListener { toggleFullscreenWithControls() }
        gameControls = findViewById(R.id.game_controls)
        gameArea = findViewById(R.id.game_area)
        gameControls.alpha = controlsOpacity
        // The control strip overlays the game area, so the padding that
        // keeps the picture clear of it in portrait can only be applied
        // once the strip has been measured — and again whenever its
        // height changes (fullscreen toggle, rotation).
        gameControls.addOnLayoutChangeListener { _, _, _, _, _, _, _, _, _ -> updateControlsLayout() }
        updateFullscreenLayout()
        if (fullscreen) {
            toolbar.visibility = View.GONE
            hideSystemBars()
        }
        supportActionBar?.setDisplayHomeAsUpEnabled(true)

        val name = intent.getStringExtra(EXTRA_GAME_NAME) ?: "PocketHLE"
        title = name

        surface = findViewById(R.id.surface)
        glRenderer = FrameRenderer()
        surface.isFocusableInTouchMode=true
        surface.setEGLContextClientVersion(2)
        surface.setRenderer(glRenderer)
        surface.renderMode = GLSurfaceView.RENDERMODE_WHEN_DIRTY
        progress = findViewById(R.id.progress)
        fpsOverlay = findViewById(R.id.fps_overlay)
        status = findViewById(R.id.status)

        showFps = readShowFpsPreference()
        status.visibility = if (showBackendLog) View.VISIBLE else View.GONE

        wireSurfaceTouchInput()
        wireVirtualGamepad()

        val id = intent.getStringExtra(EXTRA_GAME_ID)
        if (id == null) {
            status.text = getString(R.string.run_failed_no_id)
            status.visibility = View.VISIBLE
            progress.visibility = View.GONE
            return
        }

        val rootDir = LibraryPaths.root(this)
        rotationDegrees = readRotationDegrees(rootDir, id)
        glRenderer.setRotationDegrees(rotationDegrees)
        val handle = NativeBridge.nativeStartGame(rootDir, id)
        if (handle == 0L) {
            progress.visibility = View.GONE
            status.text = "無法啟動程式，請返回遊戲庫重試。 / Could not start program. Return to the library to retry."
            status.visibility = View.VISIBLE
            return
        }
        session = handle
        reloadControls()
        floatingMenu=Button(this).apply { text="☰"; contentDescription="模擬器選單 / Emulator menu"; setOnClickListener { openGameMenu() } }
        findViewById<FrameLayout>(android.R.id.content).addView(floatingMenu,FrameLayout.LayoutParams((48*resources.displayMetrics.density).toInt(),(48*resources.displayMetrics.density).toInt(),Gravity.TOP or Gravity.END))
        updateFullscreenLayout()
        startAudio(handle)
        status.text = "Backend: ${NativeBridge.backendName()}\nRunning…"
        // The spinner gets hidden the moment the first frame arrives.
        mainHandler.postDelayed(pollTick, POLL_INTERVAL_MS)
    }

    /**
     * The activity declares `configChanges="orientation|screenSize"`, so
     * rotating the device does *not* re-inflate the layout — the running
     * emulator session and its GL surface survive. That means the
     * portrait/landscape difference has to be applied by hand here:
     * pad the game area above the controls in portrait, let the picture
     * run full-height under them in landscape.
     */
    override fun onConfigurationChanged(newConfig: Configuration) {
        super.onConfigurationChanged(newConfig)
        updateControlsLayout()
        if(session!=0L) reloadControls()
        if (fullscreen) hideSystemBars()
        // Re-submit the last frame so the picture is re-letterboxed for
        // the new window shape even if the guest is between frames. This
        // deliberately does not go through `paintFrame`, which would
        // count a rotation as a rendered frame in the FPS overlay.
        lastFrame?.let {
            glRenderer.submit(it)
            surface.requestRender()
        }
    }

    override fun onResume() {
        super.onResume()
        (getSystemService(INPUT_SERVICE) as android.hardware.input.InputManager).registerInputDeviceListener(deviceListener, mainHandler)
        val handle = session
        if(handle!=0L) { reloadControls(); setPauseReason("settings",false); setPauseReason("background",false) }
        updateGamepadVisibility()
        if (handle != 0L && !audioRunning) startAudio(handle)
    }

    override fun onPause() {
        (getSystemService(INPUT_SERVICE) as android.hardware.input.InputManager).unregisterInputDeviceListener(deviceListener)
        setPauseReason("background",true)
        releaseHeldInput()
        stopAudio()
        super.onPause()
    }

    override fun onWindowFocusChanged(hasFocus: Boolean) {
        super.onWindowFocusChanged(hasFocus)
        if (hasFocus && fullscreen) hideSystemBars()
        if(::surface.isInitialized && session!=0L) setPauseReason("focus",!hasFocus)
    }

    override fun onSupportNavigateUp(): Boolean {
        if (session != 0L) openGameMenu() else finish()
        return true
    }

    @Deprecated("Deprecated in Java")
    override fun onBackPressed() {
        if(session!=0L) openGameMenu() else super.onBackPressed()
    }

    override fun onDestroy() {
        finishSession()
        mainHandler.removeCallbacksAndMessages(null)
        surface.onPause()
        super.onDestroy()
    }

    /**
     * Stop the emulator if it is still running, free the native
     * session, and surface the textual summary in the status panel.
     */
    private fun finishSession() {
        val handle = session
        if (handle == 0L) return
        releaseHeldInput()
        session = 0
        stopAudio()
        progress.visibility = View.GONE
        // `nativeFinishGame` blocks on the worker thread join, so
        // do it off the UI thread to keep the UI responsive — the
        // join is usually fast (the Stop signal already fired) but
        // a long emulator slice can drag it out a few hundred ms.
        Thread {
            NativeBridge.nativeRequestStop(handle)
            val summary = NativeBridge.nativeFinishGame(handle)
            mainHandler.post {
                if (!isFinishing && !isDestroyed) {
                    status.visibility = View.VISIBLE
                    status.text = "程式已結束，返回遊戲庫可重新啟動。 / Program ended. Return to the library to start again."
                    status.setOnClickListener {
                        AlertDialog.Builder(this).setTitle("診斷明細 / Diagnostic details")
                            .setMessage(summary).setPositiveButton(android.R.string.ok, null).show()
                    }
                    if (::floatingMenu.isInitialized) floatingMenu.visibility = View.GONE
                }
            }
        }.start()
    }

    private fun startAudio(handle: Long) {
        stopAudio()
        audioRunning = true
        audioThread = Thread({
            var track: AudioTrack? = null
            try {
                var packed = 0L
                while (audioRunning && session == handle && packed == 0L) {
                    packed = NativeBridge.nativeAudioFormat(handle)
                    if (packed == 0L) Thread.sleep(20)
                }
                if (!audioRunning || session != handle || packed == 0L) {
                    android.util.Log.w("PocketHLE", "Audio format was not announced by the guest")
                    return@Thread
                }
                val rate = (packed ushr 16).toInt().coerceIn(8000, 48000)
                val channels = (packed and 0xffff).toInt().coerceIn(1, 2)
                val channelMask = if (channels == 2) AudioFormat.CHANNEL_OUT_STEREO else AudioFormat.CHANNEL_OUT_MONO
                val minBuffer = AudioTrack.getMinBufferSize(rate, channelMask, AudioFormat.ENCODING_PCM_16BIT)
                // EverQuest button/attack effects must not queue behind half a
                // second of audio. Keep ~100 ms while respecting the device minimum.
                val bufferSize = maxOf(minBuffer.takeIf { it > 0 } ?: 0, rate * channels * 2 / 10, 4096)
                track = AudioTrack.Builder()
                    .setAudioAttributes(AudioAttributes.Builder()
                        .setUsage(AudioAttributes.USAGE_GAME)
                        .setContentType(AudioAttributes.CONTENT_TYPE_MUSIC)
                        .build())
                    .setAudioFormat(AudioFormat.Builder()
                        .setSampleRate(rate)
                        .setEncoding(AudioFormat.ENCODING_PCM_16BIT)
                        .setChannelMask(channelMask)
                        .build())
                    .setBufferSizeInBytes(bufferSize)
                    .setTransferMode(AudioTrack.MODE_STREAM)
                    .build()
                if (track?.state != AudioTrack.STATE_INITIALIZED) {
                    android.util.Log.e("PocketHLE", "AudioTrack was not initialized")
                    return@Thread
                }
                audioTrack = track
                track.play()
                android.util.Log.i("PocketHLE", "AudioTrack started: ${rate}Hz, ${channels}ch, buffer=${bufferSize}B")
                var reportedAudible = false
                var reportedPlayback = false
                while (audioRunning && session == handle) {
                    val pcm = NativeBridge.nativePollAudio(handle, 4096)
                    if (pcm != null && pcm.isNotEmpty()) {
                        writeAudio(track, pcm)
                        if (!reportedAudible) {
                            val peak = pcm.maxOf { kotlin.math.abs(it.toInt()) }
                            if (peak > 0) {
                                reportedAudible = true
                                android.util.Log.i("PocketHLE", "AudioTrack received non-silent mix: peak=$peak")
                            }
                        }
                        if (!reportedPlayback && track.playbackHeadPosition > 0) {
                            reportedPlayback = true
                            android.util.Log.i("PocketHLE", "AudioTrack output advancing: ${track.playbackHeadPosition} frames")
                        }
                    } else {
                        writeSilence(track, maxOf(channels * rate / 200, 256))
                        Thread.sleep(5)
                    }
                }
            } catch (error: Throwable) {
                android.util.Log.e("PocketHLE", "AudioTrack playback failed", error)
            } finally {
                track?.let {
                    if (it.state == AudioTrack.STATE_INITIALIZED) {
                        android.util.Log.i("PocketHLE", "AudioTrack finished: ${it.playbackHeadPosition} frames, ${it.underrunCount} underruns")
                    }
                }
                try { track?.pause() } catch (_: Throwable) {}
                try { track?.flush() } catch (_: Throwable) {}
                try { track?.release() } catch (_: Throwable) {}
                if (audioTrack === track) audioTrack = null
                if (audioThread === Thread.currentThread()) {
                    audioRunning = false
                    audioThread = null
                }
            }
        }, "pockethle-audio")
        audioThread?.start()
    }

    private fun stopAudio() {
        audioRunning = false
        audioTrack?.pause()
        audioTrack?.flush()
        val oldThread = audioThread
        audioThread = null
        audioTrack = null
        oldThread?.interrupt()
        if (oldThread !== Thread.currentThread()) {
            try { oldThread?.join(250) } catch (_: InterruptedException) { Thread.currentThread().interrupt() }
        }
    }

    private fun writeAudio(track: AudioTrack, pcm: ShortArray) {
        var offset = 0
        while (offset < pcm.size && audioRunning) {
            val written = track.write(pcm, offset, pcm.size - offset, AudioTrack.WRITE_BLOCKING)
            if (written < 0 && audioRunning) {
                throw IllegalStateException("AudioTrack.write failed: $written")
            }
            if (written <= 0) return
            offset += written
        }
    }

    private fun writeSilence(track: AudioTrack, samples: Int) {
        writeAudio(track, ShortArray(samples))
    }

    // -------------------------------------------------------------------
    // Surface rendering
    // -------------------------------------------------------------------

    private fun decodeFrame(raw: ByteArray): FrameSnapshot? {
        if (raw.size < 8) return null
        val buf = ByteBuffer.wrap(raw).order(ByteOrder.LITTLE_ENDIAN)
        val w = buf.int
        val h = buf.int
        if (w <= 0 || h <= 0) return null
        val pixelBytes = w * h * 4
        if (raw.size < 8 + pixelBytes) return null
        return FrameSnapshot(w, h, raw, 8)
    }

    private fun paintFrame(frame: FrameSnapshot) {
        progress.visibility = View.GONE
        glRenderer.submit(frame)
        surface.requestRender()
        fpsCounter.recordFrame()
        updateFpsOverlay()
    }

    private fun updateFpsOverlay() {
        if (showFps) {
            fpsOverlay.text = "FPS ${fpsCounter.lastSecondCount}"
            fpsOverlay.visibility = View.VISIBLE
        } else {
            fpsOverlay.visibility = View.GONE
        }
    }

    /**
     * Read the global "Show FPS counter" preference from the
     * library's `config.json`. Falls back to `false` if the file is
     * missing or malformed, matching `LauncherConfig::default`.
     */
    private fun readShowFpsPreference(): Boolean {
        val raw = NativeBridge.readConfig(LibraryPaths.root(this))
        return runCatching {
            val obj = JSONObject(raw)
            if (obj.has("ok") && !obj.optBoolean("ok", true)) false
            else obj.optBoolean("show_fps", false)
        }.getOrDefault(false)
    }

    /**
     * Read `GameSettings::rotation` for this game and turn it into
     * degrees clockwise. Presentation only: the guest keeps rendering at
     * whatever `screen` says.
     */
    private fun readRotationDegrees(rootDir: String, id: String): Int {
        val raw = NativeBridge.readGameSettings(rootDir, id)
        val settings = runCatching {
            val obj = JSONObject(raw)
            if (obj.has("ok") && !obj.optBoolean("ok", true)) GameSettings.default()
            else GameSettings.fromJson(obj)
        }.getOrDefault(GameSettings.default())
        return when (settings.rotation) {
            "cw90" -> 90
            "half" -> 180
            "ccw90" -> 270
            else -> 0
        }
    }

    private fun orientationFor(value: String): Int = when (value) {
        "portrait" -> ActivityInfo.SCREEN_ORIENTATION_PORTRAIT
        "landscape" -> ActivityInfo.SCREEN_ORIENTATION_LANDSCAPE
        else -> ActivityInfo.SCREEN_ORIENTATION_UNSPECIFIED
    }

    private fun hideSystemBars() {
        @Suppress("DEPRECATION")
        window.decorView.systemUiVisibility = (
            View.SYSTEM_UI_FLAG_IMMERSIVE_STICKY or
                View.SYSTEM_UI_FLAG_FULLSCREEN or
                View.SYSTEM_UI_FLAG_HIDE_NAVIGATION or
                View.SYSTEM_UI_FLAG_LAYOUT_FULLSCREEN or
                View.SYSTEM_UI_FLAG_LAYOUT_HIDE_NAVIGATION or
                View.SYSTEM_UI_FLAG_LAYOUT_STABLE
            )
        window.decorView.setOnSystemUiVisibilityChangeListener { visibility ->
            if (fullscreen && visibility and View.SYSTEM_UI_FLAG_FULLSCREEN == 0) {
                hideSystemBars()
            }
        }
    }

    private fun toggleFullscreenWithControls() {
        if (fullscreen) {
            exitFullscreenWithControls()
        } else {
            fullscreen = true
            updateFullscreenLayout()
            fullscreenButton.setImageResource(R.drawable.ic_fullscreen_exit)
            hideSystemBars()
        }
    }

    private fun exitFullscreenWithControls() {
        fullscreen = false
        updateFullscreenLayout()
        fullscreenButton.setImageResource(R.drawable.ic_fullscreen)
        showSystemBars()
    }

    private fun updateFullscreenLayout() {
        toolbar.visibility = if (fullscreen) View.GONE else View.VISIBLE
        gameControls.visibility = if (fullscreen && fullscreenMode == "without_controls") View.GONE else View.VISIBLE
        if(controlConfig.optJSONArray("keys")?.length()==0&&!controlConfig.optBoolean("keyboard")) gameControls.visibility=View.GONE
        if(::floatingMenu.isInitialized) floatingMenu.visibility=if(fullscreen) View.VISIBLE else View.GONE
        updateGamepadVisibility()
    }

    /**
     * Keep the framebuffer clear of the overlaid control strip.
     *
     * In portrait there is height to spare, so the game area is padded
     * by exactly the strip's height and the picture sits above the
     * buttons the way it always has. In landscape the window is barely
     * taller than the strip itself — the old vertical LinearLayout is
     * what squeezed the surface into a sliver in the rotated screenshot
     * — so the padding is dropped and the (semi-transparent, see
     * `controls_opacity`) buttons float over the picture instead.
     */
    private fun updateControlsLayout() {
        val landscape = resources.configuration.orientation == Configuration.ORIENTATION_LANDSCAPE
        val strip = if (gameControls.visibility == View.VISIBLE && !landscape) gameControls.height else 0
        if (gameArea.paddingBottom != strip) {
            gameArea.setPadding(0, 0, 0, strip)
        }
    }

    private fun showSystemBars() {
        @Suppress("DEPRECATION")
        window.decorView.systemUiVisibility = View.SYSTEM_UI_FLAG_LAYOUT_STABLE
        window.decorView.setOnSystemUiVisibilityChangeListener(null)
    }

    private fun readLauncherConfig(): LauncherConfig {
        val raw = NativeBridge.readConfig(LibraryPaths.root(this))
        return runCatching {
            val obj = JSONObject(raw)
            if (obj.has("ok") && !obj.optBoolean("ok", true)) LauncherConfig.default()
            else LauncherConfig.fromJson(obj)
        }.getOrDefault(LauncherConfig.default())
    }

    /**
     * j2me-loader-inspired FPS sampler. `recordFrame()` is called
     * once per painted frame; every full second of wall-clock the
     * total is latched into [lastSecondCount] (the number drawn in
     * the overlay) and the running counter is reset. Mirrors the
     * `FpsCounter` class shipped with j2me-loader, just without
     * the periodic `Timer`: we already have a UI repaint cadence,
     * so we sample lazily.
     */
    private class FpsCounter {
        private var windowStart: Long = 0L
        private var inFlight: Int = 0

        @Volatile var lastSecondCount: Int = 0
            private set

        fun recordFrame() {
            val now = SystemClock.uptimeMillis()
            if (windowStart == 0L) {
                windowStart = now
            }
            inFlight += 1
            if (now - windowStart >= 1000L) {
                lastSecondCount = inFlight
                inFlight = 0
                windowStart = now
            }
        }
    }

    // -------------------------------------------------------------------
    // Input plumbing
    // -------------------------------------------------------------------

    /**
     * Android controller keys arrive through the activity key dispatch path,
     * not through the SurfaceView touch listener. Dispatch them before child
     * views consume them and use the same guest codes as the virtual pad.
     * Counts are kept per guest key because Android can report, for example,
     * Y and Start as two different physical keys that share the Gizmondo
     * Start code.
     */
    override fun dispatchKeyEvent(event: KeyEvent): Boolean {
        val bindings=controlConfig.optJSONArray("bindings")
        val reservedStick=event.keyCode==KeyEvent.KEYCODE_BUTTON_THUMBL && (bindings==null || (0 until bindings.length()).none { bindings.getJSONObject(it).optString("input").equals("PadL3",true) })
        if((event.keyCode==KeyEvent.KEYCODE_BUTTON_MODE || reservedStick || ControlNames.input(event.keyCode).equals(controlConfig.optString("menu_key","F1"),true)) && event.action==KeyEvent.ACTION_DOWN && event.repeatCount==0) { if(menuDialog?.isShowing==true)menuDialog?.dismiss() else openGameMenu(); return true }
        if(pauseReasons.isNotEmpty()) return super.dispatchKeyEvent(event)
        if(currentFocus is Button && currentFocus?.parent===gameControls && event.source and InputDevice.SOURCE_GAMEPAD != InputDevice.SOURCE_GAMEPAD) return super.dispatchKeyEvent(event)
        var mapped:Int?=null
        if(bindings!=null) for(i in 0 until bindings.length()) { val b=bindings.getJSONObject(i); if(ControlNames.canonical(b.optString("input"))==ControlNames.canonical(ControlNames.input(event.keyCode,event.source and InputDevice.SOURCE_GAMEPAD == InputDevice.SOURCE_GAMEPAD))) mapped=b.optInt("vk") }
        val vk = mapped ?: virtualKeyFor(event.keyCode) ?: return super.dispatchKeyEvent(event)
        if(vk == 0) return true
        val handle = session
        if (handle == 0L) return super.dispatchKeyEvent(event)
        when (event.action) {
            KeyEvent.ACTION_DOWN -> {
                if (event.repeatCount == 0 && heldPhysicalKeys.putIfAbsent(event.deviceId to event.keyCode, vk) == null) {
                    acquireGuestKey(handle, vk)
                }
            }
            KeyEvent.ACTION_UP -> {
                heldPhysicalKeys.remove(event.deviceId to event.keyCode)?.let { releaseGuestKey(handle, it) }
            }
        }
        return true
    }

    private fun virtualKeyFor(keyCode: Int): Int? = when (keyCode) {
        KeyEvent.KEYCODE_DPAD_UP -> VK_UP
        KeyEvent.KEYCODE_DPAD_DOWN -> VK_DOWN
        KeyEvent.KEYCODE_DPAD_LEFT -> VK_LEFT
        KeyEvent.KEYCODE_DPAD_RIGHT -> VK_RIGHT
        KeyEvent.KEYCODE_ENTER -> VK_RETURN
        KeyEvent.KEYCODE_SPACE -> VK_SPACE
        KeyEvent.KEYCODE_TAB -> VK_TAB
        KeyEvent.KEYCODE_ESCAPE -> VK_ESCAPE
        KeyEvent.KEYCODE_SHIFT_LEFT, KeyEvent.KEYCODE_SHIFT_RIGHT -> VK_SHIFT
        KeyEvent.KEYCODE_CTRL_LEFT, KeyEvent.KEYCODE_CTRL_RIGHT -> VK_CTRL
        KeyEvent.KEYCODE_BUTTON_A -> VK_RETURN
        KeyEvent.KEYCODE_BUTTON_B -> VK_SPACE
        KeyEvent.KEYCODE_BUTTON_X -> VK_SHIFT
        KeyEvent.KEYCODE_BUTTON_Y -> VK_CTRL
        KeyEvent.KEYCODE_BUTTON_START -> VK_ESCAPE
        KeyEvent.KEYCODE_BUTTON_SELECT -> VK_TAB
        KeyEvent.KEYCODE_BUTTON_L1, KeyEvent.KEYCODE_BUTTON_L2 -> VK_TAB
        KeyEvent.KEYCODE_BUTTON_R1, KeyEvent.KEYCODE_BUTTON_R2 -> VK_ESCAPE
        KeyEvent.KEYCODE_F1 -> VK_F1
        KeyEvent.KEYCODE_F2 -> VK_F2
        KeyEvent.KEYCODE_F3 -> VK_F3
        KeyEvent.KEYCODE_F11 -> VK_F11
        else -> null
    }

    /**
     * A DualShock 4 (and most other pads) reports its left stick and its
     * hat switch as motion axes, not as `KEYCODE_DPAD_*` keys, so the
     * D-pad would do nothing without this. Both are folded into the same
     * guest arrow keys the on-screen pad sends, through the same
     * refcounted acquire/release pair, so a stick and the on-screen
     * button can be held at once without one release cancelling both.
     */
    override fun onGenericMotionEvent(event: MotionEvent): Boolean {
        val joystick = event.source and InputDevice.SOURCE_JOYSTICK == InputDevice.SOURCE_JOYSTICK ||
            event.source and InputDevice.SOURCE_GAMEPAD == InputDevice.SOURCE_GAMEPAD
        val handle = session
        if (!joystick || handle == 0L || pauseReasons.isNotEmpty() || event.action != MotionEvent.ACTION_MOVE) {
            return super.onGenericMotionEvent(event)
        }
        val x = strongestAxis(event, MotionEvent.AXIS_HAT_X, MotionEvent.AXIS_X)
        val y = strongestAxis(event, MotionEvent.AXIS_HAT_Y, MotionEvent.AXIS_Y)
        applyAxisKey(handle, event.deviceId, VK_LEFT, x <= -AXIS_DEADZONE)
        applyAxisKey(handle, event.deviceId, VK_RIGHT, x >= AXIS_DEADZONE)
        applyAxisKey(handle, event.deviceId, VK_UP, y <= -AXIS_DEADZONE)
        applyAxisKey(handle, event.deviceId, VK_DOWN, y >= AXIS_DEADZONE)
        return true
    }

    /** The hat wins when it is off centre, otherwise the analog stick. */
    private fun strongestAxis(event: MotionEvent, hatAxis: Int, stickAxis: Int): Float {
        val hat = event.getAxisValue(hatAxis)
        return if (kotlin.math.abs(hat) >= AXIS_DEADZONE) hat else event.getAxisValue(stickAxis)
    }

    private fun applyAxisKey(handle: Long, deviceId: Int, vk: Int, pressed: Boolean) {
        val name = when(vk) { VK_UP -> "PadUp"; VK_DOWN -> "PadDown"; VK_LEFT -> "PadLeft"; else -> "PadRight" }
        val list = controlConfig.optJSONArray("bindings")
        var target = vk
        if(list!=null) for(i in 0 until list.length()) { val b=list.getJSONObject(i);if(b.optString("input").equals(name,true)) target=b.optInt("vk") }
        if(target == 0) return
        if (pressed) {
            if (heldAxisKeys.putIfAbsent(deviceId to vk, target) == null) acquireGuestKey(handle, target)
        } else {
            heldAxisKeys.remove(deviceId to vk)?.let { releaseGuestKey(handle, it) }
        }
    }

    private fun acquireGuestKey(handle: Long, vk: Int) {
        val count = heldGuestKeys[vk] ?: 0
        heldGuestKeys[vk] = count + 1
        if (count == 0) {
            NativeBridge.nativeSendInput(handle, NativeBridge.INPUT_KEY_DOWN, vk, 0)
        }
    }

    private fun releaseGuestKey(handle: Long, vk: Int) {
        val count = heldGuestKeys[vk] ?: return
        if (count <= 1) {
            heldGuestKeys.remove(vk)
            NativeBridge.nativeSendInput(handle, NativeBridge.INPUT_KEY_UP, vk, 0)
        } else {
            heldGuestKeys[vk] = count - 1
        }
    }

    /** Release keys and pointer state before the native session can outlive the UI. */
    private fun releaseHeldInput() {
        inputEpoch++
        val handle = session
        if (handle != 0L) {
            for (vk in heldGuestKeys.keys.toList()) {
                NativeBridge.nativeSendInput(handle, NativeBridge.INPUT_KEY_UP, vk, 0)
            }
            if (surfacePointerDown) {
                NativeBridge.nativeSendInput(handle, NativeBridge.INPUT_POINTER_UP, lastPointerX, lastPointerY)
            }
        }
        heldPhysicalKeys.clear()
        heldGuestKeys.clear()
        heldVirtualKeys.clear()
        heldAxisKeys.clear()
        surfacePointerDown = false
    }

    /**
     * Forward any touches on the framebuffer surface as
     * `WM_LBUTTONDOWN` / `WM_LBUTTONUP` events with stylus
     * coordinates in 240×320 game space — the same mapping the
     * desktop GUI uses.
     */
    @SuppressLint("ClickableViewAccessibility")
    private fun wireSurfaceTouchInput() {
        surface.setOnTouchListener { v, event ->
            val handle = session
            if (handle == 0L || pauseReasons.isNotEmpty()) return@setOnTouchListener false
            val frame = lastFrame
            val mapped = frame?.let { mapTouchToGame(v, event, it) }
            when (event.actionMasked) {
                MotionEvent.ACTION_DOWN -> {
                    val (gx, gy) = mapped ?: return@setOnTouchListener true
                    v.requestFocus()
                    NativeBridge.nativeSendInput(handle, NativeBridge.INPUT_POINTER_DOWN, gx, gy)
                    activeStylusId = event.getPointerId(0)
                    surfacePointerDown = true
                    lastPointerX = gx
                    lastPointerY = gy
                }
                MotionEvent.ACTION_MOVE -> {
                    if (surfacePointerDown && event.getPointerId(0) == activeStylusId && mapped != null) {
                        val (gx, gy) = mapped
                        NativeBridge.nativeSendInput(handle, NativeBridge.INPUT_POINTER_MOVE, gx, gy)
                        lastPointerX = gx
                        lastPointerY = gy
                    }
                }
                MotionEvent.ACTION_UP, MotionEvent.ACTION_CANCEL, MotionEvent.ACTION_POINTER_UP -> {
                    if (surfacePointerDown && (event.actionMasked == MotionEvent.ACTION_CANCEL || event.getPointerId(event.actionIndex) == activeStylusId)) {
                        val (gx, gy) = mapped ?: (lastPointerX to lastPointerY)
                        NativeBridge.nativeSendInput(handle, NativeBridge.INPUT_POINTER_UP, gx, gy)
                        surfacePointerDown = false
                        activeStylusId = -1
                        lastPointerX = gx
                        lastPointerY = gy
                        if (event.actionMasked == MotionEvent.ACTION_UP) v.performClick()
                    }
                }
            }
            true
        }
    }

    /**
     * j2me-loader-inspired virtual gamepad: a D-pad on the left and
     * three action / soft-key buttons on the right. The button views
     * live in `activity_game.xml`. We listen for touch events
     * directly so the WM_KEYDOWN/WM_KEYUP pair is fired as the user
     * presses and releases the button — not just once per click.
     */
    private fun wireVirtualGamepad() {
        bindVk(R.id.btn_up, VK_UP)
        bindVk(R.id.btn_down, VK_DOWN)
        bindVk(R.id.btn_left, VK_LEFT)
        bindVk(R.id.btn_right, VK_RIGHT)
        bindVk(R.id.btn_action, VK_RETURN)
        bindVk(R.id.btn_turbo, VK_F3)
        bindVk(R.id.btn_a, VK_CTRL)
        bindVk(R.id.btn_b, VK_SPACE)
        bindVk(R.id.btn_c, VK_SHIFT)
        bindVk(R.id.btn_soft1, VK_TAB)
        bindVk(R.id.btn_soft2, VK_ESCAPE)
    }

    @SuppressLint("ClickableViewAccessibility")
    private fun bindVk(viewId: Int, vk: Int) {
        val btn = findViewById<View?>(viewId) ?: return
        bindControlView(btn,vk)
    }

    @SuppressLint("ClickableViewAccessibility")
    private fun bindControlView(btn: View, vk: Int, pulse: Boolean = false) {
        if(pulse) {
            // Let Android handle up-inside/cancel and accessibility activation.
            // This PPC2003 skin sends a short pair even after a long press.
            btn.setOnClickListener {
                val handle=session
                if(handle!=0L && pauseReasons.isEmpty()) {
                    acquireGuestKey(handle,vk)
                    releaseGuestKey(handle,vk)
                }
            }
            return
        }
        var pointerClick=false
        btn.setOnClickListener {
            if(!pointerClick && session!=0L && pauseReasons.isEmpty()) {
                val handle=session
                val epoch=inputEpoch
                acquireGuestKey(handle,vk)
                // A semantic click has no release edge. Give it its own hold;
                // it must not release a simultaneously held hardware/touch key.
                mainHandler.postDelayed({if(session==handle && inputEpoch==epoch)releaseGuestKey(handle,vk)},64)
            }
        }
        btn.setOnTouchListener { v, event ->
            val handle = session
            if (handle == 0L || pauseReasons.isNotEmpty()) return@setOnTouchListener false
            when (event.actionMasked) {
                MotionEvent.ACTION_DOWN -> {
                    if (heldVirtualKeys.add(vk)) acquireGuestKey(handle, vk)
                    v.isPressed = true
                }
                MotionEvent.ACTION_UP, MotionEvent.ACTION_CANCEL -> {
                    if (heldVirtualKeys.remove(vk)) releaseGuestKey(handle, vk)
                    v.isPressed = false
                    if (event.actionMasked == MotionEvent.ACTION_UP) {pointerClick=true;v.performClick();pointerClick=false}
                }
            }
            true
        }
    }

    /**
     * Map a screen-space touch on the SurfaceView into the
     * streamed game-space coordinates the kernel expects. Returns
     * `null` if the touch landed in the letter-box around the
     * scaled framebuffer.
     *
     * The presented picture may be a quarter turn away from the guest's
     * own framebuffer ([rotationDegrees]), in which case the turn has to
     * be undone here or a stylus tap lands somewhere the user did not
     * touch. This is the Android twin of the desktop launcher's
     * `rotated_pointer_to_game`.
     */
    private fun mapTouchToGame(
        v: View,
        event: MotionEvent,
        frame: FrameSnapshot,
    ): Pair<Int, Int>? {
        val viewW = v.width.toFloat()
        val viewH = v.height.toFloat()
        if (viewW <= 0 || viewH <= 0) return null
        val quarter = rotationDegrees == 90 || rotationDegrees == 270
        val shownW = if (quarter) frame.height else frame.width
        val shownH = if (quarter) frame.width else frame.height
        val scale = minOf(viewW / shownW, viewH / shownH)
        val drawnW = shownW * scale
        val drawnH = shownH * scale
        val dx = event.x - (viewW - drawnW) / 2f
        val dy = event.y - (viewH - drawnH) / 2f
        if (dx < 0f || dy < 0f || dx >= drawnW || dy >= drawnH) return null
        val px = (dx / scale).toInt().coerceIn(0, shownW - 1)
        val py = (dy / scale).toInt().coerceIn(0, shownH - 1)
        val (gx, gy) = when (rotationDegrees) {
            90 -> py to (shownW - 1 - px)
            180 -> (shownW - 1 - px) to (shownH - 1 - py)
            270 -> (shownH - 1 - py) to px
            else -> px to py
        }
        val zoom=controlConfig.optDouble("zoom",1.0).coerceIn(1.0,4.0);val half=.5/zoom
        val cx=controlConfig.optDouble("zoom_x",.5).coerceIn(half,1-half);val cy=controlConfig.optDouble("zoom_y",.5).coerceIn(half,1-half)
        return (((gx.toDouble()/frame.width-.5)/zoom+cx)*frame.width).toInt().coerceIn(0,frame.width-1) to (((gy.toDouble()/frame.height-.5)/zoom+cy)*frame.height).toInt().coerceIn(0,frame.height-1)
    }

    private data class FrameSnapshot(
        val width: Int,
        val height: Int,
        val rgba: ByteArray,
        val rgbaOffset: Int,
    )

    private class FrameRenderer : GLSurfaceView.Renderer {
        @Volatile private var pending: FrameSnapshot? = null
        private var texture = 0
        private var program = 0
        private var vertexBuffer: java.nio.FloatBuffer? = null
        private var positionHandle = 0
        private var texCoordHandle = 0
        private var textureHandle = 0
        private var samplerHandle = 0
        private var textureWidth = 0
        private var textureHeight = 0
        private var viewportWidth = 1
        private var viewportHeight = 1
        /** Clockwise quarter turn applied when the quad is drawn. */
        @Volatile private var rotationDegrees = 0
        @Volatile private var view = floatArrayOf(1f,.5f,.5f)
        fun setView(zoom:Float,x:Float,y:Float) { val z=zoom.coerceIn(1f,4f);val half=.5f/z;view=floatArrayOf(z,x.coerceIn(half,1-half),y.coerceIn(half,1-half)) }

        fun submit(frame: FrameSnapshot) {
            pending = frame
        }

        /**
         * Set the presentation turn (0/90/180/270, clockwise).
         *
         * The rotation is applied to the texture coordinates rather than
         * to the guest framebuffer, so the game keeps rendering into the
         * 240x320 panel it was written for (JumpyBall only draws
         * correctly at that size) while the player sees the landscape
         * picture the game was designed around.
         */
        fun setRotationDegrees(degrees: Int) {
            rotationDegrees = ((degrees % 360) + 360) % 360
        }

        override fun onSurfaceCreated(gl: javax.microedition.khronos.opengles.GL10?, config: javax.microedition.khronos.egl.EGLConfig?) {
            GLES20.glClearColor(0f, 0f, 0f, 1f)
            program = buildProgram(VERTEX_SHADER, FRAGMENT_SHADER)
            val textures = IntArray(1)
            GLES20.glGenTextures(1, textures, 0)
            texture = textures[0]
            GLES20.glBindTexture(GLES20.GL_TEXTURE_2D, texture)
            GLES20.glTexParameteri(GLES20.GL_TEXTURE_2D, GLES20.GL_TEXTURE_MIN_FILTER, GLES20.GL_NEAREST)
            GLES20.glTexParameteri(GLES20.GL_TEXTURE_2D, GLES20.GL_TEXTURE_MAG_FILTER, GLES20.GL_NEAREST)
            GLES20.glTexParameteri(GLES20.GL_TEXTURE_2D, GLES20.GL_TEXTURE_WRAP_S, GLES20.GL_CLAMP_TO_EDGE)
            GLES20.glTexParameteri(GLES20.GL_TEXTURE_2D, GLES20.GL_TEXTURE_WRAP_T, GLES20.GL_CLAMP_TO_EDGE)
            vertexBuffer = java.nio.ByteBuffer.allocateDirect(VERTICES.size * 4)
                .order(java.nio.ByteOrder.nativeOrder()).asFloatBuffer().apply { put(VERTICES); position(0) }
            positionHandle = GLES20.glGetAttribLocation(program, "aPosition")
            texCoordHandle = GLES20.glGetAttribLocation(program, "aTexCoord")
            textureHandle = GLES20.glGetUniformLocation(program, "uTexture")
            samplerHandle = textureHandle
        }

        override fun onSurfaceChanged(gl: javax.microedition.khronos.opengles.GL10?, width: Int, height: Int) {
            GLES20.glViewport(0, 0, width, height)
            viewportWidth = width
            viewportHeight = height
        }

        override fun onDrawFrame(gl: javax.microedition.khronos.opengles.GL10?) {
            GLES20.glClear(GLES20.GL_COLOR_BUFFER_BIT)
            val frame = pending ?: return
            GLES20.glUseProgram(program)
            GLES20.glActiveTexture(GLES20.GL_TEXTURE0)
            GLES20.glBindTexture(GLES20.GL_TEXTURE_2D, texture)
            val pixels = java.nio.ByteBuffer.wrap(frame.rgba, frame.rgbaOffset, frame.width * frame.height * 4).slice()
            if (frame.width != textureWidth || frame.height != textureHeight) {
                GLES20.glTexImage2D(GLES20.GL_TEXTURE_2D, 0, GLES20.GL_RGBA, frame.width, frame.height, 0, GLES20.GL_RGBA, GLES20.GL_UNSIGNED_BYTE, pixels)
                textureWidth = frame.width
                textureHeight = frame.height
            } else {
                GLES20.glTexSubImage2D(GLES20.GL_TEXTURE_2D, 0, 0, 0, frame.width, frame.height, GLES20.GL_RGBA, GLES20.GL_UNSIGNED_BYTE, pixels)
            }
            // A quarter turn swaps the presented aspect ratio, so the
            // letterbox has to be computed from the *shown* size, not
            // from the framebuffer's own width and height.
            val turn = rotationDegrees
            val quarter = turn == 90 || turn == 270
            val shownWidth = if (quarter) frame.height else frame.width
            val shownHeight = if (quarter) frame.width else frame.height
            val scale = minOf(viewportWidth.toFloat() / shownWidth, viewportHeight.toFloat() / shownHeight)
            val drawnWidth = shownWidth * scale
            val drawnHeight = shownHeight * scale
            val left = (viewportWidth - drawnWidth) / viewportWidth - 1f
            val right = (viewportWidth + drawnWidth) / viewportWidth - 1f
            val bottom = 1f - (viewportHeight + drawnHeight) / viewportHeight
            val top = 1f - (viewportHeight - drawnHeight) / viewportHeight
            // Texture coordinates per screen corner. Rotating the image
            // clockwise means each displayed corner samples the source
            // corner one step counter-clockwise from it: at 90° the
            // top-left of the picture is the bottom-left of the guest
            // framebuffer.
            // Order: bottom-left, bottom-right, top-left, top-right —
            // the order the quad's vertices are written below.
            val uv = when (turn) {
                90 -> floatArrayOf(1f, 1f, 1f, 0f, 0f, 1f, 0f, 0f)
                180 -> floatArrayOf(1f, 0f, 0f, 0f, 1f, 1f, 0f, 1f)
                270 -> floatArrayOf(0f, 0f, 0f, 1f, 1f, 0f, 1f, 1f)
                else -> floatArrayOf(0f, 1f, 1f, 1f, 0f, 0f, 1f, 0f)
            }
            val crop=view
            for(i in uv.indices step 2) { uv[i]=crop[1]+(uv[i]-.5f)/crop[0];uv[i+1]=crop[2]+(uv[i+1]-.5f)/crop[0] }
            vertexBuffer?.let { buffer ->
                buffer.clear()
                buffer.put(floatArrayOf(
                    left, bottom, uv[0], uv[1],
                    right, bottom, uv[2], uv[3],
                    left, top, uv[4], uv[5],
                    right, top, uv[6], uv[7],
                ))
                buffer.position(0)
                GLES20.glEnableVertexAttribArray(positionHandle)
                GLES20.glVertexAttribPointer(positionHandle, 2, GLES20.GL_FLOAT, false, 16, buffer)
                buffer.position(2)
                GLES20.glEnableVertexAttribArray(texCoordHandle)
                GLES20.glVertexAttribPointer(texCoordHandle, 2, GLES20.GL_FLOAT, false, 16, buffer)
                GLES20.glUniform1i(samplerHandle, 0)
                GLES20.glDrawArrays(GLES20.GL_TRIANGLE_STRIP, 0, 4)
            }
        }

        private fun buildProgram(vertex: String, fragment: String): Int {
            val vs = compileShader(GLES20.GL_VERTEX_SHADER, vertex)
            val fs = compileShader(GLES20.GL_FRAGMENT_SHADER, fragment)
            return GLES20.glCreateProgram().also { p ->
                GLES20.glAttachShader(p, vs)
                GLES20.glAttachShader(p, fs)
                GLES20.glLinkProgram(p)
                GLES20.glDeleteShader(vs)
                GLES20.glDeleteShader(fs)
            }
        }

        private fun compileShader(type: Int, source: String): Int {
            return GLES20.glCreateShader(type).also { shader ->
                GLES20.glShaderSource(shader, source)
                GLES20.glCompileShader(shader)
            }
        }

        companion object {
            private val VERTICES = floatArrayOf(
                -1f, -1f, 0f, 1f,
                 1f, -1f, 1f, 1f,
                -1f,  1f, 0f, 0f,
                 1f,  1f, 1f, 0f,
            )
            private const val VERTEX_SHADER = "attribute vec2 aPosition; attribute vec2 aTexCoord; varying vec2 vTexCoord; void main() { gl_Position = vec4(aPosition, 0.0, 1.0); vTexCoord = aTexCoord; }"
            private const val FRAGMENT_SHADER = "precision mediump float; varying vec2 vTexCoord; uniform sampler2D uTexture; void main() { gl_FragColor = texture2D(uTexture, vTexCoord); }"
        }
    }

    companion object {
        const val EXTRA_GAME_ID = "com.pockethle.app.EXTRA_GAME_ID"
        const val EXTRA_GAME_NAME = "com.pockethle.app.EXTRA_GAME_NAME"

        // Win32 virtual-key codes — same set the desktop GUI uses.
        private const val VK_UP = 0x26
        private const val VK_DOWN = 0x28
        private const val VK_LEFT = 0x25
        private const val VK_RIGHT = 0x27
        private const val VK_RETURN = 0x0D
        private const val VK_SPACE = 0x20
        private const val VK_TAB = 0x09
        private const val VK_SHIFT = 0x10
        private const val VK_CTRL = 0x11
        private const val VK_ESCAPE = 0x1B
        private const val VK_F1 = 0x70
        private const val VK_F2 = 0x71
        private const val VK_F3 = 0x72
        private const val VK_F11 = 0x7A

        /** Polling cadence in ms. 33 ≈ 30 Hz. */
        private const val POLL_INTERVAL_MS = 16L

        /** How far a stick has to travel before it counts as a press.
         * A DualShock 4 rests a few percent off centre. */
        private const val AXIS_DEADZONE = 0.5f

    }
}
