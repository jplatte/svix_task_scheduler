use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub(crate) struct Task {
    pub id: TaskId,
    #[serde(rename = "type")]
    pub ty: TaskType,
    pub state: TaskState,
    pub start_time: DateTime<Utc>,
}

#[derive(Clone, Copy, Deserialize, Serialize, sqlx::Type)]
#[sqlx(transparent, no_pg_array)]
pub(crate) struct TaskId(pub Uuid);

impl TaskId {
    pub(crate) fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl fmt::Debug for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "task_type")]
pub(crate) enum TaskType {
    Foo,
    Bar,
    Baz,
}

#[derive(Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "task_state")]
pub(crate) enum TaskState {
    Pending,
    Active,
    Failed,
    Done,
}
