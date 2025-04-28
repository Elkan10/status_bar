use core::str;
use std::{fs, thread, time::Duration};

use chrono::{TimeDelta, Timelike};
use font_loader::system_fonts::{get, FontPropertyBuilder};
use hyprland::{data::Workspace, shared::HyprDataActive};
use serde::Deserialize;
use widgets::{pixel_util::Vector2, widget::{make_connection, Anchor, Font, Layer, WidgetBuilder}};

#[derive(Debug, Deserialize)]
struct Config {
    icons: Vec<char>,
    width: u32,
    height: u32,
    exclusive: u32,
    cpu_label: String,
    text_size: u32,
    text_height: u32,
    bg_color: String,
    text_color: String,
    selected_color: String,
    font: String,
}


fn main() {
    let path = expanduser::expanduser("~/.config/status_bar/config.json")
        .expect("Error finding config file (are you running as a user?)");
    dbg!(&path);
    let json = fs::read(path).expect("Config file not found");
    let config: Config = serde_json::from_str(str::from_utf8(&json).unwrap())
        .expect("Failed to parse config.json");
    
    let wrk_icons = config.icons;
    let conn = make_connection();
    let mut widget = WidgetBuilder::new(&conn, config.width, config.height)
        .layer(Layer::Background)
        .anchor(Anchor::Top)
        .exclusive_edge(Anchor::Top)
        .exclusive_zone(config.exclusive as i32)
        .build();

    let prop = FontPropertyBuilder::new().family(&config.font).monospace().build();
    let data = get(&prop)
        .expect("Could not find font!")
        .0;
    let font = Font::try_from_bytes(&data)
        .expect("Could not load font data");

    widget.create_surface("status_bar".into()).unwrap();

    let bg_color = u32::from_str_radix(&config.bg_color, 16).unwrap();
    let text_color = u32::from_str_radix(&config.text_color, 16).unwrap();
    let selected_color = u32::from_str_radix(&config.selected_color, 16).unwrap();
    
    let mut time = chrono::Local::now().checked_sub_signed(TimeDelta::minutes(2)).unwrap();
    let mut wrk = -1;
    let mut temp = 0.0;

    let mut comp = widget.get_comp().unwrap();

    let mut sys = sysinfo::Components::new_with_refreshed_list();

    let cpu = sys.iter_mut().find(|x| x.label() == config.cpu_label).unwrap();

    loop {
        let mut t = cpu.temperature().unwrap();
        while (chrono::Local::now().minute() == time.minute()) && (Workspace::get_active().unwrap().id - 1 == wrk) && (temp >= t - 1.0 && temp <= t + 1.0) {
            thread::sleep(Duration::from_millis(500));
            cpu.refresh();
            t = cpu.temperature().unwrap();
        }
        time = chrono::Local::now();
        wrk = Workspace::get_active().unwrap().id - 1;
        temp = t;
        let text = format!("{} --- {} | CPU: {} °C",time.format("%H:%M, %d %b %Y"), wrk_icons.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(" "), temp.round());
        comp.draw_rect(bg_color.into(), Vector2::new(0.0,0.0), Vector2::new(config.width as f32, config.height as f32)).unwrap();
        comp.draw_text(text.to_string(), Vector2::new(0.0,config.text_height as f32), config.text_size as f32, font.clone(), |_,c| {
            if c == wrk_icons[wrk as usize] {
                return selected_color.into()
            }
            text_color.into()
        },bg_color.into()).unwrap();

        println!("Updating! {}", text);
        comp.update_blocking().unwrap();
    }
}
