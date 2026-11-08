//! The emulator's interface colors: the glossy Windows Mobile 5/6 look.
//!
//! Red is the default. The other colors are themes the player can pick; the
//! web page, the Windows window and the Android app all read the same list so
//! a theme looks the same everywhere. Only the player's own interface (title
//! bar, buttons, pause menu, settings) uses these colors; the software library
//! keeps its own look.

/// An sRGB color, `0xRRGGBB`.
pub type Rgb = u32;

/// One interface theme.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Theme {
    /// Stable name saved in settings (`"red"`, `"blue"`, …).
    pub id: &'static str,
    /// Name shown in Traditional Chinese.
    pub zh: &'static str,
    /// Name shown in English.
    pub en: &'static str,
    /// The main color: title bar, accents, sliders.
    pub flat: Rgb,
    /// Darker edge of the title bar and borders.
    pub edge: Rgb,
    /// Selected row and the main button.
    pub select: Rgb,
    /// Text on a selected row.
    pub select_ink: Rgb,
    /// Window and dialog background.
    pub panel: Rgb,
    /// Raised surfaces (inputs, group headers).
    pub panel2: Rgb,
    /// Body text.
    pub ink: Rgb,
    /// Lines around inputs and buttons.
    pub line: Rgb,
    /// Secondary text.
    pub dim: Rgb,
    /// True when the surfaces are dark (a night theme).
    pub dark: bool,
    /// Square corners, as on the Pocket PC 2003 itself (the classic theme).
    pub square: bool,
}

/// The theme used until the player chooses another.
pub const DEFAULT_ID: &str = "red";

/// Every theme, default first.
pub const THEMES: &[Theme] = &[
    Theme {
        id: "red",
        zh: "紅",
        en: "Red",
        flat: 0xc8372d,
        edge: 0xa3231b,
        select: 0xd8392e,
        select_ink: 0xffffff,
        panel: 0xfbf5f4,
        panel2: 0xf3e3e1,
        ink: 0x3a1412,
        line: 0xd7a6a2,
        dim: 0x8c6a67,
        dark: false,
        square: false,
    },
    Theme {
        id: "classic",
        zh: "2003 經典",
        en: "2003 classic",
        flat: 0x1055b5,
        edge: 0x0a3a8a,
        select: 0x00009c,
        select_ink: 0xffffff,
        panel: 0xffffff,
        panel2: 0xbdebff,
        ink: 0x000000,
        line: 0x848284,
        dim: 0x636563,
        dark: false,
        square: true,
    },
    Theme {
        id: "blue",
        zh: "藍＋橘",
        en: "Blue + orange",
        flat: 0x2f5fb8,
        edge: 0x1f4796,
        select: 0xf58a14,
        select_ink: 0x2a1600,
        panel: 0xf4f7fc,
        panel2: 0xe4ebf8,
        ink: 0x14213d,
        line: 0x9bb0d6,
        dim: 0x6a7a99,
        dark: false,
        square: false,
    },
    Theme {
        id: "green",
        zh: "綠",
        en: "Green",
        flat: 0x2e8b3d,
        edge: 0x237a34,
        select: 0xf0b020,
        select_ink: 0x2b2000,
        panel: 0xf4faf5,
        panel2: 0xe1f1e4,
        ink: 0x10281a,
        line: 0x9cc8a4,
        dim: 0x5f7d66,
        dark: false,
        square: false,
    },
    Theme {
        id: "purple",
        zh: "紫",
        en: "Purple",
        flat: 0x6b3fb0,
        edge: 0x5a2f96,
        select: 0xa874ec,
        select_ink: 0x1f0a3a,
        panel: 0xf8f5fc,
        panel2: 0xebe3f7,
        ink: 0x21143a,
        line: 0xb9a0e0,
        dim: 0x7b6a99,
        dark: false,
        square: false,
    },
    Theme {
        id: "orange",
        zh: "橘",
        en: "Orange",
        flat: 0xd9770f,
        edge: 0xb85a08,
        select: 0x2d5fbd,
        select_ink: 0xffffff,
        panel: 0xfdf8f1,
        panel2: 0xf6e8d3,
        ink: 0x33200c,
        line: 0xe0b784,
        dim: 0x9a7f5f,
        dark: false,
        square: false,
    },
    Theme {
        id: "graphite",
        zh: "石墨",
        en: "Graphite",
        flat: 0x4a525f,
        edge: 0x353b46,
        select: 0x7a8497,
        select_ink: 0x10151e,
        panel: 0xf5f6f8,
        panel2: 0xe6e9ef,
        ink: 0x1a1f29,
        line: 0xaab2c0,
        dim: 0x767e8d,
        dark: false,
        square: false,
    },
    Theme {
        id: "nightred",
        zh: "深色紅（夜間）",
        en: "Dark red (night)",
        flat: 0xb3322a,
        edge: 0x6a2a26,
        select: 0xd8392e,
        select_ink: 0xffffff,
        panel: 0x1d1112,
        panel2: 0x2a1819,
        ink: 0xf1e4e2,
        line: 0x5a3436,
        dim: 0xa58885,
        dark: true,
        square: false,
    },
];

/// The theme with this id, or the default for an unknown or empty one (a
/// settings file written by a newer version must not break the window).
pub fn by_id(id: &str) -> &'static Theme {
    THEMES.iter().find(|t| t.id == id).unwrap_or(&THEMES[0])
}

/// File in the library folder that remembers the chosen theme.
const FILE: &str = "ui-theme.txt";

/// The theme saved in a library folder, or the default when there is none or
/// it cannot be read.
pub fn load(root: &std::path::Path) -> &'static Theme {
    by_id(
        std::fs::read_to_string(root.join(FILE))
            .unwrap_or_default()
            .trim(),
    )
}

/// Remember `id` in a library folder. An unknown id is stored as the default.
pub fn save(root: &std::path::Path, id: &str) -> std::io::Result<()> {
    std::fs::write(root.join(FILE), by_id(id).id)
}

/// Split a color into red, green and blue bytes.
pub fn rgb(color: Rgb) -> [u8; 3] {
    [(color >> 16) as u8, (color >> 8) as u8, color as u8]
}

/// Relative luminance (WCAG 2.x) of a color, `0.0` black to `1.0` white.
fn luminance(color: Rgb) -> f64 {
    let channel = |c: u8| {
        let c = f64::from(c) / 255.0;
        if c <= 0.03928 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };
    let [r, g, b] = rgb(color);
    0.2126 * channel(r) + 0.7152 * channel(g) + 0.0722 * channel(b)
}

/// WCAG contrast ratio of two colors, `1.0` (none) to `21.0`.
pub fn contrast(a: Rgb, b: Rgb) -> f64 {
    let (la, lb) = (luminance(a), luminance(b));
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn red_is_first_and_the_default() {
        assert_eq!(THEMES[0].id, DEFAULT_ID);
        assert_eq!(by_id(DEFAULT_ID).id, "red");
    }

    #[test]
    fn ids_are_unique() {
        for (i, a) in THEMES.iter().enumerate() {
            assert!(THEMES[i + 1..].iter().all(|b| b.id != a.id), "{}", a.id);
        }
    }

    #[test]
    fn an_unknown_id_falls_back_to_red() {
        assert_eq!(by_id("").id, "red");
        assert_eq!(by_id("future-theme").id, "red");
    }

    #[test]
    fn text_is_readable_on_every_surface() {
        // WCAG AA for normal text is 4.5; the dim secondary text only needs
        // to clear 3.0 (it is never the only carrier of meaning).
        for t in THEMES {
            assert!(contrast(t.ink, t.panel) >= 4.5, "{} body text", t.id);
            assert!(contrast(t.ink, t.panel2) >= 4.5, "{} raised text", t.id);
            assert!(
                contrast(t.select_ink, t.select) >= 4.5,
                "{} selection",
                t.id
            );
            assert!(contrast(t.dim, t.panel) >= 3.0, "{} secondary text", t.id);
            assert!(contrast(0xffffff, t.edge) >= 4.5, "{} title text", t.id);
        }
    }

    #[test]
    fn a_dark_theme_has_dark_surfaces() {
        for t in THEMES {
            assert_eq!(t.dark, luminance(t.panel) < 0.2, "{}", t.id);
        }
    }

    #[test]
    fn the_choice_is_remembered_in_the_library_folder() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(load(dir.path()).id, "red");
        save(dir.path(), "green").unwrap();
        assert_eq!(load(dir.path()).id, "green");
        save(dir.path(), "no-such-theme").unwrap();
        assert_eq!(load(dir.path()).id, "red");
    }

    #[test]
    fn only_the_classic_theme_is_square() {
        for t in THEMES {
            assert_eq!(t.square, t.id == "classic", "{}", t.id);
        }
    }

    #[test]
    fn rgb_splits_the_bytes() {
        assert_eq!(rgb(0x123456), [0x12, 0x34, 0x56]);
    }
}
