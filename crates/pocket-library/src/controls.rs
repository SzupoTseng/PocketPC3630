//! Versioned, per-program controls shared by all frontends. Progress archives
//! deliberately remain independent: changing a layout must never restore a save.
use serde::{Deserialize, Serialize};
use std::io;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Profile {
    Stylus,
    Direction,
    #[default]
    Full,
    Text,
    Desktop,
    Custom,
    Device,
    Ppc2003,
}
impl Profile {
    pub const ALL: [Self; 8] = [
        Self::Stylus,
        Self::Direction,
        Self::Full,
        Self::Text,
        Self::Desktop,
        Self::Custom,
        Self::Device,
        Self::Ppc2003,
    ];
    pub fn label(self) -> &'static str {
        match self {
            Self::Stylus => "Stylus first",
            Self::Direction => "Direction + action",
            Self::Full => "Pocket PC buttons",
            Self::Text => "Text / reading",
            Self::Desktop => "Mouse + keyboard",
            Self::Custom => "Custom",
            Self::Device => "CE hardware buttons",
            Self::Ppc2003 => "Pocket PC 2003 panel",
        }
    }
    pub fn keys(self) -> Vec<u16> {
        match self {
            Self::Stylus | Self::Desktop => vec![],
            Self::Direction => vec![38, 40, 37, 39, 13],
            Self::Text => vec![9, 27, 33, 34],
            // CE hardware application keys are independent of the existing
            // GAPI A/B/C mapping; never globally change the latter for EQ.
            Self::Device => vec![38, 40, 37, 39, 0x86, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6],
            Self::Ppc2003 => ppc2003_panel().buttons.iter().map(|b| b.vk).collect(),
            Self::Full | Self::Custom => vec![38, 40, 37, 39, 13, 9, 27, 17, 32, 16],
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct PanelButton {
    pub vk: u16,
    pub label: String,
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub mode: String,
}
#[derive(Debug, Deserialize)]
pub struct DevicePanel {
    pub buttons: Vec<PanelButton>,
}
pub fn ppc2003_panel() -> &'static DevicePanel {
    static PANEL: std::sync::OnceLock<DevicePanel> = std::sync::OnceLock::new();
    PANEL.get_or_init(|| {
        serde_json::from_str(include_str!("../data/ppc2003-panel.json"))
            .expect("bundled PPC2003 panel must be valid")
    })
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Binding {
    /// Stable host key or standardized controller name, e.g. "Up", "PadA".
    pub input: String,
    pub vk: u16,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ButtonPosition {
    pub vk: u16,
    /// Normalized coordinates and size; frontends resolve their own safe area.
    pub x: f32,
    pub y: f32,
    pub size: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Controls {
    pub version: u32,
    pub profile: Profile,
    pub keys: Vec<u16>,
    pub keyboard: bool,
    pub menu_key: String,
    pub bindings: Vec<Binding>,
    pub portrait: Vec<ButtonPosition>,
    pub landscape: Vec<ButtonPosition>,
    pub opacity: f32,
    pub volume: f32,
    pub zoom: f32,
    pub zoom_x: f32,
    pub zoom_y: f32,
    pub auto_hide_gamepad: bool,
    pub relative_mouse: bool,
    pub sensitivity: f32,
}
impl Default for Controls {
    fn default() -> Self {
        Self {
            version: 1,
            profile: Profile::Full,
            keys: Profile::Full.keys(),
            keyboard: false,
            menu_key: "F1".into(),
            bindings: vec![],
            portrait: vec![],
            landscape: vec![],
            opacity: 0.7,
            volume: 1.0,
            zoom: 1.0,
            zoom_x: 0.5,
            zoom_y: 0.5,
            auto_hide_gamepad: true,
            relative_mouse: false,
            sensitivity: 1.0,
        }
    }
}
impl Controls {
    pub fn panel_button(&self, vk: u16) -> Option<&'static PanelButton> {
        (self.profile == Profile::Ppc2003)
            .then(|| ppc2003_panel().buttons.iter().find(|b| b.vk == vk))
            .flatten()
    }
    pub fn preset(profile: Profile) -> Self {
        Self {
            profile,
            keys: profile.keys(),
            keyboard: profile == Profile::Text,
            ..Self::default()
        }
    }
    pub fn validate(&self) -> io::Result<()> {
        let invalid = || {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "unsupported or invalid control configuration",
            )
        };
        if self.menu_key.is_empty()
            || self.menu_key.len() > 64
            || self.version != 1
            || self.keys.len() > 64
            || self.keys.iter().any(|k| *k == 0 || *k > 255)
            || self.bindings.len() > 256
            || self
                .bindings
                .iter()
                .any(|b| b.input.is_empty() || b.input.len() > 64 || b.vk > 255)
            || !self.opacity.is_finite()
            || !(0.1..=1.0).contains(&self.opacity)
            || !self.volume.is_finite()
            || !(0.0..=1.0).contains(&self.volume)
            || !self.zoom.is_finite()
            || !(1.0..=4.0).contains(&self.zoom)
            || !self.zoom_x.is_finite()
            || !(0.0..=1.0).contains(&self.zoom_x)
            || !self.zoom_y.is_finite()
            || !(0.0..=1.0).contains(&self.zoom_y)
            || !self.sensitivity.is_finite()
            || !(0.1..=10.0).contains(&self.sensitivity)
        {
            return Err(invalid());
        }
        for layout in [&self.portrait, &self.landscape] {
            if layout.len() > 64
                || layout.iter().any(|b| {
                    b.vk == 0
                        || b.vk > 255
                        || !b.x.is_finite()
                        || !(0.0..=1.0).contains(&b.x)
                        || !b.y.is_finite()
                        || !(0.0..=1.0).contains(&b.y)
                        || !b.size.is_finite()
                        || !(0.03..=0.4).contains(&b.size)
                })
            {
                return Err(invalid());
            }
        }
        for (i, binding) in self.bindings.iter().enumerate() {
            if self.bindings[..i]
                .iter()
                .any(|other| crate::keybindings::keys_match(&other.input, &binding.input))
            {
                return Err(invalid());
            }
        }
        if self
            .keys
            .iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
            != self.keys.len()
        {
            return Err(invalid());
        }
        Ok(())
    }
    /// Same guest-space focus for rendering UVs and inverse stylus mapping.
    /// Keep the visible crop inside the framebuffer, even at a corner.
    pub fn zoom_center(&self) -> (f32, f32) {
        let half = 0.5 / self.zoom;
        (
            self.zoom_x.clamp(half, 1.0 - half),
            self.zoom_y.clamp(half, 1.0 - half),
        )
    }
    pub fn mapped_key(&self, name: &str) -> Option<u16> {
        self.bindings
            .iter()
            .find(|b| crate::keybindings::keys_match(&b.input, name))
            .map(|b| b.vk)
    }
}

/// Default host-controller vocabulary shared with Android's existing mapping.
pub fn gamepad_default(name: &str) -> Option<u16> {
    Some(match name {
        "PadUp" => 38,
        "PadDown" => 40,
        "PadLeft" => 37,
        "PadRight" => 39,
        "PadA" => 13,
        "PadB" => 32,
        "PadX" => 16,
        "PadY" => 17,
        "PadSelect" | "PadL1" | "PadL2" => 9,
        "PadStart" | "PadR1" | "PadR2" => 27,
        _ => return None,
    })
}

pub fn read(path: &Path) -> io::Result<Option<Controls>> {
    let text = match std::fs::read(path) {
        Ok(b) => b,
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(e),
    };
    if text.len() > 128 * 1024 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "controls file too large",
        ));
    }
    let value: Controls = serde_json::from_slice(&text).map_err(io::Error::other)?;
    value.validate()?;
    Ok(Some(value))
}
pub fn write(path: &Path, value: &Controls) -> io::Result<()> {
    value.validate()?;
    let bytes = serde_json::to_vec_pretty(value).map_err(io::Error::other)?;
    let temp = path.with_extension("json.new");
    std::fs::write(&temp, bytes)?;
    // A same-directory rename keeps readers from seeing truncated JSON.
    std::fs::rename(&temp, path)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Presentation {
    pub alias: String,
    pub favorite: bool,
    pub last_played: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn legacy_web_controls_migrate_without_discarding_the_selection() {
        let value: Controls =
            serde_json::from_str(r#"{"version":1,"profile":"stylus","keys":[],"keyboard":false}"#)
                .unwrap();
        value.validate().unwrap();
        assert_eq!(value.profile, Profile::Stylus);
        assert!(value.keys.is_empty());
    }
    #[test]
    fn zoom_focus_keeps_edges_reachable_and_visible_at_each_corner() {
        let mut value = Controls::default();
        for zoom in [1.0, 2.0, 4.0] {
            value.zoom = zoom;
            for x in [0.0, 0.5, 1.0] {
                for y in [0.0, 0.5, 1.0] {
                    value.zoom_x = x;
                    value.zoom_y = y;
                    let (cx, cy) = value.zoom_center();
                    let half = 0.5 / zoom;
                    assert!(cx - half >= 0.0 && cx + half <= 1.0);
                    assert!(cy - half >= 0.0 && cy + half <= 1.0);
                    if x == 1.0 {
                        assert_eq!(cx + half, 1.0);
                    }
                    if y == 0.0 {
                        assert_eq!(cy - half, 0.0);
                    }
                }
            }
        }
    }
    #[test]
    fn different_programs_and_global_defaults_remain_independent() {
        let dir = tempfile::tempdir().unwrap();
        write(&dir.path().join("global"), &Controls::preset(Profile::Full)).unwrap();
        write(&dir.path().join("eq1"), &Controls::preset(Profile::Stylus)).unwrap();
        write(&dir.path().join("reader"), &Controls::preset(Profile::Text)).unwrap();
        assert_eq!(
            read(&dir.path().join("eq1")).unwrap().unwrap().profile,
            Profile::Stylus
        );
        assert_eq!(
            read(&dir.path().join("global")).unwrap().unwrap().profile,
            Profile::Full
        );
        assert_eq!(
            read(&dir.path().join("reader")).unwrap().unwrap().profile,
            Profile::Text
        );
        let bad = Controls {
            version: 2,
            ..Controls::default()
        };
        assert!(write(&dir.path().join("eq1"), &bad).is_err());
        assert_eq!(
            read(&dir.path().join("eq1")).unwrap().unwrap().profile,
            Profile::Stylus
        );
    }
}
