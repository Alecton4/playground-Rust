#[macro_use]
extern crate rocket;

mod pool;

use rocket::fairing::{self, AdHoc};
use rocket::form::Form;
use rocket::fs::relative;
use rocket::fs::FileServer;
use rocket::http::Status;
use rocket::request::FlashMessage;
use rocket::response::{self, Flash, Redirect, Responder};
use rocket::serde::json::Json;
use rocket::{Build, Request, Rocket};
use rocket_dyn_templates::serde::json::json;
use rocket_dyn_templates::Template;
use sea_orm::{ActiveModelTrait, DeleteResult, EntityTrait, QueryOrder, Set};
use sea_orm_rocket::{Connection, Database};

use entity::tasks;
use entity::tasks::Entity as Tasks;
use migration::MigratorTrait;
use pool::Db;

struct DatabaseError(sea_orm::DbErr);

impl<'r> Responder<'r, 'r> for DatabaseError {
    fn respond_to(self, _request: &Request) -> response::Result<'r> {
        Err(Status::InternalServerError)
    }
}

impl From<sea_orm::DbErr> for DatabaseError {
    fn from(error: sea_orm::DbErr) -> Self {
        DatabaseError(error)
    }
}

#[post("/addtask", data = "<task_form>")]
async fn add_task(conn: Connection<'_, Db>, task_form: Form<tasks::Model>) -> Flash<Redirect> {
    let db = conn.into_inner();
    let task = task_form.into_inner();

    let active_task: tasks::ActiveModel = tasks::ActiveModel {
        item: Set(task.item),
        ..Default::default()
    };

    let _result = match active_task.insert(db).await {
        Ok(result) => result,
        Err(_) => {
            return Flash::error(Redirect::to("/"), "Issue creating the task");
        }
    };

    Flash::success(Redirect::to("/"), "Task created!")
}

#[get("/readtasks")]
async fn read_tasks(conn: Connection<'_, Db>) -> Result<Json<Vec<tasks::Model>>, DatabaseError> {
    let db = conn.into_inner();

    Ok(Json(
        Tasks::find()
            .order_by_asc(tasks::Column::Id)
            .all(db)
            .await?,
    ))
}

#[put("/edittask", data = "<task_form>")]
async fn edit_task(conn: Connection<'_, Db>, task_form: Form<tasks::Model>) -> Flash<Redirect> {
    let db = conn.into_inner();
    let task = task_form.into_inner();

    let task_to_update = match Tasks::find_by_id(task.id).one(db).await {
        Ok(result) => result,
        Err(_) => {
            return Flash::error(Redirect::to("/"), "Issue editing the task");
        }
    };
    let mut task_to_update: tasks::ActiveModel = task_to_update.unwrap().into();
    task_to_update.item = Set(task.item);
    let _result = match task_to_update.update(db).await {
        Ok(result) => result,
        Err(_) => {
            return Flash::error(Redirect::to("/"), "Issue editing the task");
        }
    };

    Flash::success(Redirect::to("/"), "Task edited successfully!")
}

#[get("/edit/<id>")]
async fn edit_task_page(conn: Connection<'_, Db>, id: i32) -> Result<Template, DatabaseError> {
    let db = conn.into_inner();
    let task = Tasks::find_by_id(id).one(db).await?.unwrap();

    Ok(Template::render(
        "edit_task_form",
        json!({
            "task": task
        }),
    ))
}

#[delete("/deletetask/<id>")]
async fn delete_task(conn: Connection<'_, Db>, id: i32) -> Flash<Redirect> {
    let db = conn.into_inner();
    let _result = match Tasks::delete_by_id(id).exec(db).await {
        Ok(value) => value,
        Err(_) => {
            return Flash::error(Redirect::to("/"), "Issue deleting the task");
        }
    };

    Flash::success(Redirect::to("/"), "Task successfully deleted!")
}

#[get("/")]
async fn index(
    conn: Connection<'_, Db>,
    flash: Option<FlashMessage<'_>>,
) -> Result<Template, DatabaseError> {
    let db = conn.into_inner();
    let tasks = Tasks::find()
        .order_by_asc(tasks::Column::Id)
        .all(db)
        .await?;

    Ok(Template::render(
        "todo_list",
        json!({
            "tasks": tasks,
            "flash": flash.map(FlashMessage::into_inner)
        }),
    ))
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount("/", FileServer::from(relative!("/public")))
        .mount(
            "/",
            routes![
                index,
                add_task,
                read_tasks,
                edit_task,
                edit_task_page,
                delete_task
            ],
        )
        .attach(Template::fairing())
}
