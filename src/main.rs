use std::{thread, time::Duration};

use chrono::{TimeDelta, Timelike};
use hyprland::{data::Workspace, shared::HyprDataActive};
use widgets::{pixel_util::Vector2, widget::{make_connection, Anchor, Font, Layer, WidgetBuilder}};



fn main() {
    let wrk_icons = vec!['', '󰖟', '', '', ''];
    let conn = make_connection();
    let mut widget = WidgetBuilder::new(&conn, 1920, 50)
        .layer(Layer::Background)
        .anchor(Anchor::Top)
        .exclusive_edge(Anchor::Top)
        .exclusive_zone(30)
        .build();

    let data = include_bytes!("../font.ttf");
    let font = Font::try_from_bytes(data).expect("Failed to parse font");

    widget.create_surface("status_bar".into()).unwrap();


    
    let mut time = chrono::Local::now().checked_sub_signed(TimeDelta::minutes(2)).unwrap();
    let mut wrk = -1;

    let mut comp = widget.get_comp().unwrap();


    loop {
        while (chrono::Local::now().minute() == time.minute()) && (Workspace::get_active().unwrap().id - 1 == wrk) {
            thread::sleep(Duration::from_millis(500));
        }
        time = chrono::Local::now();
        wrk = Workspace::get_active().unwrap().id - 1;
        let text = format!("{} --- {}",time.format("%H:%M, %d %b %Y"), wrk_icons.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(" "));
        comp.draw_rect(0x7F000000.into(), Vector2::new(0.0,0.0), Vector2::new(1920.0, 30.0)).unwrap();
        comp.draw_text(text.to_string(), Vector2::new(0.0,25.0), 30.0, font.clone(), |_,c| {
            if c == wrk_icons[wrk as usize] {
                return 0xFF00FFFF.into()
            }
            0xFFFFFFFF.into()
        }, 0x7F000000.into()).unwrap();

        println!("Updating! {}", text);
        comp.update_blocking().unwrap();
    }
}
