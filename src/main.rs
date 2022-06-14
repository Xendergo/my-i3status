use std::{
    f32::consts::PI,
    fs,
    io,
    sync::{ Arc, RwLock},
    thread,
};

use chrono::{ Datelike, Local, NaiveDate, Timelike, Utc};
use home::home_dir;
use serde::{Deserialize, Serialize};
use serde_json::json;

const DEG_TO_RAD: f32 = PI / 180.0;
const AXIAL_TILT: f32 = 23.44 * DEG_TO_RAD;

fn main() {
    println!(
        "{}",
        json!({
            "version": 1,
            "click_events": true
        })
        .to_string()
    );

    println!("[[]");

    let todos: Arc<RwLock<Todos>> = Arc::new(RwLock::new(
        serde_json::from_str(
            fs::read_to_string(
                home_dir()
                    .expect("Couldn't find a home directory")
                    .join(".i3")
                    .join("todos.json"),
            )
            .as_ref()
            .map::<&str, _>(|v| v)
            .unwrap_or("{\"todos\": {}}"),
        )
        .expect("Couldn't parse todos.json file"),
    ));

    let click_evt_todos = Arc::clone(&todos);

    thread::spawn(move || loop {
        let mut click_event_json = String::new();

        if let Err(_) = io::stdin().read_line(&mut click_event_json) {
            continue;
        }

        let click_event =
            match serde_json::from_str::<ClickEvent>(&click_event_json.trim_start_matches(",")) {
                Ok(v) => v,
                Err(_) => continue,
            };

        if click_event.name == "todo" {
            let mut todos = click_evt_todos.write().unwrap();

            if click_event.instance.parse::<usize>().unwrap() < todos.todos.len() {
                let todo = &mut todos.todos[click_event.instance.parse::<usize>().unwrap()];

                if let Some(done_today) = todo.done_today {
                    if Local::today().naive_local() != done_today {
                        todo.last_completed = todo.done_today.take();
                    }
                }

                match todo.done_today {
                    Some(_) => todo.done_today = None,
                    None => todo.done_today = Some(Local::today().naive_local()),
                }

                write_todos(&todos);

                update_bar(&todos);
            }
        }
    });

    loop {
        let todos = todos.read().unwrap();

        update_bar(&todos);

        drop(todos);

        thread::sleep(std::time::Duration::from_secs(60));
    }
}

fn write_todos(todos: &Todos) {
    fs::write(
        home_dir()
            .expect("Couldn't find home directory")
            .join(".i3")
            .join("todos.json"),
        serde_json::to_string_pretty(todos).expect("Couldn't convert todos to a json string"),
    )
    .expect("Couldn't write todos.json file");
}

#[derive(Debug, Serialize, Deserialize)]
struct Todos {
    todos: Vec<Todo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    name: String,
    color: String,
    interval: i64,
    done_today: Option<NaiveDate>,
    last_completed: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
struct ClickEvent {
    name: String,
    instance: String,
}

fn update_bar(todos: &Todos) {
    let mut strings: Vec<String> = vec![];

    for (i, todo) in todos.todos.iter().enumerate() {
        strings.push(json!({
            "background": match todo.done_today {
                Some(done_today) => if Local::today().naive_local() == done_today {&todo.color} else {"#000000"},
                None => "#000000",
            },
            "border": match todo.last_completed {
                Some(last_completed) => if (Local::today().naive_local() - last_completed).num_days() < todo.interval {&todo.color} else {"#000000"},
                None => "#000000",
            },
            "full_text": format!("  {}  ", &todo.name),
            "name": "todo",
            "instance": i.to_string(),
            "separator": false,
        }).to_string());
    }

    let latitude_deg: f32 = std::env::args().nth(1).unwrap().parse().unwrap();
    let longitude_deg: f32 = std::env::args().nth(2).unwrap().parse().unwrap();

    let date = Utc::today();
    let winter_solstice = date.with_day(5).unwrap().with_month(12).unwrap();

    let latitude = latitude_deg * DEG_TO_RAD;
    let longitude = longitude_deg * DEG_TO_RAD;

    let earth_position_angle = (date - winter_solstice).num_days() as f32 / 365.25 * 2.0 * PI;

    let time = Local::now();

    let angle_from_noon_estimate =
        Utc::now().time().num_seconds_from_midnight() as f32 / 86400.0 * PI * 2.0 + PI + longitude;

    for i in 0..48 {
        let color;

        let effective_long = angle_from_noon_estimate + (i as f32) / 24.0 * PI;

        let dot = (AXIAL_TILT * (earth_position_angle + effective_long).cos() + latitude).cos()
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

        let color_text = format!(
            "#{:02X}{:02X}{:02X}",
            (color.0 * 255.0) as u8,
            (color.1 * 255.0) as u8,
            (color.2 * 255.0) as u8
        );

        strings.push(
            json!({
                "background": color_text,
                "full_text": "  ",
                "separator": false,
                "separator_block_width": 0,
                "name": "sunlight-block",
                "instance": i.to_string(),
            })
            .to_string(),
        );
    }

    strings.push(json!({
        "full_text": format!("  {} {}  ", time.date().format("%y.%m.%d"), time.time().format("%H:%M")),
        "name": "time",
        "instance": "time",
    }).to_string());

    println!(",[{}]", strings.join(","));
}

fn day(dot: f32) -> (f32, f32, f32) {
    (0.25, lerp(dot, 0.38, 0.75), lerp(dot, 0.5, 1.0))
}

fn night(_dot: f32) -> (f32, f32, f32) {
    (0.125, 0.0625, 0.1875)
}

fn lerp(v: f32, a: f32, b: f32) -> f32 {
    a * (1.0 - v) + b * v
}
