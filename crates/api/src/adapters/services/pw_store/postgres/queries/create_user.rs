pub const CREATE_USER: &str = "
INSERT INTO users(id, username, pw_hash) values ($1, $2, $3) ON CONFLICT DO NOTHING RETURNING id;
";
