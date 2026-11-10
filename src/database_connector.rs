//! Модуль доступу до даних (Data Access Layer).
//! Уся робота з SQLite зосереджена тут: підключення, створення таблиці
//! та CRUD-функції. Логіка веб-застосунку (маршрути) знаходиться в main.rs.

use rusqlite::{Connection, OptionalExtension, Result, params};
use serde::{Deserialize, Serialize};

/// Шлях до файлу бази даних (створюється автоматично).
const DB_PATH: &str = "data.db";

/// SQL-скрипт створення таблиці `books`.
pub const CREATE_TABLE_SQL: &str = "
    CREATE TABLE IF NOT EXISTS books (
        id        INTEGER PRIMARY KEY AUTOINCREMENT,
        title     TEXT    NOT NULL,
        author    TEXT    NOT NULL,
        year      INTEGER NOT NULL,
        genre     TEXT,
        available INTEGER NOT NULL DEFAULT 1
    )";

/// Один запис таблиці `books`.
#[derive(Debug, Clone, Serialize)]
pub struct Book {
    pub id: i64,
    pub title: String,
    pub author: String,
    pub year: i64,
    pub genre: String,
    pub available: bool,
}

/// Дані, що надходять з HTML-форми (без id).
#[derive(Debug, Clone, Deserialize)]
pub struct BookData {
    pub title: String,
    pub author: String,
    pub year: i64,
    pub genre: String,
    /// Чекбокс: присутній у формі лише коли відмічений.
    pub available: Option<String>,
}

/// Встановлює з'єднання з файлом 'data.db'.
pub fn create_connection() -> Result<Connection> {
    Connection::open(DB_PATH)
}

/// Створює таблицю при першому запуску програми.
pub fn create_db_table() -> Result<()> {
    let conn = create_connection()?;
    conn.execute(CREATE_TABLE_SQL, [])?;
    Ok(())
}

/// Перетворює рядок результату SELECT у структуру Book.
fn row_to_book(row: &rusqlite::Row) -> Result<Book> {
    Ok(Book {
        id: row.get(0)?,
        title: row.get(1)?,
        author: row.get(2)?,
        year: row.get(3)?,
        genre: row.get::<_, Option<String>>(4)?.unwrap_or_default(),
        available: row.get::<_, i64>(5)? != 0,
    })
}

/// C (Create): додає новий запис. Повертає id нового запису.
pub fn insert_item(data: &BookData) -> Result<i64> {
    let conn = create_connection()?;
    // Безпечна передача даних через плейсхолдери '?'
    conn.execute(
        "INSERT INTO books (title, author, year, genre, available) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![data.title, data.author, data.year, data.genre, data.available.is_some()],
    )?;
    Ok(conn.last_insert_rowid())
}

/// R (Read All): повертає всі записи.
pub fn get_all_items() -> Result<Vec<Book>> {
    let conn = create_connection()?;
    let mut stmt =
        conn.prepare("SELECT id, title, author, year, genre, available FROM books ORDER BY id")?;
    let items = stmt.query_map([], row_to_book)?.collect::<Result<Vec<_>>>()?;
    Ok(items)
}

/// R (Read One): повертає один запис за id або None, якщо його немає.
pub fn get_item_by_id(id: i64) -> Result<Option<Book>> {
    let conn = create_connection()?;
    conn.query_row(
        "SELECT id, title, author, year, genre, available FROM books WHERE id = ?1",
        params![id],
        row_to_book,
    )
    .optional()
}

/// U (Update): оновлює запис за id. Повертає кількість змінених рядків.
pub fn update_item(id: i64, new_data: &BookData) -> Result<usize> {
    let conn = create_connection()?;
    conn.execute(
        "UPDATE books SET title = ?1, author = ?2, year = ?3, genre = ?4, available = ?5 WHERE id = ?6",
        params![
            new_data.title,
            new_data.author,
            new_data.year,
            new_data.genre,
            new_data.available.is_some(),
            id
        ],
    )
}

/// D (Delete): видаляє запис за id. Повертає кількість видалених рядків.
pub fn delete_item(id: i64) -> Result<usize> {
    let conn = create_connection()?;
    conn.execute("DELETE FROM books WHERE id = ?1", params![id])
}
