
const UPSERT_CREDENTIAL: &str = "
INSERT INTO credentials (id, service, username, password, user_id) VALUES ($1, $2, $3, $4, $5)
ON CONFLICT DO UPDATE SET
    service = EXCLUDED.service,
    username = EXCLUDED.username,
    password = EXCLUDED.password;
";
