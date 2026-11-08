package com.pockethle.app

import android.app.Activity
import androidx.preference.PreferenceManager

/**
 * The interface color theme: red unless the player picked another in Settings.
 * The colors are the same list the web page and the Windows window use
 * (`crates/pocket-library/src/theme.rs`); each one is a style in themes.xml.
 */
object Themes {
    const val KEY = "ui_theme"
    private val styles = mapOf(
    // BEGIN GENERATED THEMES (tools/ui/gen-themes.py)
        "red" to R.style.Theme_PocketHLE,
        "classic" to R.style.Theme_PocketHLE_Classic,
        "blue" to R.style.Theme_PocketHLE_Blue,
        "green" to R.style.Theme_PocketHLE_Green,
        "purple" to R.style.Theme_PocketHLE_Purple,
        "orange" to R.style.Theme_PocketHLE_Orange,
        "graphite" to R.style.Theme_PocketHLE_Graphite,
        "nightred" to R.style.Theme_PocketHLE_NightRed,
    // END GENERATED THEMES
    )

    /** The style for a saved id; anything unknown (or empty) is the red default. */
    fun styleFor(id: String?): Int = styles[id] ?: R.style.Theme_PocketHLE

    /** The saved theme id (null until the player picks one). */
    fun id(activity: Activity): String? =
        PreferenceManager.getDefaultSharedPreferences(activity).getString(KEY, null)

    /** Call before `super.onCreate` so the whole screen uses the chosen theme. */
    fun apply(activity: Activity) {
        activity.setTheme(styleFor(id(activity)))
    }
}
