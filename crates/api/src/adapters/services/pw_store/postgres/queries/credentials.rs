pub const UPSERT_CREDENTIAL: &str = "
INSERT INTO credentials (id, service, username, password, service_index, user_id) VALUES ($1, $2, $3, $4, $5, $6)
ON CONFLICT (id) DO UPDATE SET
    service = EXCLUDED.service,
    username = EXCLUDED.username,
    password = EXCLUDED.password,
    service_index = EXCLUDED.service_index;
";
