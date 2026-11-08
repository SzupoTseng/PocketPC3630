package com.pockethle.app

import android.content.Intent
import android.net.Uri
import android.os.Bundle
import android.view.View
import android.widget.*
import androidx.appcompat.app.AlertDialog
import androidx.appcompat.app.AppCompatActivity
import androidx.appcompat.widget.SearchView
import androidx.appcompat.widget.Toolbar
import androidx.lifecycle.lifecycleScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import org.json.JSONArray
import org.json.JSONObject

/** Read-only public catalog. Choosing a local installation is always explicit. */
class CatalogActivity : AppCompatActivity() {
    private lateinit var list: LinearLayout
    private var rows=emptyList<JSONObject>()
    private var query=""
    private var need="all"
    private var limit=48
    private val chinese get()=resources.configuration.locales[0].language=="zh"
    private fun tr(zh:String,en:String)=if(chinese)zh else en
    private fun label(value:String)=TextView(this).apply { text=value;textSize=16f;setPadding(16,12,16,12) }
    override fun onCreate(state:Bundle?) {
        Themes.apply(this)
        super.onCreate(state)
        val root=LinearLayout(this).apply { orientation=LinearLayout.VERTICAL }
        val toolbar=Toolbar(this).apply { title=tr("程式圖書館","Software catalog") };root.addView(toolbar)
        val search=SearchView(this).apply { queryHint=tr("名稱／版本","Name / version");isIconified=false };root.addView(search)
        val needs=arrayOf("all","stylus","direction","text","hardware","mouse","keyboard")
        val filter=Spinner(this).apply { adapter=ArrayAdapter(this@CatalogActivity,android.R.layout.simple_spinner_dropdown_item,needs) };root.addView(filter)
        val scroll=ScrollView(this);list=LinearLayout(this).apply { orientation=LinearLayout.VERTICAL };scroll.addView(list);root.addView(scroll,LinearLayout.LayoutParams(-1,0,1f))
        setContentView(root);setSupportActionBar(toolbar);supportActionBar?.setDisplayHomeAsUpEnabled(true)
        search.setOnQueryTextListener(object:SearchView.OnQueryTextListener {
            override fun onQueryTextSubmit(value:String?):Boolean {query=value.orEmpty();limit=48;render();return true}
            override fun onQueryTextChange(value:String?):Boolean {query=value.orEmpty();limit=48;render();return true}
        })
        filter.onItemSelectedListener=object:AdapterView.OnItemSelectedListener {
            override fun onNothingSelected(parent:AdapterView<*>?) {}
            override fun onItemSelected(parent:AdapterView<*>?,view:View?,position:Int,id:Long) {need=needs[position];limit=48;render()}
        }
        lifecycleScope.launch {
            val result=withContext(Dispatchers.IO) {runCatching {
                val array=JSONArray(NativeBridge.catalog(if(chinese)1 else 0));(0 until array.length()).map {array.getJSONObject(it)}
            }}
            result.onSuccess {rows=it;render()}.onFailure {list.removeAllViews();list.addView(label(tr("無法載入：","Cannot load: ")+it.message))}
        }
    }
    override fun onSupportNavigateUp():Boolean {finish();return true}
    private fun render() {
        if(!::list.isInitialized)return
        list.removeAllViews()
        val visible=rows.filter {row->val entry=row.getJSONObject("software");val guide=row.optJSONObject("guidance");
            (entry.getString("name")+" "+entry.getString("id")).contains(query,true) && (need=="all" || guide?.optJSONArray("needs")?.let {a->(0 until a.length()).any {a.optString(it)==need}}==true)
        }
        list.addView(label(tr("公開目錄；不代表此平台已驗證操作。","Public catalog; controls on this platform are not verified.")+" ${visible.size}"))
        for(row in visible.take(limit)) {
            val entry=row.getJSONObject("software")
            list.addView(Button(this).apply {text=entry.getString("name")+"\n"+entry.optString("year")+" · "+entry.getString("status")+" · "+entry.optJSONArray("cpus")?.toString().orEmpty();isAllCaps=false;setOnClickListener {showDetails(row)}})
        }
        if(visible.size>limit)list.addView(Button(this).apply {text=tr("顯示更多","Show more");setOnClickListener {limit+=48;render()}})
    }
    private fun showDetails(row:JSONObject) {
        val entry=row.getJSONObject("software");val guide=row.optJSONObject("guidance")
        val body=LinearLayout(this).apply {orientation=LinearLayout.VERTICAL}
        val scroll=ScrollView(this);scroll.addView(body)
        body.addView(label(row.optString("description")))
        body.addView(label("${entry.optString("year")} · ${entry.optJSONArray("cpus")} · ${entry.optJSONArray("platforms")}"))
        body.addView(label(tr("畫面占位：本機未附此版本截圖。","Screenshot placeholder: no local image bundled for this version.")))
        if(entry.optString("shot").isNotBlank() && !entry.isNull("shot"))body.addView(Button(this).apply {text=tr("開啟目錄畫面紀錄","Open catalog screen record");setOnClickListener {startActivity(Intent(Intent.ACTION_VIEW,Uri.parse("https://szupotseng.github.io/PocketPC3630/library/shots/${entry.getString("shot")}")))}})
        val language=if(chinese)"zh" else "en"
        val steps=guide?.optJSONObject("steps")?.optJSONArray(language)
        if(steps!=null)for(i in 0 until steps.length())body.addView(label("${i+1}. ${steps.getString(i)}"))
        else body.addView(label(tr("此版本尚無專屬操作教學；請依軟體說明選擇方案。","No guide for this version; choose controls using the program's instructions.")))
        body.addView(label(guide?.optJSONObject("save")?.optString(language) ?: tr("先使用軟體內存檔，再備份進度 ZIP；不是執行畫面快照。","Save inside the program before backing up progress; this is not a running-state snapshot.")))
        body.addView(label(tr("素材／安裝：未比對本機版本。啟動紀錄：","Assets / installation: local version unmatched. Catalog startup record: ")+entry.optString("status")+tr("。操作／進度：本平台未逐項驗證。",". Controls / progress: not individually verified on this platform.")))
        val profiles=arrayOf("stylus","direction","full","text","desktop","custom","device")
        val picker=Spinner(this).apply {adapter=ArrayAdapter(this@CatalogActivity,android.R.layout.simple_spinner_dropdown_item,profiles);setSelection(profiles.indexOf(guide?.optString("recommended")).coerceAtLeast(0))};body.addView(picker)
        val preview=LinearLayout(this).apply {orientation=LinearLayout.VERTICAL};body.addView(preview)
        var landscape=false
        fun redraw() {
            preview.removeAllViews();val profile=profiles[picker.selectedItemPosition]
            preview.addView(label(if(landscape)tr("橫向示意（不送客體事件）","Landscape preview (no guest input)") else tr("直向示意（不送客體事件）","Portrait preview (no guest input)")))
            for(vk in ControlNames.preset(profile))preview.addView(Button(this).apply {text=ControlNames.label(vk);setOnClickListener {Toast.makeText(this@CatalogActivity,"Preview: CE VK $vk",Toast.LENGTH_SHORT).show()}})
            if(profile=="device")preview.addView(label(tr("通用 CE 按鍵；機型／OS／skin 參照尚未驗證，Power／Reset 未提供。","Generic CE keys; device / OS / skin reference unverified. Power / Reset unavailable.")))
        }
        picker.onItemSelectedListener=object:AdapterView.OnItemSelectedListener {override fun onNothingSelected(parent:AdapterView<*>?){};override fun onItemSelected(parent:AdapterView<*>?,v:View?,position:Int,id:Long){redraw()}}
        body.addView(Button(this).apply {text=tr("旋轉預覽","Rotate preview");setOnClickListener {landscape=!landscape;redraw()}})
        body.addView(Button(this).apply {text=tr("選擇已匯入軟體來設定…","Configure an installed program…");setOnClickListener {
            val games=runCatching {GameEntry.listFromJson(NativeBridge.listGames(LibraryPaths.root(this@CatalogActivity)))}.getOrElse {emptyList()}
            if(games.isEmpty()) {Toast.makeText(this@CatalogActivity,tr("請先匯入自己的軟體；目錄只供預覽。","Import your program first; the catalog is preview only."),Toast.LENGTH_LONG).show();return@setOnClickListener}
            AlertDialog.Builder(this@CatalogActivity).setTitle(tr("版本尚未比對；明確選擇本機軟體","Version unmatched; choose a local program"))
                .setItems(games.map {it.displayName}.toTypedArray()){_,index->val game=games[index];startActivity(Intent(this@CatalogActivity,ControlsActivity::class.java).putExtra("game_id",game.id).putExtra("game_name",game.displayName).putExtra("preview_profile",profiles[picker.selectedItemPosition]))}.setNegativeButton(android.R.string.cancel,null).show()
        }})
        redraw();AlertDialog.Builder(this).setTitle(entry.getString("name")).setView(scroll).setPositiveButton(android.R.string.ok,null).show()
    }
}
