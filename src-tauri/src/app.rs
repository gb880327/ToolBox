use dirs::config_dir;

pub fn init() {
    let config_path = if cfg!(debug_assertions) {
        config_dir().unwrap().join("toolbox_text.db")
    } else {
        config_dir().unwrap().join("toolbox.db")
    };
    println!("{:?}", config_path);
}