//! Read-only catalog guidance. A suggestion never proves that a local binary
//! matches a catalog version, or that its controls passed platform acceptance.
use crate::controls::Profile;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::OnceLock};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Software {
    pub id: String,
    pub name: String,
    pub cat: String,
    pub status: String,
    #[serde(default)]
    pub cpus: Vec<String>,
    #[serde(default)]
    pub platforms: Vec<String>,
    #[serde(default)]
    pub year: String,
    #[serde(default)]
    pub shot: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Words<T> {
    pub zh: T,
    pub en: T,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Guidance {
    pub recommended: Profile,
    pub needs: Vec<String>,
    pub steps: Words<Vec<String>>,
    pub save: Words<String>,
}
const GUIDES: &str = include_str!("../data/control-guides.json");
pub fn entries() -> &'static [Software] {
    static DATA: OnceLock<Vec<Software>> = OnceLock::new();
    DATA.get_or_init(|| {
        serde_json::from_str(include_str!("../../../docs/library/index.json"))
            .expect("bundled catalog")
    })
}
pub fn guidance(id: &str) -> Option<&'static Guidance> {
    static DATA: OnceLock<HashMap<String, Guidance>> = OnceLock::new();
    DATA.get_or_init(|| serde_json::from_str(GUIDES).expect("bundled control guidance"))
        .get(id)
}
pub fn description(id: &str, chinese: bool) -> &'static str {
    static ZH: OnceLock<HashMap<String, String>> = OnceLock::new();
    static EN: OnceLock<HashMap<String, String>> = OnceLock::new();
    let data = if chinese {
        ZH.get_or_init(|| {
            serde_json::from_str(include_str!("../../../docs/library/desc.zh-TW.json"))
                .expect("bundled descriptions")
        })
    } else {
        EN.get_or_init(|| {
            serde_json::from_str(include_str!("../../../docs/library/desc.en.json"))
                .expect("bundled descriptions")
        })
    };
    data.get(id)
        .map(String::as_str)
        .unwrap_or("Description unavailable for this version.")
}
pub fn json(chinese: bool) -> String {
    let rows: Vec<_> = entries().iter().map(|entry| serde_json::json!({
        "software":entry,"guidance":guidance(&entry.id),"description":description(&entry.id,chinese)
    })).collect();
    serde_json::to_string(&rows).expect("serializable catalog")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn guides_match_exact_catalog_ids_without_family_inheritance() {
        assert!(entries().len() > 1000);
        let ids: std::collections::HashSet<_> = entries().iter().map(|e| e.id.as_str()).collect();
        let guides: HashMap<String, Guidance> = serde_json::from_str(GUIDES).unwrap();
        for id in guides.keys() {
            assert!(ids.contains(id.as_str()), "unknown guide: {id}");
        }
        for id in ["everquest", "everquest-e2", "everquest-e3"] {
            assert_eq!(guidance(id).unwrap().recommended, Profile::Stylus);
        }
        assert!(guidance("wm-uno-free-fall").is_none());
    }
}
