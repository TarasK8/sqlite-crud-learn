# sqlite-crud-learn

Веб-застосунок «Бібліотека» з повним набором CRUD-операцій над SQLite.
Лабораторна робота №6-7, реалізована на Rust замість Python/Flask.

| У методичці (Python) | Тут (Rust) |
|---|---|
| Flask | [axum](https://crates.io/crates/axum) + tokio |
| модуль `sqlite3` | [rusqlite](https://crates.io/crates/rusqlite) (SQLite вбудовано, `bundled`) |
| шаблони Jinja2 | [minijinja](https://crates.io/crates/minijinja) |
| `app.py` | `src/main.rs` |
| `database_connector.py` | `src/database_connector.rs` |
| `requirements.txt` | `Cargo.toml` |
| `templates/` | `templates/` |

## Запуск

```bash
cargo run
```

Відкрийте http://127.0.0.1:5000/. Файл `data.db` створюється автоматично.

## Маршрути

| Метод | Шлях | Дія |
|---|---|---|
| GET | `/` | список книг (Read All) |
| GET / POST | `/add` | форма та додавання (Create) |
| GET / POST | `/edit/{id}` | форма з даними та збереження (Read One + Update) |
| POST | `/delete/{id}` | видалення та перехід на головну (Delete) |
