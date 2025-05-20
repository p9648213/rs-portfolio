CREATE TYPE "RealEstateRole" AS ENUM ('Manager', 'Tenant');

-- CREATE TABLE
CREATE TABLE IF NOT EXISTS users (
    id VARCHAR(255) PRIMARY KEY NOT NULL,
    account_id VARCHAR(255) UNIQUE,
    username VARCHAR(255),
    email VARCHAR(255) UNIQUE,
    password VARCHAR(255),
    image_url VARCHAR(255),
    provider VARCHAR(20),
    phone_number VARCHAR(20),
    real_estate_role "RealEstateRole" NOT NULL DEFAULT 'Tenant',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

--SAMPLE DATA
INSERT INTO users (
    id, account_id, username, email, password,
    image_url, provider, phone_number, real_estate_role,
    created_at, updated_at
) VALUES (
    '7ff4109f-de50-490f-989e-db3518368c3e', NULL, 'admin', 'admin@gmail.com',
    '$argon2id$v=19$m=19456,t=2,p=1$QlBlrAxWGFxs8tE4PdhsTA$IlVULEH4LOJWntbs5mqN5DpKJp+0vLizqvIP20bCRGw',
    NULL, NULL, NULL, 'Manager',
    '2025-05-20 14:21:26.653135', '2025-05-20 14:23:41.188395'
);

INSERT INTO users (
    id, account_id, username, email, password,
    image_url, provider, phone_number, real_estate_role,
    created_at, updated_at
) VALUES (
    'd1ecac15-8fa2-4d1c-9c15-abbbb2bcedf5', NULL, 'user01', 'user01@gmail.com',
    '$argon2id$v=19$m=19456,t=2,p=1$YlsJk8N5a+da0n8vZEZfLg$w9WYIUjG+r6av86BD+5yUGXuXe+6sxD9pzsWYPkZODA',
    NULL, NULL, NULL, 'Tenant',
    '2025-05-20 14:21:38.379273', '2025-05-20 14:23:41.188395'
);

-- FUNCTION
CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- TRIGGER
DO
$$
DECLARE
    tbl RECORD;
BEGIN
    FOR tbl IN
        SELECT table_name
        FROM information_schema.columns
        WHERE table_schema = 'public' 
          AND column_name = 'updated_at'
    LOOP
        -- Drop the trigger if it already exists to avoid duplication
        EXECUTE format('
            DROP TRIGGER IF EXISTS trigger_update_timestamp ON %I
        ', tbl.table_name);

        -- Create the trigger to update `updated_at` on each update
        EXECUTE format('
            CREATE TRIGGER trigger_update_timestamp
            BEFORE UPDATE ON %I
            FOR EACH ROW
            EXECUTE FUNCTION update_timestamp()
        ', tbl.table_name);
    END LOOP;
END
$$;
