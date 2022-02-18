use crate::entity;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, FromSqlRow, AsExpression, TS)]
#[ts(export, export_to = "bindings/misc/AdminLogAction.ts")]
#[sql_type = "AdminLogActionType"]
pub enum AdminLogAction {
    Create,
    Update,
    Delete,
}

#[derive(SqlType)]
#[postgres(type_name = "admin_log_action")]
pub struct AdminLogActionType;

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, FromSqlRow, AsExpression, TS)]
#[ts(export, export_to = "bindings/misc/UserRole.ts")]
#[sql_type = "UserRoleType"]
pub enum UserRole {
    Admin,
    Moderator,
    User,
    Ghost,
}

impl From<entity::sea_orm_active_enums::UserRole> for UserRole {
    fn from(value: entity::sea_orm_active_enums::UserRole) -> Self {
        match value {
            entity::sea_orm_active_enums::UserRole::Admin => UserRole::Admin,
            entity::sea_orm_active_enums::UserRole::Moderator => UserRole::Moderator,
            entity::sea_orm_active_enums::UserRole::User => UserRole::User,
            entity::sea_orm_active_enums::UserRole::Ghost => UserRole::Ghost,
        }
    }
}

impl From<UserRole> for entity::sea_orm_active_enums::UserRole {
    fn from(value: UserRole) -> Self {
        match value {
            UserRole::Admin => entity::sea_orm_active_enums::UserRole::Admin,
            UserRole::Moderator => entity::sea_orm_active_enums::UserRole::Moderator,
            UserRole::User => entity::sea_orm_active_enums::UserRole::User,
            UserRole::Ghost => entity::sea_orm_active_enums::UserRole::Ghost,
        }
    }
}

#[derive(SqlType)]
#[postgres(type_name = "user_role")]
pub struct UserRoleType;

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, FromSqlRow, AsExpression, TS)]
#[ts(export, export_to = "bindings/misc/SearchItemType.ts")]
#[sql_type = "SearchItemTypeType"]
pub enum SearchItemType {
    Project,
    BlogPost,
    Page,
    ExternalLink,
}

#[derive(SqlType)]
#[postgres(type_name = "search_item_type")]
pub struct SearchItemTypeType;

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

impl<Db: Backend> ToSql<AdminLogActionType, Db> for AdminLogAction {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Db>) -> serialize::Result {
        match *self {
            AdminLogAction::Create => out.write_all(b"Create")?,
            AdminLogAction::Update => out.write_all(b"Update")?,
            AdminLogAction::Delete => out.write_all(b"Delete")?,
        }
        Ok(IsNull::No)
    }
}

impl<Db: Backend> ToSql<SearchItemTypeType, Db> for SearchItemType {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Db>) -> serialize::Result {
        match *self {
            SearchItemType::Project => out.write_all(b"Project")?,
            SearchItemType::BlogPost => out.write_all(b"Blog Post")?,
            SearchItemType::Page => out.write_all(b"Page")?,
            SearchItemType::ExternalLink => out.write_all(b"External Link")?,
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

impl FromSql<AdminLogActionType, Pg> for AdminLogAction {
    fn from_sql(bytes: Option<&<Pg as Backend>::RawValue>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"Create" => Ok(AdminLogAction::Create),
            b"Update" => Ok(AdminLogAction::Update),
            b"Delete" => Ok(AdminLogAction::Delete),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl FromSql<SearchItemTypeType, Pg> for SearchItemType {
    fn from_sql(bytes: Option<&<Pg as Backend>::RawValue>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"Project" => Ok(SearchItemType::Project),
            b"Blog Post" => Ok(SearchItemType::BlogPost),
            b"Page" => Ok(SearchItemType::Page),
            b"External Link" => Ok(SearchItemType::ExternalLink),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
