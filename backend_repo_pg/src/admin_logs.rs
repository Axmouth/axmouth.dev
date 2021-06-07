use crate::errors::PgRepoError;
use crate::filters::GetAllAdminLogsFilter;
use crate::insertables::NewAdminLog;
use crate::models::{db_models, domain};
use crate::options::{AdminLogSortType, PaginationOptions};
use crate::schema::admin_logs;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

pub struct AdminLogRepo<'a> {
    conn: &'a crate::pg_util::RepoConnection,
}

impl<'a> AdminLogRepo<'a> {
    pub fn new(conn: &'a crate::pg_util::RepoConnection) -> Self {
        Self { conn }
    }

    pub fn insert_one(
        &self,
        new_admin_log: NewAdminLog,
    ) -> Result<domain::AdminLog, diesel::result::Error> {
        let conn = &self.conn.pg_conn;
        let query = diesel::insert_into(admin_logs::table).values(&new_admin_log);
        let result = query.get_result(conn)?;
        Ok(domain::AdminLog::from(result))
    }

    pub fn delete_one(&self, id_value: i32) -> Result<usize, diesel::result::Error> {
        use crate::schema::admin_logs::dsl::{admin_logs, id};
        let conn = &self.conn.pg_conn;
        let query = diesel::delete(admin_logs.filter(id.eq(id_value)));
        Ok(query.execute(conn)?)
    }

    pub fn find_one(
        &self,
        id_value: i32,
    ) -> Result<Option<domain::AdminLog>, diesel::result::Error> {
        use crate::schema::admin_logs::dsl::{admin_logs, id};

        let conn = &self.conn.pg_conn;
        let query = admin_logs
            .filter(id.eq(id_value))
            .select(admin_logs::all_columns());
        let admin_log: db_models::AdminLog = match query.first(conn).optional()? {
            Some(value) => value,
            None => return Ok(None),
        };
        Ok(Some(domain::AdminLog::from(admin_log)))
    }

    pub fn find(
        &self,
        filter: GetAllAdminLogsFilter,
        sort: Option<AdminLogSortType>,
        pagination: PaginationOptions,
    ) -> Result<Vec<domain::AdminLog>, diesel::result::Error> {
        use crate::schema::admin_logs::dsl::admin_logs as admin_logs_dsl;
        let q = admin_logs_dsl
            .select(admin_logs_dsl::all_columns())
            .into_boxed();

        let q = if let (Some(page), Some(page_size)) = (pagination.page, pagination.page_size) {
            q.offset((page - 1) * page_size).limit(page_size)
        } else {
            q
        };

        let q = if let Some(sort_type) = sort {
            match sort_type {
                AdminLogSortType::ActionTimeAsc => q.order(admin_logs::action_time.asc()),
                AdminLogSortType::ActionTimeDesc => q.order(admin_logs::action_time.desc()),
            }
        } else {
            q
        };

        let conn = &self.conn.pg_conn;
        let results: Vec<db_models::AdminLog> = q.load(conn)?;

        Ok(results
            .into_iter()
            .map(|admin_log| domain::AdminLog::from(admin_log))
            .collect::<Vec<_>>())
    }
}
