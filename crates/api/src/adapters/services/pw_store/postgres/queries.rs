pub const CREATE_USER: &str = r#"
INSERT INTO authentication(id, username, pw_hash) values ($1, $2, $3) ON CONFLICT DO NOTHING RETURNING id;
"#;
