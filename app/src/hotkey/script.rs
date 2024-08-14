use super::manager::HotKeyManager;

pub struct Script {
    pub name: String,
    pub hot_key: u32,
    pub block_input: bool,
    pub condition: fn(hot_key_manager: &mut HotKeyManager) -> bool,
    pub run: fn(hot_key_manager: &mut HotKeyManager),
}

impl Script {
    pub fn new(
        name: &str,
        hot_key: u32,
        block_input: bool,
        condition: fn(hot_key_manager: &mut HotKeyManager) -> bool,
        run: fn(hot_key_manager: &mut HotKeyManager),
    ) -> Script {
        Script {
            name: name.to_string(),
            hot_key,
            block_input,
            condition,
            run,
        }
    }

    pub fn run(&self, hot_key_manager: &mut HotKeyManager) {
        (self.run)(hot_key_manager);
    }

    pub fn condition(&self, hot_key_manager: &mut HotKeyManager) -> bool {
        (self.condition)(hot_key_manager)
    }
}
