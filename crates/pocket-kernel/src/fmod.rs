//! Per-process FMOD metadata. EverQuest sessions must not share handles,
//! decoded file caches, channel assignments or volume settings.
use crate::GuestFormat;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct FmodClip {
    pub format: GuestFormat,
    pub samples: Arc<Vec<i16>>,
    pub looped: bool,
}

pub struct FmodChannel {
    pub clip: u32,
    pub stream: bool,
}

pub struct FmodState {
    pub clips: HashMap<u32, FmodClip>,
    pub by_path: HashMap<String, (GuestFormat, Arc<Vec<i16>>)>,
    pub channels: HashMap<u32, FmodChannel>,
    pub next_handle: u32,
    pub next_channel: u32,
    pub sfx_volume: u32,
}

impl Default for FmodState {
    fn default() -> Self {
        Self {
            clips: HashMap::new(),
            by_path: HashMap::new(),
            channels: HashMap::new(),
            next_handle: 0,
            next_channel: 0,
            sfx_volume: 255,
        }
    }
}
