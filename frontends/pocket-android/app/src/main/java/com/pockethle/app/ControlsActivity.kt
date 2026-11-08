package com.pockethle.app

import android.os.Bundle
import android.view.KeyEvent
import android.view.MotionEvent
import android.view.View
import android.widget.*
import androidx.appcompat.app.AppCompatActivity
import androidx.appcompat.app.AlertDialog
import androidx.appcompat.widget.Toolbar
import org.json.JSONArray
import org.json.JSONObject

/** Edits the same versioned controls.json that Rust and Web consume. */
class ControlsActivity : AppCompatActivity() {
    private lateinit var body: LinearLayout
    private lateinit var config: JSONObject
    private lateinit var rootDir: String
    private var gameId = ""
    private var capture: EditText? = null
    private var restoredDraft: String? = null
    private lateinit var menuShortcut: EditText
    private var configReady=false
    private fun text(zh: String, en: String) = if (resources.configuration.locales[0].language == "zh") zh else en
    private fun dp(n: Int) = (n * resources.displayMetrics.density).toInt()
    private fun message(s: String) { Toast.makeText(this, s, Toast.LENGTH_LONG).show() }

    override fun onCreate(savedInstanceState: Bundle?) {
        Themes.apply(this)
        super.onCreate(savedInstanceState)
        restoredDraft=savedInstanceState?.getString("controls_draft")
        rootDir = LibraryPaths.root(this)
        gameId = intent.getStringExtra("game_id").orEmpty()
        val root = LinearLayout(this).apply { orientation = LinearLayout.VERTICAL }
        val toolbar = Toolbar(this).apply { title = text("操作方式", "Controls") }
        root.addView(toolbar)
        val scroll = ScrollView(this)
        body = LinearLayout(this).apply { orientation = LinearLayout.VERTICAL; setPadding(dp(16), dp(8), dp(16), dp(24)) }
        scroll.addView(body); root.addView(scroll, LinearLayout.LayoutParams(-1, 0, 1f))
        setContentView(root); setSupportActionBar(toolbar); supportActionBar?.setDisplayHomeAsUpEnabled(true)
        load()
    }
    override fun onCreateOptionsMenu(menu: android.view.Menu):Boolean { menu.add(0,1001,0,text("保存操作","Save controls")).setShowAsAction(android.view.MenuItem.SHOW_AS_ACTION_ALWAYS);return true }
    override fun onOptionsItemSelected(item: android.view.MenuItem):Boolean { if(item.itemId==1001){saveConfiguration();return true};return super.onOptionsItemSelected(item) }
    private fun saveConfiguration() {
        if(!configReady || !::config.isInitialized)return
        if(::menuShortcut.isInitialized)config.put("menu_key",menuShortcut.text.toString().trim())
        val result=JSONObject(NativeBridge.writeControls(rootDir,gameId,config.toString()))
        if(result.optBoolean("ok")){message(text("已保存","Saved"));finish()}else message(result.optString("error"))
    }
    override fun onSaveInstanceState(state: Bundle) { if(::config.isInitialized){if(::menuShortcut.isInitialized)config.put("menu_key",menuShortcut.text.toString());state.putString("controls_draft",config.toString())};super.onSaveInstanceState(state) }
    override fun onSupportNavigateUp(): Boolean { finish(); return true }
    private fun load() {
        configReady=false
        val reply = JSONObject(NativeBridge.readControls(rootDir, gameId))
        body.removeAllViews()
        if (!reply.optBoolean("ok")) { label(reply.optString("error")); button(text("重試", "Retry")) { load() };button(text("明確恢復繼承／預設","Restore inherited / default controls")){val result=JSONObject(NativeBridge.writeControls(rootDir,gameId,"null"));if(result.optBoolean("ok"))load()else message(result.optString("error"))};return }
        config = restoredDraft?.let { JSONObject(it) } ?: reply.getJSONObject("value")
        restoredDraft=null
        configReady=true
        intent.getStringExtra("preview_profile")?.let { profile ->
            config.put("profile",profile);config.put("keys",JSONArray(ControlNames.preset(profile)));config.put("keyboard",profile=="text");intent.removeExtra("preview_profile")
        }
        label(if (gameId.isEmpty()) text("全域操作預設", "Global control defaults") else intent.getStringExtra("game_name") ?: gameId)
        label(if (reply.optBoolean("overridden")) text("此軟體自訂", "Program override") else text("繼承全域設定", "Inherited defaults"))
        if(gameId.isNotEmpty()) {
            val file=java.io.File(rootDir,"games/$gameId/presentation.json")
            val presentation=runCatching { JSONObject(file.readText()) }.getOrElse { JSONObject() }
            val alias=EditText(this).apply { hint=text("顯示別名（正式名稱保留）","Display alias (original name preserved)");setText(presentation.optString("alias"));maxLines=1 };body.addView(alias)
            val favorite=CheckBox(this).apply { text=this@ControlsActivity.text("收藏","Favorite");isChecked=presentation.optBoolean("favorite") };body.addView(favorite)
            button(text("保存名稱與收藏","Save alias and favorite")) {
                runCatching { presentation.put("alias",alias.text.toString().take(256));presentation.put("favorite",favorite.isChecked)
                    val staged=java.io.File(file.parentFile,"presentation.json.new");staged.writeText(presentation.toString());check(staged.renameTo(file)) { "Cannot save name" }
                }.onSuccess { message(text("已保存","Saved")) }.onFailure { message(it.message.orEmpty()) }
            }
        }
        val names = arrayOf(text("觸控筆優先", "Stylus first"), text("方向＋動作鍵", "Direction + action"), text("完整 Pocket PC", "Pocket PC buttons"), text("文字／閱讀", "Text / reading"), text("滑鼠＋鍵盤", "Mouse + keyboard"), text("自訂", "Custom"), text("CE 硬體按鈕", "CE hardware buttons"), text("Pocket PC 2003 面板", "Pocket PC 2003 panel"))
        val profiles = arrayOf("stylus", "direction", "full", "text", "desktop", "custom", "device", "ppc2003")
        val picker = Spinner(this).apply { adapter = ArrayAdapter(this@ControlsActivity, android.R.layout.simple_spinner_dropdown_item, names); setSelection(profiles.indexOf(config.optString("profile")).coerceAtLeast(0)) }
        body.addView(picker)
        val capability=TextView(this).apply { setPadding(0,dp(8),0,dp(8));textSize=15f };body.addView(capability)
        val keysArea = LinearLayout(this).apply { orientation = LinearLayout.VERTICAL }
        button(text("可見按鍵…","Visible buttons…")){keysArea.visibility=if(keysArea.visibility==View.VISIBLE)View.GONE else View.VISIBLE}
        body.addView(keysArea)
        fun showKeys() {
            val profile=config.optString("profile")
            keysArea.visibility=if(profile=="custom")View.VISIBLE else View.GONE
            capability.text=when(profile){
                "ppc2003"->text("PPC2003：快捷鍵與 Enter 點擊一次；方向與錄音可長按。不含 Power／Reset。", "PPC2003: shortcuts and Enter click once; directions and Record hold. No Power / Reset.")
                "device"->text("通用 CE Action／App1–6；指定機型／skin 尚待驗證，Power／Reset 未提供。","Generic CE Action / App1–6. Device-specific skins are unverified; Power / Reset unavailable.")
                "desktop"->text("滑鼠點擊模擬觸控筆；相對滑鼠尚未提供。","Mouse clicks act as stylus taps; relative mouse is unavailable.")
                else->text("直接點擊程式畫面；模擬器選單會暫停程式。","Tap the program screen directly. The emulator menu pauses it.")
            }
            keysArea.removeAllViews()
            for (vk in ControlNames.keys) {
                val box = CheckBox(this).apply { text = ControlNames.label(vk); isChecked = selectedKeys().contains(vk) }
                box.setOnCheckedChangeListener { _, checked ->
                    val keys = selectedKeys().toMutableSet(); if (checked) keys.add(vk) else keys.remove(vk)
                    config.put("keys", JSONArray(keys.sorted())); config.put("profile", "custom"); picker.setSelection(5)
                }
                keysArea.addView(box)
            }
        }
        picker.onItemSelectedListener = object : AdapterView.OnItemSelectedListener {
            override fun onNothingSelected(parent: AdapterView<*>?) {}
            override fun onItemSelected(parent: AdapterView<*>?, view: View?, position: Int, id: Long) {
                val profile = profiles[position]
                if (profile == config.optString("profile")) return
                config.put("profile", profile)
                if (profile != "custom") { config.put("keys", JSONArray(ControlNames.preset(profile))); config.put("keyboard", profile == "text") }
                showKeys()
                for(i in 0 until body.childCount) body.getChildAt(i).invalidate()
            }
        }
        showKeys()
        menuShortcut=EditText(this).apply { hint=text("選單快捷鍵","Menu shortcut");setText(config.optString("menu_key","F1")) };body.addView(menuShortcut)
        toggle(text("顯示文字鍵盤", "Show text keyboard"), "keyboard")
        toggle(text("連接手把時隱藏控制盤", "Hide controls with gamepad"), "auto_hide_gamepad")
        slider(text("按鍵透明度", "Button opacity"), "opacity", .1f, 1f)
        slider(text("音量", "Volume"), "volume", 0f, 1f)
        slider(text("畫面放大", "Screen zoom"), "zoom", 1f, 4f)
        slider(text("客體焦點 X", "Guest focus X"), "zoom_x", 0f, 1f)
        slider(text("客體焦點 Y", "Guest focus Y"), "zoom_y", 0f, 1f)
        label(text("放大會裁切顯示區；使用拖曳／指標定位，恢復 1× 可看完整畫面。", "Zoom crops the visible screen; return to 1× for the whole frame."))

        val bindingsArea = LinearLayout(this).apply { orientation = LinearLayout.VERTICAL }; body.addView(bindingsArea)
        fun showBindings() {
            bindingsArea.removeAllViews()
            val bindings = config.optJSONArray("bindings") ?: JSONArray()
            for (i in 0 until bindings.length()) {
                val b = bindings.getJSONObject(i)
                val remove = Button(this).apply { text = "${b.optString("input")} → ${ControlNames.label(b.optInt("vk"))}  ×" }
                remove.setOnClickListener { val next = JSONArray(); for (j in 0 until bindings.length()) if (i != j) next.put(bindings.get(j)); config.put("bindings", next); showBindings() }
                bindingsArea.addView(remove)
            }
        }
        label(text("鍵盤／手把映射", "Keyboard / gamepad mapping")); showBindings()
        button(text("新增按鍵映射", "Add key binding")) {
            val content = LinearLayout(this).apply { orientation = LinearLayout.VERTICAL; setPadding(dp(16), 0, dp(16), 0) }
            val input = EditText(this).apply { hint = "Up, Enter, A, PadA…" }
            val target = Spinner(this).apply { adapter = ArrayAdapter(this@ControlsActivity, android.R.layout.simple_spinner_dropdown_item, ControlNames.keys.map { ControlNames.label(it) }) }
            val listen = Button(this).apply { text = this@ControlsActivity.text("按下一個實體按鍵", "Capture a hardware key") }
            listen.setOnClickListener { capture = input; input.requestFocus(); message(text("請按鍵盤或手把按鈕", "Press a keyboard or controller button")) }
            content.addView(input); content.addView(listen); content.addView(target)
            AlertDialog.Builder(this).setTitle(text("按鍵映射", "Key binding")).setView(content)
                .setPositiveButton(text("加入", "Add")) { _, _ ->
                    capture = null; val name = input.text.toString().trim()
                    if (name.isNotEmpty()) { val list = config.optJSONArray("bindings") ?: JSONArray(); list.put(JSONObject().put("input", name).put("vk", ControlNames.keys[target.selectedItemPosition])); config.put("bindings", list); showBindings() }
                }.setNegativeButton(android.R.string.cancel) { _, _ -> capture = null }.setOnDismissListener { capture = null }.show()
        }
        label(text("直向按鈕布局（拖曳可移動）", "Portrait button layout (drag to move)"))
        body.addView(LayoutEditor(this, config, "portrait"), LinearLayout.LayoutParams(-1, dp(280)))
        label(text("橫向按鈕布局（拖曳可移動）", "Landscape button layout (drag to move)"))
        body.addView(LayoutEditor(this, config, "landscape"), LinearLayout.LayoutParams(-1, dp(180)))
        label(text("所有可見按鍵尺寸", "Size of all visible buttons"))
        val sizeBar=SeekBar(this).apply { max=37;progress=9 }
        sizeBar.setOnSeekBarChangeListener(object:SeekBar.OnSeekBarChangeListener {
            override fun onStartTrackingTouch(b:SeekBar?) {}
            override fun onStopTrackingTouch(b:SeekBar?) {}
            override fun onProgressChanged(b:SeekBar?,p:Int,user:Boolean) { if(!user)return
                for(key in listOf("portrait","landscape")) {
                    val list=config.optJSONArray(key) ?: JSONArray()
                    for(i in 0 until list.length()) list.getJSONObject(i).put("size",.03+p/100.0)
                    config.put(key,list)
                }
                for(i in 0 until body.childCount) body.getChildAt(i).invalidate()
            }
        });body.addView(sizeBar)
        button(text("重設按鈕位置", "Reset button positions")) { config.put("portrait",JSONArray());config.put("landscape",JSONArray());for(i in 0 until body.childCount)body.getChildAt(i).invalidate() }
        label(text("設定只影響操作與顯示，不會保存目前客體狀態；進度仍須從軟體內正常保存。", "Controls do not save the running guest. Save progress inside the program."))
        button(text("保存設定", "Save controls")) { saveConfiguration() }
        button(text("恢復繼承／預設", "Restore inherited / default controls")) {
            val result = JSONObject(NativeBridge.writeControls(rootDir, gameId, "null"))
            if (result.optBoolean("ok")) load() else message(result.optString("error"))
        }
    }
    private fun selectedKeys(): Set<Int> = config.optJSONArray("keys")?.let { a -> (0 until a.length()).map { a.optInt(it) }.toSet() } ?: emptySet()
    private fun label(value: String) { body.addView(TextView(this).apply { text = value; setPadding(0, dp(12), 0, dp(8)); textSize = 15f }) }
    private fun button(value: String, action: () -> Unit) { body.addView(Button(this).apply { text = value; setOnClickListener { action() } }) }
    private fun toggle(value: String, key: String) { body.addView(CheckBox(this).apply { text = value; isChecked = config.optBoolean(key); setOnCheckedChangeListener { _, checked -> config.put(key, checked) } }) }
    private fun slider(name: String, key: String, min: Float, max: Float) {
        val caption = TextView(this); body.addView(caption)
        fun update(v: Float) { caption.text = "$name: ${"%.2f".format(v)}" }
        val bar = SeekBar(this).apply { this.max = 100; progress = ((config.optDouble(key, min.toDouble()).toFloat() - min) / (max - min) * 100).toInt() }
        update(min + bar.progress / 100f * (max - min))
        bar.setOnSeekBarChangeListener(object : SeekBar.OnSeekBarChangeListener {
            override fun onStartTrackingTouch(b: SeekBar?) {}
            override fun onStopTrackingTouch(b: SeekBar?) {}
            override fun onProgressChanged(b: SeekBar?, p: Int, user: Boolean) { val v = min + p / 100f * (max - min); config.put(key, v.toDouble()); update(v) }
        }); body.addView(bar)
    }
    override fun dispatchKeyEvent(event: KeyEvent): Boolean {
        capture?.let { if (event.action == KeyEvent.ACTION_DOWN && event.keyCode != KeyEvent.KEYCODE_BACK) { it.setText(ControlNames.input(event.keyCode,event.source and android.view.InputDevice.SOURCE_GAMEPAD == android.view.InputDevice.SOURCE_GAMEPAD)); capture = null; return true } }
        return super.dispatchKeyEvent(event)
    }
}

internal object ControlNames {
    fun canonical(value:String):String { val name=value.lowercase();return when(name) {"esc","escape"->"escape";"ctrl","control","ctrlleft","ctrlright"->"ctrl";"shiftleft","shiftright"->"shift";"arrowup"->"up";"arrowdown"->"down";"arrowleft"->"left";"arrowright"->"right";else->name} }
    val keys = listOf(38, 40, 37, 39, 13, 9, 27, 17, 32, 16, 33, 34, 0x86, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 112, 113, 119, 120, 121)
    fun label(vk: Int): String = when (vk) { 38 -> "↑"; 40 -> "↓"; 37 -> "←"; 39 -> "→"; 13 -> "Enter"; 9 -> "Tab / Soft 1"; 27 -> "Esc / Soft 2"; 17 -> "A / Ctrl"; 32 -> "B / Space"; 16 -> "C / Shift"; 33 -> "Page Up"; 34 -> "Page Down"; 112, 113, 119, 120, 121 -> Ppc2003Panel.button(vk)?.optString("label") ?: "F${vk-111}"; 0x86 -> "Action"; in 0xc1..0xc6 -> "App ${vk - 0xc0}"; else -> "VK $vk" }
    fun preset(profile: String) = when(profile) { "stylus", "desktop" -> emptyList(); "direction" -> keys.take(5); "text" -> listOf(9,27,33,34); "ppc2003" -> Ppc2003Panel.keys(); "device" -> listOf(38,40,37,39,0x86,0xc1,0xc2,0xc3,0xc4,0xc5,0xc6); else -> keys.take(10) }
    fun input(code: Int, gamepad: Boolean = false): String = if(gamepad && code in listOf(KeyEvent.KEYCODE_DPAD_UP,KeyEvent.KEYCODE_DPAD_DOWN,KeyEvent.KEYCODE_DPAD_LEFT,KeyEvent.KEYCODE_DPAD_RIGHT)) "Pad"+input(code) else when(code) { KeyEvent.KEYCODE_DPAD_UP -> "Up"; KeyEvent.KEYCODE_DPAD_DOWN -> "Down"; KeyEvent.KEYCODE_DPAD_LEFT -> "Left"; KeyEvent.KEYCODE_DPAD_RIGHT -> "Right"; KeyEvent.KEYCODE_CTRL_LEFT, KeyEvent.KEYCODE_CTRL_RIGHT -> "Ctrl"; KeyEvent.KEYCODE_SHIFT_LEFT, KeyEvent.KEYCODE_SHIFT_RIGHT -> "Shift"; KeyEvent.KEYCODE_ENTER -> "Enter"; KeyEvent.KEYCODE_ESCAPE -> "Escape"; KeyEvent.KEYCODE_TAB -> "Tab"; KeyEvent.KEYCODE_SPACE -> "Space"; KeyEvent.KEYCODE_BUTTON_A -> "PadA"; KeyEvent.KEYCODE_BUTTON_B -> "PadB"; KeyEvent.KEYCODE_BUTTON_X -> "PadX"; KeyEvent.KEYCODE_BUTTON_Y -> "PadY"; KeyEvent.KEYCODE_BUTTON_L1 -> "PadL1"; KeyEvent.KEYCODE_BUTTON_R1 -> "PadR1"; KeyEvent.KEYCODE_BUTTON_L2 -> "PadL2"; KeyEvent.KEYCODE_BUTTON_R2 -> "PadR2"; KeyEvent.KEYCODE_BUTTON_THUMBL -> "PadL3"; KeyEvent.KEYCODE_BUTTON_THUMBR -> "PadR3"; KeyEvent.KEYCODE_BUTTON_START -> "PadStart"; KeyEvent.KEYCODE_BUTTON_SELECT -> "PadSelect"; else -> KeyEvent.keyCodeToString(code).removePrefix("KEYCODE_") }
}

private class LayoutEditor(context: android.content.Context, private val config: JSONObject, private val key: String) : View(context) {
    private val paint = android.graphics.Paint(android.graphics.Paint.ANTI_ALIAS_FLAG)
    private var selected = -1
    private fun positions(): JSONArray {
        val list = config.optJSONArray(key) ?: JSONArray()
        val keys = config.optJSONArray("keys") ?: JSONArray()
        for (i in 0 until keys.length()) if((0 until list.length()).none { list.getJSONObject(it).optInt("vk") == keys.getInt(i) }) list.put(JSONObject().put("vk", keys.getInt(i)).put("x", .17 + (i % 3) * .32).put("y", .12 + (i / 3) * .13).put("size", .12))
        config.put(key, list); return list
    }
    override fun onDraw(canvas: android.graphics.Canvas) {
        canvas.drawColor(android.graphics.Color.rgb(28, 31, 36)); val list = positions()
        for (i in 0 until list.length()) { val b = list.getJSONObject(i); val x = b.optDouble("x").toFloat()*width; val y = b.optDouble("y").toFloat()*height
            paint.color = android.graphics.Color.rgb(70,110,160); canvas.drawCircle(x,y,(b.optDouble("size",.12)*width.coerceAtMost(height)).toFloat()/2,paint)
            paint.color = android.graphics.Color.WHITE; paint.textSize = 24f; paint.textAlign = android.graphics.Paint.Align.CENTER; canvas.drawText(ControlNames.label(b.optInt("vk")),x,y+8,paint) }
    }
    override fun onTouchEvent(event: MotionEvent): Boolean {
        val list = positions()
        when(event.actionMasked) {
            MotionEvent.ACTION_DOWN -> { parent.requestDisallowInterceptTouchEvent(true); selected = (0 until list.length()).minByOrNull { val b=list.getJSONObject(it); val dx=event.x-b.optDouble("x")*width; val dy=event.y-b.optDouble("y")*height; dx*dx+dy*dy } ?: -1 }
            MotionEvent.ACTION_MOVE -> if (selected >= 0) { list.getJSONObject(selected).put("x",(event.x/width).coerceIn(.05f,.95f).toDouble()).put("y",(event.y/height).coerceIn(.05f,.95f).toDouble()); invalidate() }
            MotionEvent.ACTION_UP, MotionEvent.ACTION_CANCEL -> { selected = -1; parent.requestDisallowInterceptTouchEvent(false); performClick() }
        }; return true
    }
    override fun performClick(): Boolean { super.performClick(); return true }
}
