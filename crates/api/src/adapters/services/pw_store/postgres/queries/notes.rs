
const UPSERT_NOTE: &str = "
    INSERT INTO notes (credential_id, content) VALUES ($1, $2)
    ON CONFLICT DO UPDATE SET
        content = EXCLUDED.content;
";
