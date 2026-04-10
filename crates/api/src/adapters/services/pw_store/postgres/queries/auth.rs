use crate::ports::services::pw_store::AuthCredentials;
use sqlx::postgres::PgRow;
use sqlx::{Error, FromRow, Row};

pub const GET_AUTH_CREDENTIALS: &str =
    "SELECT id, username, pw_hash FROM authentication where username = $1;";

impl FromRow<'_, PgRow> for AuthCredentials {
    fn from_row(row: &'_ PgRow) -> Result<Self, Error> {
        Ok(AuthCredentials {
            id: row.try_get("id")?,
            username: row.try_get("username")?,
            pw_hash: row.try_get("pw_hash")?,
        })
    }
}
