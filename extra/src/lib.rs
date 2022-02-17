pub trait PaginatedQueryExt {
    fn paginated_query(&self) -> Pagination;
}

pub struct Pagination {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}
