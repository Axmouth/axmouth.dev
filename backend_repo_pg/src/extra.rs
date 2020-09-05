use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, FromSqlRow, AsExpression)]
#[sql_type = "UserRoleType"]
pub enum UserRole {
    Admin,
    Moderator,
    User,
    Ghost,
}

#[derive(SqlType)]
#[postgres(type_name = "user_role")]
pub struct UserRoleType;

use std::io::Write;

use diesel::backend::Backend;
use diesel::serialize::{self, IsNull, Output, ToSql};

impl<Db: Backend> ToSql<UserRoleType, Db> for UserRole {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Db>) -> serialize::Result {
        match *self {
            UserRole::Admin => out.write_all(b"Admin")?,
            UserRole::Moderator => out.write_all(b"Moderator")?,
            UserRole::User => out.write_all(b"User")?,
            UserRole::Ghost => out.write_all(b"Ghost")?,
        }
        Ok(IsNull::No)
    }
}

use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;

impl FromSql<UserRoleType, Pg> for UserRole {
    fn from_sql(bytes: Option<&<Pg as Backend>::RawValue>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"Admin" => Ok(UserRole::Admin),
            b"Moderator" => Ok(UserRole::Moderator),
            b"User" => Ok(UserRole::User),
            b"Ghost" => Ok(UserRole::Ghost),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
