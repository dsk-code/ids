-- Add up migration script here
CREATE TABLE IF NOT EXISTS teachers (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(), -- 主キー
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE, -- 外部キー制約
    last_name VARCHAR(50) NOT NULL,
    first_name VARCHAR(50) NOT NULL,
    last_name_kana VARCHAR(50) NOT NULL CHECK (last_name_kana ~ '^[ァ-ンー]+$'),
    first_name_kana VARCHAR(50) NOT NULL CHECK (first_name_kana ~ '^[ァ-ンー]+$'),
    phone VARCHAR(15),
    mobile_phone VARCHAR(15),
    email VARCHAR(255),
    post_code1 VARCHAR(3) NOT NULL CHECK (post_code1 ~ '^[0-9]{3}$'),
    post_code2 VARCHAR(4) NOT NULL CHECK (post_code2 ~ '^[0-9]{4}$'),
    prefecture VARCHAR(30) NOT NULL, -- 都道府県名（例：東京都）
    city VARCHAR(40) NOT NULL,
    street_address VARCHAR(100) NOT NULL,
    building VARCHAR(50),
    prefectures_kana VARCHAR(50) CHECK (prefectures_kana ~ '^[ァ-ンー]+$'),
    city_kana VARCHAR(50) CHECK (city_kana ~ '^[ァ-ンー]+$'),
    building_kana VARCHAR(50) CHECK (building_kana ~ '^[ァ-ンー]+$'),
    hire_date DATE NOT NULL,
    leave_date DATE,
    status VARCHAR(20) NOT NULL CHECK (status IN ('active', 'inactive')),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, -- 作成日時
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP -- 更新日時
);

CREATE TRIGGER update_teachers_modtime
BEFORE UPDATE ON teachers
FOR EACH ROW
EXECUTE PROCEDURE update_timestamp();