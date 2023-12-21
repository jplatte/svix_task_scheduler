use std::time::Duration;

use rand::{thread_rng, Rng};
use sqlx::{query, Executor, PgPool, Postgres};
use tracing::error;

use crate::model::{TaskId, TaskState, TaskType};

pub(crate) async fn run_tasks(db_pool: PgPool) {
    const WAIT_FOR_NEW_TASK_DURATION: Duration = Duration::from_secs(10);
    const WAIT_ON_TASK_ERROR_DURATION: Duration = Duration::from_secs(30);

    loop {
        match run_single_task(&db_pool).await {
            Ok(true) => {}
            Ok(false) => {
                tokio::time::sleep(WAIT_FOR_NEW_TASK_DURATION).await;
            }
            Err(e) => {
                error!("Failed to run task: {e}");
                tokio::time::sleep(WAIT_ON_TASK_ERROR_DURATION).await;
            }
        }
    }
}

/// Attempt to run a single task.
///
/// Returns `Err(_)` if there was a problem with the database.\
/// Returns `Ok(true)` if a task was successfully run.\
/// Returns `Ok(false)` if there was no task to run.
async fn run_single_task(db_pool: &PgPool) -> sqlx::Result<bool> {
    let Some((task_type, task_id)) = get_new_task(db_pool).await? else {
        return Ok(false);
    };

    let task_state = match task_type {
        TaskType::Foo => run_foo(task_id).await,
        TaskType::Bar => run_bar().await,
        TaskType::Baz => run_baz().await,
    };

    // TODO: How to handle failure here?
    set_task_state(task_id, task_state, db_pool).await?;

    Ok(true)
}

async fn get_new_task(db_pool: &PgPool) -> sqlx::Result<Option<(TaskType, TaskId)>> {
    let mut db_conn = db_pool.acquire().await?;

    // Fetch a single task, making sure to receive unique access to it.
    //
    // TODO: Also fetch failed tasks to retry them?
    let res = query!(
        r#"
        SELECT "id" as "id: TaskId", "type" as "ty: TaskType" FROM "task"
        WHERE "state" = 'Pending' AND "start_time" <= NOW()
        LIMIT 1
        FOR NO KEY UPDATE SKIP LOCKED
        "#
    )
    .fetch_optional(&mut *db_conn)
    .await?;
    let Some(row) = res else {
        return Ok(None);
    };

    set_task_state(row.id, TaskState::Active, &mut *db_conn).await?;

    Ok(Some((row.ty, row.id)))
}

async fn set_task_state<'c, E: Executor<'c, Database = Postgres>>(
    task_id: TaskId,
    task_state: TaskState,
    db: E,
) -> sqlx::Result<()> {
    query!(r#"UPDATE "task" SET "state" = $2 WHERE "id" = $1"#, task_id as _, task_state as _)
        .execute(db)
        .await?;
    Ok(())
}

async fn run_foo(task_id: TaskId) -> TaskState {
    tokio::time::sleep(Duration::from_secs(3)).await;
    println!("Foo {task_id:?}");
    TaskState::Done
}

async fn run_bar() -> TaskState {
    match reqwest::get("https://www.whattimeisitrightnow.com/").await {
        Ok(response) => {
            println!("{}", response.status());
            TaskState::Done
        }
        Err(e) => {
            error!("Bar task failed: {e}");
            TaskState::Failed
        }
    }
}

async fn run_baz() -> TaskState {
    let mut rng = thread_rng();
    println!("Baz {}", rng.gen_range(0..=343));
    TaskState::Done
}
