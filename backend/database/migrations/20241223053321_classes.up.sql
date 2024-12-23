-- Add up migration script here
CREATE TABLE IF NOT EXISTS classes (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(), -- 主キー
    user_id UUID REFERENCES users(id) ON DELETE CASCADE, -- 外部キー制約
    class_name TEXT NOT NULL, -- クラス名
    is_active BOOLEAN DEFAULT TRUE, -- アクティブ状態
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, -- 作成日時
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP -- 更新日時
);

CREATE TRIGGER update_classes_modtime
BEFORE UPDATE ON classes
FOR EACH ROW
EXECUTE PROCEDURE update_timestamp();