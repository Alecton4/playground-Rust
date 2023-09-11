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

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct TaskUpdate<'r> {
    id: u8,
    item: &'r str,
}

#[post("/addtask", data = "<task>")]
fn add_task(task: Json<Task<'_>>) -> &'static str {
    let mut tasks = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("tasks.txt")
        .expect("Unable to access tasks.txt!!!");
    let reader = BufReader::new(&tasks);
    let id = reader.lines().count();
    let task_item_string = format!("{},{}\n", id, task.item);
    let task_item_bytes = task_item_string.as_bytes();
    tasks
        .write_all(task_item_bytes)
        .expect("Unable to write to tasks.txt!!!");
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
            .map(|line| {
                let line_string: String = line.expect("Could not read line!!!");
                let line_pieces: Vec<&str> = line_string.split(',').collect();
                line_pieces[1].to_string()
            })
            .collect(),
    )
}

#[put("/edittask", data = "<task_update>")]
fn edit_task(task_update: Json<TaskUpdate<'_>>) -> &'static str {
    let tasks = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("tasks.txt")
        .expect("Unable to access tasks.txt!!!");
    let mut temp = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("temp.txt")
        .expect("Unable to access temp.txt!!!");

    let reader = BufReader::new(tasks);
    for line in reader.lines() {
        let line_string: String = line.expect("Could not read line!!!");
        let line_pieces: Vec<&str> = line_string.split(',').collect();

        if line_pieces[0]
            .parse::<u8>()
            .expect("Unable to parse id as u8!!!")
            == task_update.id
        {
            let task_items: [&str; 2] = [line_pieces[0], task_update.item];
            let task = format!("{}\n", task_items.join(","));
            temp.write_all(task.as_bytes())
                .expect("Could not write to temp file!!!");
        } else {
            let task = format!("{}\n", line_string);
            temp.write_all(task.as_bytes())
                .expect("Could not write to temp file!!!");
        }
    }

    std::fs::remove_file("tasks.txt").expect("Unable to remove tasks.txt!!!");
    std::fs::rename("temp.txt", "tasks.txt").expect("Unable to rename temp.txt!!!");
    "Task updated successfully"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, add_task, read_tasks, edit_task])
}
