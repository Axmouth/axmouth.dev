use crate::errors::PgRepoError;
use crate::filters::GetAllAdminLogsFilter;
use crate::insertables::NewAdminLog;
use crate::models::{db_models, domain};
use crate::options::{AdminLogSort, PaginationOptions};
use crate::schema::admin_logs;
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, PgConnection, QueryDsl, RunQueryDsl};
use r2d2::Pool;

#[derive(Clone)]
pub struct AdminLogRepo {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl AdminLogRepo {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }

    pub async fn insert_one(
        &self,
        new_admin_log: NewAdminLog,
    ) -> Result<domain::AdminLog, PgRepoError> {
        let conn = self.pool.get()?;
        let query = diesel::insert_into(admin_logs::table).values(&new_admin_log);
        let result = tokio::task::block_in_place(move || query.get_result(&conn))?;
        Ok(domain::AdminLog::from(result))
    }

    pub async fn delete_one(&self, id_value: i32) -> Result<usize, PgRepoError> {
        use crate::schema::admin_logs::dsl::{admin_logs, id};
        let conn = self.pool.get()?;
        let query = diesel::delete(admin_logs.filter(id.eq(id_value)));
        Ok(query.execute(&conn)?)
    }

    pub async fn find_one(&self, id_value: i32) -> Result<Option<domain::AdminLog>, PgRepoError> {
        use crate::schema::admin_logs::dsl::{admin_logs, id};

        let conn = self.pool.get()?;
        let query = admin_logs
            .filter(id.eq(id_value))
            .select(admin_logs::all_columns());
        let admin_log: db_models::AdminLog =
            match tokio::task::block_in_place(move || query.first(&conn).optional())? {
                Some(value) => value,
                None => return Ok(None),
            };
        Ok(Some(domain::AdminLog::from(admin_log)))
    }

    pub async fn find(
        &self,
        filter: GetAllAdminLogsFilter,
        sort: AdminLogSort,
        pagination: PaginationOptions,
    ) -> Result<Vec<domain::AdminLog>, PgRepoError> {
        use crate::schema::admin_logs::dsl::{admin_logs, id};
        let q = admin_logs.select(admin_logs::all_columns()).into_boxed();

        let q = if let (Some(page), Some(page_size)) = (pagination.page, pagination.page_size) {
            q.offset((page - 1) * page_size).limit(page_size)
        } else {
            q
        };

        let conn = self.pool.get()?;
        let results: Vec<db_models::AdminLog> = tokio::task::block_in_place(move || q.load(&conn))?;

        Ok(results
            .into_iter()
            .map(|admin_log| domain::AdminLog::from(admin_log))
            .collect::<Vec<_>>())
    }
}
