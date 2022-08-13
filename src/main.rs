#[macro_use]
extern crate rocket;

use std::{fs::{OpenOptions}, io::{Write}};
use std::io::{BufRead, BufReader};
use rocket::serde::{Deserialize, json::Json};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Task<'r> {
    item: &'r str,
}

#[post("/add-task", data = "<task>")]
fn add_task(task: Json<Task<'_>>) -> &'static str {
    let mut tasks = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("tasks.txt")
        .expect("unable to access tasks.txt");
    let task_item_string = format!("{}\n", task.item);
    let task_item_bytes = task_item_string.as_bytes();
    tasks.write(task_item_bytes).expect("unable to write to tasks.txt");
    "Task added successfully"
}

#[get("/read-tasks")]
fn read_tasks() -> Json<Vec<String>> {
    let tasks = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("tasks.txt")
        .expect("unable to access tasks.txt");
    let reader = BufReader::new(tasks);
    Json(reader.lines()
        .map(|line| line.expect("could not read line"))
        .collect())
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/about")]
fn about() -> &'static str {
    "This is the about page of my Rocket Demo App"
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", routes![index, about, add_task, read_tasks])
        .launch()
        .await
        .expect("panic message");
}

