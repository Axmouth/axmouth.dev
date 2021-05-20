use crate::filters::GetAllUploadedImagesFilter;
use crate::insertables::NewUploadedImage;
use crate::models::{db_models, domain};
use crate::options::{PaginationOptions, UploadedImageSortType};
use crate::schema::uploaded_images;
use crate::{errors::PgRepoError, pg_util::Repo};
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, PgConnection, QueryDsl, RunQueryDsl};
use r2d2::Pool;

#[derive(Clone)]
pub struct UploadedImageRepo {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl UploadedImageRepo {
    pub fn new(repo: Repo) -> Self {
        Self { pool: repo.pool }
    }

    pub async fn insert_one(
        &self,
        new_text_body: NewUploadedImage,
    ) -> Result<domain::UploadedImage, PgRepoError> {
        let conn = self.pool.get()?;
        let query = diesel::insert_into(uploaded_images::table).values(&new_text_body);
        let result = tokio::task::block_in_place(move || query.get_result(&conn))?;
        Ok(domain::UploadedImage::from(result))
    }

    pub async fn delete_one(&self, id_value: i32) -> Result<usize, PgRepoError> {
        use crate::schema::uploaded_images::dsl::{id, uploaded_images};
        let conn = self.pool.get()?;
        let query = diesel::delete(uploaded_images.filter(id.eq(id_value)));
        Ok(query.execute(&conn)?)
    }

    pub async fn find_one(
        &self,
        id_value: i32,
    ) -> Result<Option<domain::UploadedImage>, PgRepoError> {
        use crate::schema::uploaded_images::dsl::{id, uploaded_images};

        let conn = self.pool.get()?;
        let query = uploaded_images
            .filter(id.eq(id_value))
            .select(uploaded_images::all_columns());
        let text_body: db_models::UploadedImage =
            match tokio::task::block_in_place(move || query.first(&conn).optional())? {
                Some(value) => value,
                None => return Ok(None),
            };
        Ok(Some(domain::UploadedImage::from(text_body)))
    }

    pub async fn find(
        &self,
        filter: GetAllUploadedImagesFilter,
        sort: Option<UploadedImageSortType>,
        pagination: PaginationOptions,
    ) -> Result<Vec<domain::UploadedImage>, PgRepoError> {
        use crate::schema::uploaded_images::dsl::uploaded_images;
        let q = uploaded_images
            .select(uploaded_images::all_columns())
            .into_boxed();

        let q = if let (Some(page), Some(page_size)) = (pagination.page, pagination.page_size) {
            q.offset((page - 1) * page_size).limit(page_size)
        } else {
            q
        };

        let conn = self.pool.get()?;
        let results: Vec<db_models::UploadedImage> =
            tokio::task::block_in_place(move || q.load(&conn))?;

        Ok(results
            .into_iter()
            .map(|text_body| domain::UploadedImage::from(text_body))
            .collect::<Vec<_>>())
    }
}
