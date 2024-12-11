#![allow(non_snake_case)]
use std::{collections::HashMap, vec};

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use serde::{Deserialize, Serialize};

mod localstorage;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
enum Status {
    #[default]
    Todo,
    InProgress,
    Done,
    Closed,
}

impl Status {
    fn as_str(&self) -> &str {
        match self {
            Status::Todo => "Todo",
            Status::InProgress => "InProgress",
            Status::Done => "Done",
            Status::Closed => "Closed",
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
struct Task {
    id: i32,
    title: String,
    description: Option<String>,
    due_date: Option<String>,
    status: Status,
    priority: Option<i32>,
    created_at: String,
    updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
struct NewTask {
    title: String,
    description: Option<String>,
    due_date: Option<String>,
    #[serde(default)]
    status: Status,
    priority: Option<i32>,
}

#[component]
fn Home() -> Element {
    let mut new_task: Signal<NewTask> = use_signal(|| NewTask::default());
    let mut tasks: Signal<Vec<Task>> = use_signal(|| vec![Default::default()]);

    let on_submit = move |event: Event<FormData>| {
        let values = serde_json::from_str(
            &serde_json::to_string(
                &event
                    .values()
                    .into_iter()
                    .map(|(k, v)| (k, v.as_value()))
                    .collect::<HashMap<_, _>>(),
            )
            .unwrap(),
        );
        info!("submitting from values: {:?}", values);

        let task: NewTask = match values {
            Ok(task) => task,
            Err(e) => {
                info!("failed to parse form values: {:?}", e);
                return;
            }
        };

        let priority = task.priority.unwrap_or(0);
        let new_id = tasks.read().len() as i32 + 1;

        // タスクの追加処理
        tasks.write().push(Task {
            id: new_id,
            title: task.title,
            description: task.description,
            due_date: task.due_date,
            status: task.status, // デフォルト値が設定されているため unwrap は不要
            priority: Some(priority),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        });

        new_task.set(NewTask::default());
    };

    rsx! {
        div {
            h1 { "Todo App" }
            ul {
                h1 { "tasks" }
                for task in tasks.read().iter() {
                    li {
                        p { "{task.title}" }
                        p { "{task.description:?}" }
                        p { "{task.due_date:?}" }
                        p { "{task.status:?}" }
                        p { "{task.priority:?}" }
                        p { "{task.created_at}" }
                        p { "{task.updated_at}" }
                    }
                }
            }

        }
    }
}

#[component]
fn NewTask(on_new_task: EventHandler<NewTask>) -> Element {
    let on_submit = move |event: Event<FormData>| {
        let values = event
            .values()
            .into_iter()
            .map(|(k, v)| (k, v.as_value()))
            .collect::<HashMap<_, _>>();
        info!("submitting from values: {:?}", values);

        let Some(title) = values.get("title").to_owned() else {
            return;
        };
        let description = values.get("description").clone();
        let due_date = values.get("due_date").clone();
        let priority = values.get("priority").and_then(|v| v.parse().ok());

        NewTask {
            title,
            description,
            due_date,
            priority,
            ..Default::default()
        };
    };

    rsx! {
        form { onsubmit: on_submit,
            p { "Title" }
            input { name: "title", r#type: "text" }
            p { "Description" }
            input { name: "description", r#type: "text" }
            p { "Due Date" }
            input { name: "due_date", r#type: "date" }
            p { "Priority" }
            input { name: "priority" , r#type: "number" }
            input { r#type: "submit" }
        }
    }
}
