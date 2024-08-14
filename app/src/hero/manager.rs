use std::collections::hash_map;

use crate::hotkey::script::Script;

pub struct HeroManager {
    pub hero_map: hash_map::HashMap<String, Vec<Script>>,
}

fn yasuo() {
    let mut scripts = Vec::new();
    scripts.push(Script::new(
        "双风",
        false,
        |keys| keys.last_hot_key().unwrap().vk_code == 0x41,
        |keys| {
            println!("双风");
        },
    ));
}

impl HeroManager {
    pub fn new() -> HeroManager {
        let mut hero_map = hash_map::HashMap::new();
        hero_map.insert("亚索".to_string(), Vec::new());

        HeroManager { hero_map }
    }
}
