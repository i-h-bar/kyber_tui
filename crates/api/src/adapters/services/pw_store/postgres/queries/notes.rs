pub const UPSERT_NOTE: &str = "
    INSERT INTO notes (id, credential_id, content) VALUES ($1, $2, $3)
    ON CONFLICT (id) DO UPDATE SET
        content = EXCLUDED.content;
";
