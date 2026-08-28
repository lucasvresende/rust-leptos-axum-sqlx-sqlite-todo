use super::Todo;
use leptos::prelude::*;
use server_fn::ServerFnError;
use sqlx::SqlitePool;

fn database_error(error: sqlx::Error) -> ServerFnError {
    ServerFnError::ServerError(error.to_string())
}

fn pool() -> Result<SqlitePool, ServerFnError> {
    use_context::<SqlitePool>().ok_or_else(|| {
        ServerFnError::ServerError("database pool missing from server context".into())
    })
}

pub async fn list() -> Result<Vec<Todo>, ServerFnError> {
    sqlx::query_as::<_, Todo>("SELECT id, title, completed FROM todos ORDER BY id DESC")
        .fetch_all(&pool()?)
        .await
        .map_err(database_error)
}

pub async fn create(title: String) -> Result<(), ServerFnError> {
    let title = title.trim();
    if title.is_empty() {
        return Err(ServerFnError::ServerError(
            "todo title cannot be empty".into(),
        ));
    }

    sqlx::query("INSERT INTO todos (title) VALUES (?1)")
        .bind(title)
        .execute(&pool()?)
        .await
        .map_err(database_error)?;

    Ok(())
}

pub async fn toggle(id: i64) -> Result<(), ServerFnError> {
    sqlx::query(
        "UPDATE todos \
         SET completed = CASE completed WHEN 0 THEN 1 ELSE 0 END \
         WHERE id = ?1",
    )
    .bind(id)
    .execute(&pool()?)
    .await
    .map_err(database_error)?;

    Ok(())
}

pub async fn delete(id: i64) -> Result<(), ServerFnError> {
    sqlx::query("DELETE FROM todos WHERE id = ?1")
        .bind(id)
        .execute(&pool()?)
        .await
        .map_err(database_error)?;

    Ok(())
}
