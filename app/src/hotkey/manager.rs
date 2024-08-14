pub struct HotKey {
    pub vk_code: u32,
    pub timpstamp: u64,
}

impl HotKey {
    pub fn new(vk_code: u32) -> HotKey {
        HotKey {
            vk_code,
            timpstamp:0,
        }
    }
    
}

pub struct HotKeyManager {
    pub hot_keys: Vec<HotKey>,
}


impl HotKeyManager {
    pub fn new() -> HotKeyManager {
        HotKeyManager {
            hot_keys: Vec::new(),
        }
    }
    pub fn last_hot_key(&self) -> Option<&HotKey> {
        self.hot_keys.last()
    }
}