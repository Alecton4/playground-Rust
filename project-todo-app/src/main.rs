#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::{BufRead, Write};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Task<'r> {
    item: &'r str,
}

#[post("/addtask", data = "<task>")]
fn add_task(task: Json<Task<'_>>) -> &'static str {
    let mut tasks = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("tasks.txt")
        .expect("unable to access tasks.txt");
    let task_item_string = format!("{}\n", task.item);
    let task_item_bytes = task_item_string.as_bytes();
    tasks
        .write_all(task_item_bytes)
        .expect("unable to write to tasks.txt");
    "Task added successfully"
}

#[get("/readtasks")]
fn read_tasks() -> Json<Vec<String>> {
    let tasks = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("tasks.txt")
        .expect("Unable to access tasks.txt!!!");
    let reader = BufReader::new(tasks);
    Json(
        reader
            .lines()
            .map(|line| line.expect("Could not read line!!!"))
            .collect(),
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, add_task, read_tasks])
}
