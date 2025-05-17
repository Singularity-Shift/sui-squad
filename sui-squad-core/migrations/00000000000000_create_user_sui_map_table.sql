-- migrations/00000000000000_create_user_sui_map_table.sql
CREATE TABLE IF NOT EXISTS user_sui_map (
    telegram_user_id BIGINT NOT NULL,
    telegram_group_id TEXT NOT NULL,
    sui_address TEXT NOT NULL,
    sui_account_object_id TEXT, -- Nullable
    PRIMARY KEY (telegram_user_id, telegram_group_id)
); 