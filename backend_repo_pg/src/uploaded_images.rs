use crate::errors::PgRepoError;
use crate::filters::GetAllUploadedImagesFilter;
use crate::insertables::NewUploadedImage;
use crate::models::{db_models, domain};
use crate::options::{PaginationOptions, UploadedImageSortType};
use crate::schema::uploaded_images;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

pub struct UploadedImageRepo<'a> {
    conn: &'a crate::pg_util::RepoConnection,
}

impl<'a> UploadedImageRepo<'a> {
    pub fn new(conn: &'a crate::pg_util::RepoConnection) -> Self {
        Self { conn }
    }

    pub fn insert_one(
        &self,
        new_text_body: NewUploadedImage,
    ) -> Result<domain::UploadedImage, diesel::result::Error> {
        let conn = &self.conn.pg_conn;
        let query = diesel::insert_into(uploaded_images::table).values(&new_text_body);
        let result = query.get_result(conn)?;
        Ok(domain::UploadedImage::from(result))
    }

    pub fn delete_one(&self, id_value: i32) -> Result<usize, diesel::result::Error> {
        use crate::schema::uploaded_images::dsl::{id, uploaded_images};
        let conn = &self.conn.pg_conn;
        let query = diesel::delete(uploaded_images.filter(id.eq(id_value)));
        Ok(query.execute(conn)?)
    }

    pub fn find_one(
        &self,
        id_value: i32,
    ) -> Result<Option<domain::UploadedImage>, diesel::result::Error> {
        use crate::schema::uploaded_images::dsl::{id, uploaded_images};

        let conn = &self.conn.pg_conn;
        let query = uploaded_images
            .filter(id.eq(id_value))
            .select(uploaded_images::all_columns());
        let text_body: db_models::UploadedImage = match query.first(conn).optional()? {
            Some(value) => value,
            None => return Ok(None),
        };
        Ok(Some(domain::UploadedImage::from(text_body)))
    }

    pub fn find(
        &self,
        filter: GetAllUploadedImagesFilter,
        sort: Option<UploadedImageSortType>,
        pagination: PaginationOptions,
    ) -> Result<Vec<domain::UploadedImage>, diesel::result::Error> {
        use crate::schema::uploaded_images::dsl::uploaded_images;
        let q = uploaded_images
            .select(uploaded_images::all_columns())
            .into_boxed();

        let q = if let (Some(page), Some(page_size)) = (pagination.page, pagination.page_size) {
            q.offset((page - 1) * page_size).limit(page_size)
        } else {
            q
        };

        let conn = &self.conn.pg_conn;
        let results: Vec<db_models::UploadedImage> = q.load(conn)?;

        Ok(results
            .into_iter()
            .map(|text_body| domain::UploadedImage::from(text_body))
            .collect::<Vec<_>>())
    }
}
