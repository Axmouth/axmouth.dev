use crate::errors::PgRepoError;
use crate::filters::GetAllProjectsFilter;
use crate::models::{db_models, domain};
use crate::options::{PaginationOptions, ProjectSort};
use crate::schema::{projects, projects_technologies, technologies};
use crate::{
    change_sets::UpdateProject,
    insertables::{NewProject, NewProjectTechnology, NewTechnology},
};
use diesel::prelude::*;
use diesel::{r2d2::ConnectionManager, PgConnection, QueryDsl, RunQueryDsl};
use r2d2::Pool;

#[derive(Clone)]
pub struct ProjectRepo {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl ProjectRepo {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { pool }
    }

    pub async fn insert_one(
        &self,
        new_project: NewProject,
    ) -> Result<domain::Project, PgRepoError> {
        let conn = self.pool.get()?;
        let query = diesel::insert_into(projects::table).values(&new_project);
        let result = tokio::task::block_in_place(move || query.get_result(&conn))?;
        Ok(domain::Project::from(result, vec![]))
    }

    async fn update_technologies(
        &self,
        inserted_project_id: i32,
        technologies_list: &Vec<String>,
    ) -> Result<(), PgRepoError> {
        use crate::schema::projects_technologies::dsl::{
            project_id, projects_technologies as projects_technologies_dsl, technology_id,
        };
        use crate::schema::technologies::dsl::{
            name as technology_name, technologies as technologies_dsl,
        };
        let new_technologies: Vec<NewTechnology> = technologies_list
            .clone()
            .into_iter()
            .map(|name| NewTechnology { name })
            .collect();
        let conn = self.pool.get()?;
        let query = diesel::insert_into(technologies::table)
            .values(&new_technologies)
            .on_conflict_do_nothing();
        let _ = tokio::task::block_in_place(move || query.execute(&conn))?;
        let query = technologies_dsl
            .select(technologies_dsl::all_columns())
            .into_boxed();
        let query = query.filter(technology_name.eq_any(technologies_list));
        let conn = self.pool.get()?;
        let inserted_technologies: Vec<db_models::Category> =
            tokio::task::block_in_place(move || query.load(&conn))?;
        let new_projects_technologies: Vec<NewProjectTechnology> = inserted_technologies
            .into_iter()
            .map(|technology| NewProjectTechnology {
                technology_id: technology.id,
                project_id: inserted_project_id,
            })
            .collect();
        let conn = self.pool.get()?;
        let query = diesel::insert_into(projects_technologies::table)
            .values(&new_projects_technologies)
            .on_conflict_do_nothing();
        let _ = tokio::task::block_in_place(move || query.execute(&conn))?;
        Ok(())
    }

    pub async fn insert_one_with_technologies(
        &self,
        new_project: NewProject,
        technologies_list: Vec<String>,
    ) -> Result<domain::Project, PgRepoError> {
        let conn = self.pool.get()?;
        let query = diesel::insert_into(projects::table).values(&new_project);
        let inserted_project: db_models::Project =
            match tokio::task::block_in_place(move || query.get_result(&conn)).optional()? {
                None => {
                    return Err(PgRepoError {
                        error_message: "Failed to insert".to_string(),
                        error_type: crate::errors::PgRepoErrorType::Unknown,
                    })
                }
                Some(value) => value,
            };
        let _ = self
            .update_technologies(inserted_project.id, &technologies_list)
            .await?;
        Ok(domain::Project::from(inserted_project, technologies_list))
    }

    pub async fn update_one(
        &self,
        id_value: i32,
        updated_project: UpdateProject,
    ) -> Result<domain::Project, PgRepoError> {
        use crate::schema::projects::dsl::{id, projects};
        let conn = self.pool.get()?;
        let query = diesel::update(projects.filter(id.eq(id_value))).set(&updated_project);
        let result = tokio::task::block_in_place(move || query.get_result(&conn))?;
        Ok(domain::Project::from(result, vec![]))
    }

    pub async fn update_one_with_technologies(
        &self,
        id_value: i32,
        updated_project: UpdateProject,
        technologies_list: Vec<String>,
    ) -> Result<domain::Project, PgRepoError> {
        let result = self.update_one(id_value, updated_project).await?;
        let _ = self
            .update_technologies(id_value, &technologies_list)
            .await?;
        Ok(result)
    }

    pub async fn delete_one(&self, id_value: i32) -> Result<usize, PgRepoError> {
        use crate::schema::projects::dsl::{id, projects};
        use crate::schema::projects_technologies::dsl::{
            project_id as projects_technologies_project_id,
            projects_technologies as projects_technologies_dsl, technology_id,
        };
        let conn = self.pool.get()?;
        let query = diesel::delete(
            projects_technologies_dsl.filter(projects_technologies_project_id.eq(id_value)),
        );
        tokio::task::block_in_place(move || query.execute(&conn))?;
        let conn = self.pool.get()?;
        let query = diesel::delete(projects.filter(id.eq(id_value)));
        Ok(tokio::task::block_in_place(move || query.execute(&conn))?)
    }

    pub async fn find_one(&self, id_value: i32) -> Result<Option<domain::Project>, PgRepoError> {
        use crate::schema::projects::dsl::{id as project_id, projects};
        use crate::schema::projects_technologies::dsl::{
            project_id as projects_technologies_project_id,
            projects_technologies as projects_technologies_dsl, technology_id,
        };
        use crate::schema::technologies::dsl::{
            name as technology_name, technologies as technologies_dsl,
        };

        let conn = self.pool.get()?;
        let query = projects
            .filter(project_id.eq(id_value))
            .left_join(projects_technologies_dsl.inner_join(technologies_dsl))
            .group_by(project_id)
            .select((
                projects::all_columns(),
                diesel::dsl::sql::<
                    diesel::sql_types::Array<
                        diesel::sql_types::Nullable<diesel::sql_types::VarChar>,
                    >,
                >("array_agg(\"technologies\".\"name\")"),
            ));
        let (project, technologies_list): (db_models::Project, Vec<Option<String>>) =
            match tokio::task::block_in_place(move || query.first(&conn).optional())? {
                Some(value) => value,
                None => return Ok(None),
            };
        Ok(Some(domain::Project::from(
            project,
            technologies_list.into_iter().filter_map(|v| v).collect(),
        )))
    }

    pub async fn find(
        &self,
        filter: GetAllProjectsFilter,
        sort: ProjectSort,
        pagination: PaginationOptions,
    ) -> Result<(Vec<domain::Project>, i64), PgRepoError> {
        use crate::schema::projects::dsl::{
            id as project_id, projects, published as project_published,
        };
        use crate::schema::projects_technologies::dsl::{
            project_id as projects_technologies_project_id,
            projects_technologies as projects_technologies_dsl,
            technology_id as projects_technologies_technology_id,
        };
        use crate::schema::technologies::dsl::{
            name as technology_name, technologies as technologies_dsl,
        };

        let q = projects
            .left_join(projects_technologies_dsl.inner_join(technologies_dsl))
            .group_by(project_id)
            .select((
                projects::all_columns(),
                diesel::dsl::sql::<
                    diesel::sql_types::Array<
                        diesel::sql_types::Nullable<diesel::sql_types::VarChar>,
                    >,
                >("array_agg(\"technologies\".\"name\")"),
                diesel::dsl::sql::<diesel::sql_types::BigInt>("Count(*) Over()"),
            ))
            .into_boxed();

        let q = if let Some(technology_name_filter) = filter.technology_name {
            q.filter(
                project_id.eq_any(
                    projects_technologies_dsl
                        .inner_join(technologies_dsl)
                        .filter(technology_name.eq(technology_name_filter.clone()))
                        .into_boxed()
                        .select(projects_technologies_project_id),
                ),
            )
        } else {
            q
        };

        let q = if let Some(technology_id_filter) = filter.technology_id {
            q.filter(
                project_id.eq_any(
                    projects_technologies_dsl
                        .filter(
                            projects_technologies_technology_id.eq(technology_id_filter.clone()),
                        )
                        .into_boxed()
                        .select(projects_technologies_project_id),
                ),
            )
        } else {
            q
        };

        let q = if let Some(published) = filter.published {
            q.filter(project_published.eq(published))
        } else {
            q
        };

        let q = if let (Some(page), Some(page_size)) = (pagination.page, pagination.page_size) {
            q.offset((page - 1) * page_size).limit(page_size)
        } else {
            q
        };

        let conn = self.pool.get()?;
        let results: Vec<(db_models::Project, Vec<Option<String>>, i64)> =
            tokio::task::block_in_place(move || q.load(&conn))?;

        let count = match results.get(0) {
            Some((_, _, value)) => *value,
            None => 0,
        };

        let projects_list = results
            .into_iter()
            .map(|(project, technologies_list, _)| {
                domain::Project::from(
                    project,
                    technologies_list.into_iter().filter_map(|v| v).collect(),
                )
            })
            .collect::<Vec<_>>();
        Ok((projects_list, count))
    }
}
