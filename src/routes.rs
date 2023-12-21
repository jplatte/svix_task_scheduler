use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, PgPool};

use crate::{
    error::Result,
    model::{Task, TaskId, TaskState, TaskType},
};

pub(crate) fn v1_routes() -> Router<PgPool> {
    Router::new()
        .route("/task", get(list_tasks).post(create_task))
        .route("/task/:task_id", get(show_task).delete(delete_task))
}

#[derive(Deserialize)]
struct ListParams {
    #[serde(rename = "type")]
    ty: TaskType,
    state: TaskState,
}

#[tracing::instrument(skip_all)]
async fn list_tasks(
    State(db_pool): State<PgPool>,
    params: Query<ListParams>,
) -> Result<Json<Vec<Task>>> {
    let tasks = query_as!(
        Task,
        r#"
        SELECT "id" as "id: _", "type" as "ty: _", "state" as "state: _", "start_time"
        FROM "task"
        WHERE ($1::task_type IS NULL OR "type" = $1)
            AND ($2::task_state IS NULL OR "state" = $2)
        "#,
        &params.ty as _,
        &params.state as _,
    )
    .fetch_all(&db_pool)
    .await?;

    Ok(Json(tasks))
}

#[derive(Deserialize)]
struct CreateTaskParams {
    #[serde(rename = "type")]
    ty: TaskType,
    start_time: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
struct CreateTaskResponse {
    task_id: TaskId,
}

#[tracing::instrument(skip_all)]
async fn create_task(
    State(db_pool): State<PgPool>,
    Json(params): Json<CreateTaskParams>,
) -> Result<impl IntoResponse> {
    let task_id = TaskId::new();
    query!(
        r#"
        INSERT INTO "task" ("id", "type", "state", "start_time")
        VALUES ($1, $2, 'Pending', $3)
        "#,
        task_id as _,
        params.ty as _,
        params.start_time.unwrap_or_else(Utc::now),
    )
    .execute(&db_pool)
    .await?;

    Ok((StatusCode::CREATED, Json(CreateTaskResponse { task_id })))
}

#[tracing::instrument(skip_all)]
async fn show_task(
    State(db_pool): State<PgPool>,
    Path(task_id): Path<TaskId>,
) -> Result<Json<Task>> {
    let task = query_as!(
        Task,
        r#"
        SELECT "id" as "id: _", "type" as "ty: _", "state" as "state: _", "start_time"
        FROM "task"
        WHERE "id" = $1
        "#,
        task_id as _,
    )
    .fetch_one(&db_pool)
    .await?;

    Ok(Json(task))
}

#[tracing::instrument(skip_all)]
async fn delete_task(
    State(db_pool): State<PgPool>,
    Path(task_id): Path<TaskId>,
) -> Result<StatusCode> {
    query!(r#"DELETE FROM "task" WHERE "id" = $1"#, task_id as _).execute(&db_pool).await?;
    Ok(StatusCode::NO_CONTENT)
}
