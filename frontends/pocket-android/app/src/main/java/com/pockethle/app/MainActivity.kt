package com.pockethle.app

import android.content.Intent
import android.net.Uri
import android.os.Bundle
import android.provider.OpenableColumns
import android.view.Menu
import android.view.MenuItem
import android.view.View
import android.widget.TextView
import androidx.activity.result.contract.ActivityResultContracts
import androidx.appcompat.app.AlertDialog
import androidx.appcompat.app.AppCompatActivity
import androidx.appcompat.widget.Toolbar
import androidx.lifecycle.lifecycleScope
import androidx.recyclerview.widget.GridLayoutManager
import androidx.recyclerview.widget.RecyclerView
import com.google.android.material.floatingactionbutton.FloatingActionButton
import com.google.android.material.snackbar.Snackbar
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import org.json.JSONArray
import org.json.JSONObject

/**
 * Library screen — heavily inspired by
 * [j2me-loader](https://github.com/nikita36078/j2me-loader): a
 * RecyclerView of game cards, a "+" floating action button to import a
 * new `.CAB`, and per-card overflow menu (Run / Settings / Remove).
 */
class MainActivity : AppCompatActivity() {

    private lateinit var adapter: GameAdapter
    private lateinit var recycler: RecyclerView
    private lateinit var emptyState: TextView
    private lateinit var rootDir: String
    private var launching = false
    private var libraryItems: List<GameEntry> = emptyList()
    private var query = ""
    private var recentFirst = true
    private var refreshGeneration = 0
    private fun showFiltered() {
        val filtered = libraryItems.filter { it.displayName.contains(query, true) || it.alias.contains(query,true) }
        adapter.submit(if(recentFirst) filtered.sortedWith(compareByDescending<GameEntry> { it.favorite }.thenByDescending { it.lastPlayed }) else filtered.sortedBy { it.alias.ifBlank { it.displayName }.lowercase() })
        recycler.visibility = if(filtered.isEmpty()) View.GONE else View.VISIBLE
        emptyState.visibility = if(filtered.isEmpty()) View.VISIBLE else View.GONE
        emptyState.text = if(libraryItems.isEmpty()) getString(R.string.empty_library) else "找不到符合的軟體 / No matching programs"
    }

    private val importGame = registerForActivityResult(
        ActivityResultContracts.OpenDocument(),
    ) { uri ->
        if (uri != null) handleImport(uri)
    }

    /** The game a save export or import picker was opened for. */
    private var saveGame: GameEntry? = null

    private val exportSave = registerForActivityResult(
        ActivityResultContracts.CreateDocument("application/zip"),
    ) { uri -> saveGame?.let { game -> if (uri != null) runSave(game, "export", uri) } }

    private val importSave = registerForActivityResult(
        ActivityResultContracts.OpenDocument(),
    ) { uri -> saveGame?.let { game -> if (uri != null) runSave(game, "import", uri) } }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        setSupportActionBar(findViewById<Toolbar>(R.id.toolbar))

        query=savedInstanceState?.getString("library_query").orEmpty()
        recentFirst=savedInstanceState?.getBoolean("recent_first",true) ?: true
        rootDir = LibraryPaths.root(this)
        emptyState = findViewById(R.id.empty_state)
        recycler = findViewById(R.id.recycler)
        adapter = GameAdapter(
            onRun = { entry -> launchGame(entry) },
            onSettings = { entry ->
                startActivity(
                    Intent(this, GameSettingsActivity::class.java)
                        .putExtra(GameSettingsActivity.EXTRA_GAME_ID, entry.id)
                        .putExtra(GameSettingsActivity.EXTRA_GAME_NAME, entry.displayName),
                )
            },
            onRemove = { entry -> confirmRemove(entry) },
            onSave = { entry, action -> startSaveAction(entry, action) },
            onControls = { entry -> startActivity(Intent(this, ControlsActivity::class.java).putExtra("game_id",entry.id).putExtra("game_name",entry.displayName)) },
            libraryRoot = rootDir,
        )
        recycler.layoutManager = GridLayoutManager(this, gridSpanCount())
        recycler.adapter = adapter

        findViewById<FloatingActionButton>(R.id.fab_import).setOnClickListener {
            importGame.launch(arrayOf("application/vnd.ms-cab-compressed", "application/x-rar-compressed", "application/zip", "application/octet-stream", "*/*"))
        }
    }

    override fun onSaveInstanceState(state: Bundle) {state.putString("library_query",query);state.putBoolean("recent_first",recentFirst);super.onSaveInstanceState(state)}
    override fun onResume() {
        super.onResume()
        launching = false
        refreshLibrary()
    }

    override fun onCreateOptionsMenu(menu: Menu): Boolean {
        menuInflater.inflate(R.menu.main_menu, menu)
        menu.add(0,990,0,"全域操作 / Default controls")
        menu.add(0,993,0,"程式圖書館 / Catalog")
        menu.add(0,991,0,"最近遊玩／名稱 / Recent or name")
        val search = androidx.appcompat.widget.SearchView(this)
        search.queryHint = "搜尋名稱／別名 / Search programs"
        search.setOnQueryTextListener(object : androidx.appcompat.widget.SearchView.OnQueryTextListener {
            override fun onQueryTextSubmit(text: String?): Boolean { query=text.orEmpty();showFiltered();return true }
            override fun onQueryTextChange(text: String?): Boolean { query=text.orEmpty();showFiltered();return true }
        })
        search.setQuery(query,false)
        menu.add(0,992,0,"搜尋 / Search").apply { actionView=search; setShowAsAction(MenuItem.SHOW_AS_ACTION_IF_ROOM or MenuItem.SHOW_AS_ACTION_COLLAPSE_ACTION_VIEW) }
        return true
    }

    override fun onOptionsItemSelected(item: MenuItem): Boolean {
        return when (item.itemId) {
            993 -> { startActivity(Intent(this,CatalogActivity::class.java)); true }
            991 -> { recentFirst = !recentFirst; showFiltered(); true }
            990 -> { startActivity(Intent(this,ControlsActivity::class.java)); true }
            R.id.action_settings -> {
                startActivity(Intent(this, SettingsActivity::class.java))
                true
            }
            R.id.action_about -> {
                AlertDialog.Builder(this)
                    .setTitle(R.string.about_title)
                    .setMessage(NativeBridge.banner())
                    .setPositiveButton(android.R.string.ok, null)
                    .show()
                true
            }
            else -> super.onOptionsItemSelected(item)
        }
    }

    /** Roughly 110dp-wide tiles, like a game launcher grid; at least 2 columns. */
    private fun gridSpanCount(): Int {
        val widthDp = resources.configuration.screenWidthDp
        return (widthDp / 110).coerceAtLeast(2)
    }

    private fun refreshLibrary() {
        val generation = ++refreshGeneration
        lifecycleScope.launch {
            val result = withContext(Dispatchers.IO) { runCatching { NativeBridge.listGames(rootDir) } }
            if(generation != refreshGeneration) return@launch
            result.onSuccess { libraryItems = parseGamesOrToast(it); showFiltered() }
                .onFailure { Snackbar.make(recycler,"Library: ${it.message}",Snackbar.LENGTH_LONG).show() }
        }
    }

    private fun parseGamesOrToast(raw: String): List<GameEntry> {
        return try {
            // Library returns [] when empty, JSON object {ok:false,error:...} on error.
            if (raw.startsWith("{")) {
                val obj = JSONObject(raw)
                if (!obj.optBoolean("ok", true)) {
                    Snackbar.make(
                        recycler,
                        getString(R.string.error_listing_games, obj.optString("error")),
                        Snackbar.LENGTH_LONG,
                    ).show()
                }
                emptyList()
            } else {
                JSONArray(raw)
                GameEntry.listFromJson(raw)
            }
        } catch (e: Exception) {
            Snackbar.make(recycler, "Library parse failed: ${e.message}", Snackbar.LENGTH_LONG).show()
            emptyList()
        }
    }

    private fun handleImport(uri: Uri) {
        val name = queryDisplayName(uri) ?: "import.cab"
        val snack = Snackbar.make(recycler, R.string.import_in_progress, Snackbar.LENGTH_INDEFINITE)
        snack.show()
        lifecycleScope.launch {
            val result = withContext(Dispatchers.IO) {
                runCatching {
                    val cabFile = LibraryPaths.copyUriToCache(this@MainActivity, uri, name)
                    importArchiveOrExe(cabFile)
                }
            }
            snack.dismiss()
            result.fold(
                onSuccess = { raw ->
                    handleImportResult(raw)
                    refreshLibrary()
                },
                onFailure = { err ->
                    Snackbar.make(
                        recycler,
                        getString(R.string.import_failed, err.message ?: "unknown"),
                        Snackbar.LENGTH_LONG,
                    ).show()
                },
            )
        }
    }

    private fun importArchiveOrExe(file: java.io.File): String {
        val signature = file.inputStream().use { input ->
            ByteArray(4).also { buffer ->
                var offset = 0
                while (offset < buffer.size) {
                    val count = input.read(buffer, offset, buffer.size - offset)
                    if (count < 0) break
                    offset += count
                }
            }
        }
        val isZip = signature[0] == 'P'.code.toByte() &&
            signature[1] == 'K'.code.toByte() &&
            (signature[2] == 3.toByte() || signature[2] == 5.toByte() || signature[2] == 7.toByte())
        val isCab = signature.contentEquals(byteArrayOf(
            'M'.code.toByte(),
            'S'.code.toByte(),
            'C'.code.toByte(),
            'F'.code.toByte(),
        ))
        val isExe = signature[0] == 'M'.code.toByte() && signature[1] == 'Z'.code.toByte()

        return when {
            isZip -> NativeBridge.importZip(rootDir, file.absolutePath)
            isCab -> NativeBridge.importCab(rootDir, file.absolutePath)
            isExe -> NativeBridge.importExe(rootDir, file.absolutePath)
            file.extension.equals("rar", ignoreCase = true) ->
                NativeBridge.importRar(rootDir, file.absolutePath)
            file.extension.equals("zip", ignoreCase = true) -> NativeBridge.importZip(rootDir, file.absolutePath)
            file.extension.equals("cab", ignoreCase = true) -> NativeBridge.importCab(rootDir, file.absolutePath)
            file.extension.equals("exe", ignoreCase = true) -> NativeBridge.importExe(rootDir, file.absolutePath)
            else -> error("Unsupported file: choose a RAR, ZIP, CAB, or ARM EXE, or a Windows installer")
        }
    }

    private fun handleImportResult(raw: String) {
        try {
            val obj = JSONObject(raw)
            if (!obj.optBoolean("ok", true)) {
                Snackbar.make(
                    recycler,
                    getString(R.string.import_failed, obj.optString("error")),
                    Snackbar.LENGTH_LONG,
                ).show()
                return
            }
            val name = obj.optString("display_name", obj.optString("id", "?"))
            val note = obj.optString("import_note")
            val text = getString(R.string.import_success, name) +
                if (note.isNotEmpty()) "\n" + getString(R.string.import_note, note) else ""
            Snackbar.make(recycler, text, Snackbar.LENGTH_LONG).show()
        } catch (e: Exception) {
            Snackbar.make(recycler, raw, Snackbar.LENGTH_LONG).show()
        }
    }

    private fun queryDisplayName(uri: Uri): String? {
        return contentResolver.query(uri, null, null, null, null)?.use { c ->
            val ix = c.getColumnIndex(OpenableColumns.DISPLAY_NAME)
            if (ix >= 0 && c.moveToFirst()) c.getString(ix) else null
        }
    }

    private fun startSaveAction(entry: GameEntry, action: String) {
        saveGame = entry
        when (action) {
            "export" -> exportSave.launch("${entry.id}-save.zip")
            "import" -> importSave.launch(arrayOf("application/zip", "*/*"))
            else -> AlertDialog.Builder(this)
                .setTitle(R.string.save_reset_title)
                .setMessage(getString(R.string.save_reset_message, entry.displayName))
                .setPositiveButton(R.string.action_reset_save) { _, _ -> runSave(entry, "reset", null) }
                .setNegativeButton(android.R.string.cancel, null)
                .show()
        }
    }

    /**
     * Runs a save action through the shared Rust code. The zip goes through the
     * cache because a document picker hands out a content URI, not a path.
     */
    private fun runSave(entry: GameEntry, action: String, uri: Uri?) {
        lifecycleScope.launch {
            val message = withContext(Dispatchers.IO) {
                runCatching {
                    val zip = java.io.File(cacheDir, "save-transfer.zip")
                    if (action == "import" && uri != null) {
                        contentResolver.openInputStream(uri)!!.use { input ->
                            zip.outputStream().use { input.copyTo(it) }
                        }
                    }
                    val reply = JSONObject(NativeBridge.saveAction(rootDir, entry.id, action, zip.absolutePath))
                    if (!reply.optBoolean("ok")) error(reply.optString("error"))
                    val count = reply.optInt("count")
                    if (action == "export" && uri != null && count > 0) {
                        contentResolver.openOutputStream(uri)!!.use { output ->
                            zip.inputStream().use { it.copyTo(output) }
                        }
                    }
                    zip.delete()
                    when {
                        action == "export" && count == 0 -> getString(R.string.save_none)
                        action == "export" -> getString(R.string.save_exported, count)
                        action == "import" -> getString(R.string.save_imported, count)
                        else -> getString(R.string.save_reset_done)
                    }
                }.getOrElse { getString(R.string.save_failed, it.message ?: "unknown") }
            }
            Snackbar.make(recycler, message, Snackbar.LENGTH_LONG).show()
        }
    }

    private fun confirmRemove(entry: GameEntry) {
        AlertDialog.Builder(this)
            .setTitle(R.string.remove_title)
            .setMessage(getString(R.string.remove_message, entry.displayName))
            .setPositiveButton(R.string.remove) { _, _ ->
                val raw = NativeBridge.removeGame(rootDir, entry.id)
                val obj = runCatching { JSONObject(raw) }.getOrNull()
                if (obj?.optBoolean("ok") == true) {
                    refreshLibrary()
                } else {
                    Snackbar.make(
                        recycler,
                        obj?.optString("error") ?: raw,
                        Snackbar.LENGTH_LONG,
                    ).show()
                }
            }
            .setNegativeButton(android.R.string.cancel, null)
            .show()
    }

    private fun launchGame(entry: GameEntry) {
        if(launching) return
        launching = true
        try {
            val file = java.io.File(rootDir,"games/${entry.id}/presentation.json")
            val value = if(file.isFile) JSONObject(file.readText()) else JSONObject()
            value.put("last_played",System.currentTimeMillis()/1000)
            val staged = java.io.File(file.parentFile,"presentation.json.new")
            staged.writeText(value.toString()); check(staged.renameTo(file)) { "Cannot record recent program" }
        } catch(e: Exception) { Snackbar.make(recycler,"Recent: ${e.message}",Snackbar.LENGTH_LONG).show() }
        try { startActivity(
            Intent(this, GameActivity::class.java)
                .putExtra(GameActivity.EXTRA_GAME_ID, entry.id)
                .putExtra(GameActivity.EXTRA_GAME_NAME, entry.alias.ifBlank { entry.displayName }),
        ) } catch(e: Exception) { launching=false; Snackbar.make(recycler,"Start failed: ${e.message}",Snackbar.LENGTH_LONG).show() }
    }
}
