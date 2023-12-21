CREATE TYPE "task_type" AS ENUM ('Foo', 'Bar', 'Baz');
CREATE TYPE "task_state" AS ENUM ('Pending', 'Active', 'Failed', 'Done');

CREATE TABLE "task" (
    "id" UUID PRIMARY KEY,
    "type" task_type NOT NULL,
    "state" task_state NOT NULL,
    "start_time" TIMESTAMPTZ NOT NULL
);
