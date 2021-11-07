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

    let earth_position_angle = (date - winter_solstice).num_days() as f32 / 365.0 * 2.0 * PI;

    println!("{{ \"version\": 1 }}");

    println!("[[]");

    loop {
        let time = Local::now();

        let angle_from_midnight_estimate =
            Utc::now().time().num_seconds_from_midnight() as f32 / 86400.0 * PI * 2.0;

        let mut strings: Vec<String> = vec![];

        for i in 0..48 {
            let color;

            let effective_long = longitude + angle_from_midnight_estimate + (i as f32) / 24.0 * PI;
            let (lat_sin, lat_cos) = latitude.sin_cos();

            let (axial_shift_sin, axial_shift_cos) =
                (axial_tilt * (effective_long * lat_cos + earth_position_angle).cos()).sin_cos();

            let x = lat_cos * effective_long.cos();
            let y = lat_sin;

            let dot = x * axial_shift_cos + y * axial_shift_sin;

            if dot > 0.2 {
                color = (32, 16, 48)
            } else if dot < -0.2 {
                color = (64, 192, 255)
            } else if dot < 0.0 {
                let range = dot * 5.0 + 1.0;
                let whole_range = dot * 2.5 + 0.5;

                color = (
                    ((range * 0.75 + 0.25) * 255.0) as u8,
                    ((0.75 - whole_range * 0.6875) * 255.0) as u8,
                    ((1.0 - whole_range * 0.875) * 255.0) as u8,
                )
            } else {
                let range = dot * 5.0;
                let whole_range = dot * 2.5 + 0.5;

                color = (
                    (((1.0 - range) * 0.875 + 0.125) * 255.0) as u8,
                    ((0.75 - whole_range * 0.6875) * 255.0) as u8,
                    ((1.0 - whole_range * 0.875) * 255.0) as u8,
                )
            }

            let color_text = format!("#{:02X}{:02X}{:02X}", color.0, color.1, color.2);

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
