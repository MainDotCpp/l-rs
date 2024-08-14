



use app::hotkey::script::Script;
use app::hotkey::manager::{HotKey, HotKeyManager};
use lazy_static::lazy_static;

lazy_static! {
    static ref SCRIPTS: Vec<Script> = Vec::new();
}

static mut HOT_KEY_MANAGER: Option<HotKeyManager> = None;


fn keybd_hook_callback(vk_code: i32, _w_param: i32, _l_param: i32) -> i32 {
    println!("vk_code: {}", vk_code);
    let vk_code = vk_code as u32;
    let hot_key_manager = unsafe { HOT_KEY_MANAGER.as_mut().unwrap() };

    hot_key_manager.hot_keys.push(HotKey::new(vk_code));

    // 定义一个是否阻止输入的标志位
    let mut block_input = false;

    // 遍历所有脚本
    SCRIPTS.iter().for_each(|script| {

        // 判断脚本是否满足条件
        if (script.condition)(hot_key_manager) {

            // 运行脚本
            (script.run)(hot_key_manager);

            // 如果脚本要求阻止输入，那么就阻止输入
            block_input = script.block_input;
        }
    });

    if block_input {
        1
    } else {
        // 调用下一个钩子
        0
    }
}


fn main() {
    // 调用hook.dll 安装钩子
}