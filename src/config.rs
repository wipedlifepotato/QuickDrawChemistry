use macroquad::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    window: WindowConfig,
}

#[derive(Debug, Deserialize)]
pub struct WindowConfig {
    width: u32,
    height: u32,
    fullscreen: bool,
}
fn read_config(conf_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let s = std::fs::read_to_string(conf_path)?;
    let config: Config = toml::from_str(&s)?;
    //println!("config: {:?}", config);
    Ok(config)    
}
pub fn window_conf() -> Conf {
    const PROGRAM_NAME: &str = "QuickChemDraw";
    let mut w: u32 = 800;
    let mut h: u32 = 600;
    let mut fullscreen = false;
    match read_config("config.toml") {
        Err(err) => {
            eprintln!("{}", err);
            println!("Use default settings");
        }, 
        Ok(cfg) => {
            w = cfg.window.width;
            h = cfg.window.height;
            fullscreen = cfg.window.fullscreen;
        }
    }
    Conf {
        window_title: PROGRAM_NAME.to_owned(),
        window_width: w as i32,
        window_height: h as i32,
        fullscreen: fullscreen, 
        ..Default::default()
    }
}
