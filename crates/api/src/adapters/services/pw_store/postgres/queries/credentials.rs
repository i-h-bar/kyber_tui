use crate::ports::services::pw_store::CredentialOut;
use sqlx::postgres::PgRow;
use sqlx::{Error, FromRow, Row};

pub const UPSERT_CREDENTIAL: &str = "
INSERT INTO credentials (id, service, username, password, service_index, user_id) VALUES ($1, $2, $3, $4, $5, $6)
ON CONFLICT (id) DO UPDATE SET
    service = EXCLUDED.service,
    username = EXCLUDED.username,
    password = EXCLUDED.password,
    service_index = EXCLUDED.service_index;
";

pub const GET_CREDENTIAL: &str = "
select c.id as id,
       c.password as password,
       c.service as service,
       c.username as username,
       array_agg(n.content)
filter ( where n.content is not null ) as notes from credentials as c
         join notes as n on c.id = n.credential_id
where service_index = $1 and user_id = $2
group by c.id, c.password, c.service, c.username
limit 1;
";

impl FromRow<'_, PgRow> for CredentialOut {
    fn from_row(row: &'_ PgRow) -> Result<Self, Error> {
        let notes: Vec<Vec<u8>> = row.try_get("notes")?;
        let notes = if notes.is_empty() { None } else { Some(notes) };

        Ok(CredentialOut {
            id: row.try_get("id")?,
            service: row.try_get("service")?,
            username: row.try_get("username")?,
            password: row.try_get("password")?,
            notes,
        })
    }
}
