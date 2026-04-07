pub const CREATE_USER: &str = r#"
INSERT INTO authentication(username, pw_hash) values ($1, $2) ON CONFLICT DO NOTHING RETURNING id;
"#;
