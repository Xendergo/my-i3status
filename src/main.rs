use std::{f32::consts::PI, thread};

use chrono::{Datelike, Local, Timelike, Utc};

const DEG_TO_RAD: f32 = PI / 180.0;

fn main() {
    let latitude_deg: f32 = std::env::args().nth(1).unwrap().parse().unwrap();
    let longitude_deg: f32 = std::env::args().nth(2).unwrap().parse().unwrap();

    let date = Utc::today();
    let winter_solstice = date.with_day(5).unwrap().with_month(12).unwrap();

    let latitude = latitude_deg * DEG_TO_RAD;
    let longitude = longitude_deg * DEG_TO_RAD;

    let axial_tilt = 23.44 * DEG_TO_RAD;

    let earth_position_angle = (date - winter_solstice).num_days() as f32 / 365.25 * 2.0 * PI;

    println!("{{ \"version\": 1 }}");

    println!("[[]");

    loop {
        let time = Local::now();

        let angle_from_noon_estimate =
            Utc::now().time().num_seconds_from_midnight() as f32 / 86400.0 * PI * 2.0
                + PI
                + longitude;

        let mut strings: Vec<String> = vec![];

        for i in 0..48 {
            let color;

            let effective_long = angle_from_noon_estimate + (i as f32) / 24.0 * PI;

            let dot = (axial_tilt * (earth_position_angle + effective_long).cos() + latitude).cos()
                * effective_long.cos();

            if dot > 0.2 {
                    color = day(dot)
            } else if dot < -0.2 {
                color = night(dot)
            } else if dot < 0.0 {
                let range = dot * 5.0 + 1.0;
                let whole_range = dot * 2.5 + 0.5;

                let (_, day_green, day_blue) = day(dot); 
                let (night_red, night_green, night_blue) = night(dot);

                color = (
                    (lerp(range, night_red, 1.0)),
                    (lerp(whole_range, night_green, day_green)),
                    (lerp(whole_range, night_blue, day_blue)),
                )
            } else {
                let range = dot * 5.0;
                let whole_range = dot * 2.5 + 0.5;

                let (day_red, day_green, day_blue) = day(dot); 
                let (_, night_green, night_blue) = night(dot);

                color = (
                    (lerp(range, 1.0, day_red)),
                    (lerp(whole_range, night_green, day_green)),
                    (lerp(whole_range, night_blue, day_blue)),
                )
            }

            let color_text = format!("#{:02X}{:02X}{:02X}", (color.0 * 255.0) as u8, (color.1 * 255.0) as u8, (color.2 * 255.0) as u8);

            strings.push(format!(
                "{{\"background\":\"{}\",\"full_text\":\"  \",\"separator\": false,\"separator_block_width\":0}}",
                color_text
            ))
        }

        strings.push(format!(
            "{{\"full_text\":\"  {} {}  \"}}",
            time.date().format("%y.%m.%d"),
            time.time().format("%H:%M")
        ));

        println!(",[{}]", strings.join(","));

        thread::sleep(std::time::Duration::from_secs(1))
    }
}

fn day(dot: f32) -> (f32, f32, f32) {
    (0.25, lerp(dot, 0.38, 0.75), lerp(dot, 0.5, 1.0))
}

fn night(_dot: f32) -> (f32, f32, f32) {
    (0.125, 0.0625, 0.1875)
}

fn lerp(v: f32, a: f32, b: f32) -> f32 {
    a * (1.0-v) + b * v
}
