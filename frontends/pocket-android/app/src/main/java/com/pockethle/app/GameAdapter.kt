package com.pockethle.app

import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.graphics.BitmapFactory
import java.io.File
import android.widget.ImageButton
import android.widget.PopupMenu
import android.widget.TextView
import androidx.recyclerview.widget.RecyclerView

/**
 * RecyclerView adapter for the library screen — launcher-style grid
 * of game tiles (cover art + title). Tapping a tile runs the game;
 * Settings/Remove live behind the tile's overflow (⋮) button so the
 * grid stays visually clean.
 */
class GameAdapter(
    private val onRun: (GameEntry) -> Unit,
    private val onSettings: (GameEntry) -> Unit,
    private val onRemove: (GameEntry) -> Unit,
    private val onSave: (GameEntry, String) -> Unit,
    private val onControls: (GameEntry) -> Unit,
    private val libraryRoot: String,
) : RecyclerView.Adapter<GameAdapter.ViewHolder>() {

    private var items: List<GameEntry> = emptyList()

    fun submit(newItems: List<GameEntry>) {
        items = newItems
        notifyDataSetChanged()
    }

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): ViewHolder {
        val view = LayoutInflater.from(parent.context)
            .inflate(R.layout.item_game_grid, parent, false)
        return ViewHolder(view)
    }

    override fun onBindViewHolder(holder: ViewHolder, position: Int) {
        holder.bind(items[position])
    }

    override fun getItemCount(): Int = items.size

    inner class ViewHolder(view: View) : RecyclerView.ViewHolder(view) {
        private val icon = view.findViewById<android.widget.ImageView>(R.id.game_icon)
        private val title: TextView = view.findViewById(R.id.game_title)
        private val publisherLabel: TextView = view.findViewById(R.id.game_publisher)
        private val moreBtn: ImageButton = view.findViewById(R.id.btn_more)

        fun bind(entry: GameEntry) {
            title.text = (if(entry.favorite) "★ " else "") + entry.alias.ifBlank { entry.displayName }
            title.maxLines = 4
            title.contentDescription = entry.displayName
            val iconFile = entry.icon?.let { File(libraryRoot, "games/${entry.id}/$it") }
            val bitmap = iconFile?.takeIf { it.isFile }?.let { BitmapFactory.decodeFile(it.absolutePath) }
            if (bitmap != null) {
                icon.setImageBitmap(bitmap)
                icon.imageTintList = null
            } else {
                icon.setImageResource(R.drawable.ic_game)
                icon.imageTintList = itemView.context.getColorStateList(com.google.android.material.R.color.material_dynamic_primary40)
            }
            publisherLabel.text = entry.controlsLabel + " · " + if(entry.controlsOverridden) "此軟體 / Custom" else "繼承 / Inherited"
            moreBtn.contentDescription = "${entry.displayName}: 操作、進度與設定 / Controls, progress and settings"
            itemView.setOnClickListener { onRun(entry) }
            moreBtn.setOnClickListener { anchor -> showOverflowMenu(anchor, entry) }
        }

        private fun showOverflowMenu(anchor: View, entry: GameEntry) {
            val popup = PopupMenu(anchor.context, anchor)
            popup.menu.add(0, 2, 0, R.string.action_export_save)
            popup.menu.add(0, 3, 1, R.string.action_import_save)
            popup.menu.add(0, 4, 2, R.string.action_reset_save)
            popup.menu.add(0, 0, 3, R.string.action_settings)
            popup.menu.add(0, 1, 4, R.string.action_remove)
            popup.menu.add(0, 5, 5, "操作方式 / Controls")
            popup.setOnMenuItemClickListener { item ->
                when (item.itemId) {
                    0 -> onSettings(entry)
                    1 -> onRemove(entry)
                    2 -> onSave(entry, "export")
                    3 -> onSave(entry, "import")
                    4 -> onSave(entry, "reset")
                    5 -> onControls(entry)
                }
                true
            }
            popup.show()
        }
    }
}
