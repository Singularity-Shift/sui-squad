-- migrations/00000000000001_create_sui_groups_map_table.sql
CREATE TABLE IF NOT EXISTS sui_groups_map (
    telegram_group_id TEXT PRIMARY KEY NOT NULL,
    sui_group_object_id TEXT NOT NULL
);
