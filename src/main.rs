//! Веб-застосунок (аналог app.py з Flask): маршрути та обробка запитів.
//! Доступ до БД виконується лише через модуль database_connector.

mod database_connector;

use axum::{
    Form, Router,
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, post},
};
use database_connector as db;
use minijinja::{Environment, context, value::Serde};
use std::sync::LazyLock;

/// HTML-шаблони з папки templates/ (вбудовуються у програму під час компіляції).
static TEMPLATES: LazyLock<Environment<'static>> = LazyLock::new(|| {
    let mut env = Environment::new();
    env.add_template("base.html", include_str!("../templates/base.html")).unwrap();
    env.add_template("index.html", include_str!("../templates/index.html")).unwrap();
    env.add_template("form.html", include_str!("../templates/form.html")).unwrap();
    env
});

/// Помилка, яку можна повернути з обробника як HTTP-відповідь.
struct AppError(StatusCode, String);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.0, self.1).into_response()
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(e: rusqlite::Error) -> Self {
        AppError(StatusCode::INTERNAL_SERVER_ERROR, format!("Помилка БД: {e}"))
    }
}

impl From<minijinja::Error> for AppError {
    fn from(e: minijinja::Error) -> Self {
        AppError(StatusCode::INTERNAL_SERVER_ERROR, format!("Помилка шаблону: {e}"))
    }
}

type AppResult<T> = Result<T, AppError>;

fn not_found(id: i64) -> AppError {
    AppError(StatusCode::NOT_FOUND, format!("Книгу з id = {id} не знайдено"))
}

/// Read ( / ): головна сторінка з таблицею всіх записів.
async fn index() -> AppResult<Html<String>> {
    let books = db::get_all_items()?;
    let html = TEMPLATES.get_template("index.html")?.render(context! { books => Serde(books) })?;
    Ok(Html(html))
}

/// Create ( GET /add ): форма додавання.
async fn add_form() -> AppResult<Html<String>> {
    let html = TEMPLATES
        .get_template("form.html")?
        .render(context! { heading => "Додати книгу", action => "/add" })?;
    Ok(Html(html))
}

/// Create ( POST /add ): збереження нового запису.
async fn add_submit(Form(data): Form<db::BookData>) -> AppResult<Redirect> {
    db::insert_item(&data)?;
    Ok(Redirect::to("/"))
}

/// Update ( GET /edit/{id} ): форма з даними запису.
async fn edit_form(Path(id): Path<i64>) -> AppResult<Html<String>> {
    let book = db::get_item_by_id(id)?.ok_or_else(|| not_found(id))?;
    let html = TEMPLATES.get_template("form.html")?.render(context! {
        heading => "Редагувати книгу",
        action => format!("/edit/{id}"),
        book => Serde(book),
    })?;
    Ok(Html(html))
}

/// Update ( POST /edit/{id} ): збереження змін.
async fn edit_submit(Path(id): Path<i64>, Form(data): Form<db::BookData>) -> AppResult<Redirect> {
    if db::update_item(id, &data)? == 0 {
        return Err(not_found(id));
    }
    Ok(Redirect::to("/"))
}

/// Delete ( POST /delete/{id} ): видалення та перенаправлення на головну.
async fn delete(Path(id): Path<i64>) -> AppResult<Redirect> {
    if db::delete_item(id)? == 0 {
        return Err(not_found(id));
    }
    Ok(Redirect::to("/"))
}

#[tokio::main]
async fn main() {
    db::create_db_table().expect("не вдалося створити таблицю");

    let app = Router::new()
        .route("/", get(index))
        .route("/add", get(add_form).post(add_submit))
        .route("/edit/{id}", get(edit_form).post(edit_submit))
        .route("/delete/{id}", post(delete));

    let addr = "127.0.0.1:5000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Сервер запущено: http://{addr}/");
    axum::serve(listener, app).await.unwrap();
}
