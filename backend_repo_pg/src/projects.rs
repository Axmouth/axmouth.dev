use crate::errors::PgRepoError;
use crate::filters::GetAllProjectsFilter;
use crate::models::{db_models, domain};
use crate::options::{PaginationOptions, ProjectSortType};
use crate::schema::{projects, projects_technologies, technologies};
use crate::{
    change_sets::UpdateProject,
    insertables::{NewProject, NewProjectTechnology, NewTechnology},
};
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

pub struct ProjectRepo<'a> {
    conn: &'a crate::pg_util::RepoConnection,
}

impl<'a> ProjectRepo<'a> {
    pub fn new(conn: &'a crate::pg_util::RepoConnection) -> Self {
        Self { conn }
    }

    pub fn insert_one(
        &self,
        new_project: NewProject,
    ) -> Result<domain::Project, diesel::result::Error> {
        let conn = &self.conn.pg_conn;
        let query = diesel::insert_into(projects::table).values(&new_project);
        let result = query.get_result(conn)?;
        Ok(domain::Project::from(result, vec![]))
    }

    fn update_technologies(
        &self,
        inserted_project_id: i32,
        technologies_list: &Vec<String>,
    ) -> Result<(), diesel::result::Error> {
        use crate::schema::projects_technologies::dsl::{
            project_id, projects_technologies as projects_technologies_dsl, technology_id,
        };
        use crate::schema::technologies::dsl::{
            name as technology_name, technologies as technologies_dsl,
        };
        let query =
            diesel::delete(projects_technologies::table).filter(project_id.eq(inserted_project_id));
        let conn = &self.conn.pg_conn;
        let _ = query.execute(conn)?;
        let new_technologies: Vec<NewTechnology> = technologies_list
            .clone()
            .into_iter()
            .map(|name| NewTechnology { name })
            .collect();
        let conn = &self.conn.pg_conn;
        let query = diesel::insert_into(technologies::table)
            .values(&new_technologies)
            .on_conflict_do_nothing();
        let _ = query.execute(conn)?;
        let query = technologies_dsl
            .select(technologies_dsl::all_columns())
            .into_boxed();
        let query = query.filter(technology_name.eq_any(technologies_list));
        let conn = &self.conn.pg_conn;
        let inserted_technologies: Vec<db_models::Category> = query.load(conn)?;
        let new_projects_technologies: Vec<NewProjectTechnology> = inserted_technologies
            .into_iter()
            .map(|technology| NewProjectTechnology {
                technology_id: technology.id,
                project_id: inserted_project_id,
            })
            .collect();
        let conn = &self.conn.pg_conn;
        let query = diesel::insert_into(projects_technologies::table)
            .values(&new_projects_technologies)
            .on_conflict_do_nothing();
        let _ = query.execute(conn)?;
        Ok(())
    }

    pub fn insert_one_with_technologies(
        &self,
        new_project: &NewProject,
        technologies_list: Vec<String>,
    ) -> Result<domain::Project, diesel::result::Error> {
        let conn = &self.conn.pg_conn;
        let query = diesel::insert_into(projects::table).values(new_project);
        let inserted_project: db_models::Project = match query.get_result(conn).optional()? {
            None => return Err(diesel::result::Error::__Nonexhaustive),
            Some(value) => value,
        };
        let _ = self.update_technologies(inserted_project.id, &technologies_list)?;
        Ok(domain::Project::from(inserted_project, technologies_list))
    }

    pub fn update_one(
        &self,
        id_value: i32,
        updated_project: &UpdateProject,
    ) -> Result<domain::Project, diesel::result::Error> {
        use crate::schema::projects::dsl::{id, projects};
        let conn = &self.conn.pg_conn;
        let query = diesel::update(projects.filter(id.eq(id_value))).set(updated_project);
        let result = query.get_result(conn)?;
        Ok(domain::Project::from(result, vec![]))
    }

    pub fn update_one_with_technologies(
        &self,
        id_value: i32,
        updated_project: &UpdateProject,
        technologies_list: Vec<String>,
    ) -> Result<domain::Project, diesel::result::Error> {
        let result = self.update_one(id_value, updated_project)?;
        let _ = self.update_technologies(id_value, &technologies_list)?;
        Ok(result)
    }

    pub fn delete_one(&self, id_value: i32) -> Result<usize, diesel::result::Error> {
        use crate::schema::projects::dsl::{id, projects};
        use crate::schema::projects_technologies::dsl::{
            project_id as projects_technologies_project_id,
            projects_technologies as projects_technologies_dsl, technology_id,
        };
        let conn = &self.conn.pg_conn;
        let query = diesel::delete(
            projects_technologies_dsl.filter(projects_technologies_project_id.eq(id_value)),
        );
        query.execute(conn)?;
        let conn = &self.conn.pg_conn;
        let query = diesel::delete(projects.filter(id.eq(id_value)));
        Ok(query.execute(conn)?)
    }

    pub fn find_one(
        &self,
        id_value: i32,
    ) -> Result<Option<domain::Project>, diesel::result::Error> {
        use crate::schema::projects::dsl::{id as project_id, projects};
        use crate::schema::projects_technologies::dsl::{
            project_id as projects_technologies_project_id,
            projects_technologies as projects_technologies_dsl, technology_id,
        };
        use crate::schema::technologies::dsl::{
            name as technology_name, technologies as technologies_dsl,
        };

        let conn = &self.conn.pg_conn;
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
            match query.first(conn).optional()? {
                Some(value) => value,
                None => return Ok(None),
            };
        Ok(Some(domain::Project::from(
            project,
            technologies_list.into_iter().filter_map(|v| v).collect(),
        )))
    }

    pub fn find_one_by_slug(
        &self,
        slug_value: String,
    ) -> Result<Option<domain::Project>, diesel::result::Error> {
        use crate::schema::projects::dsl::{id as project_id, projects, slug};
        use crate::schema::projects_technologies::dsl::{
            project_id as projects_technologies_project_id,
            projects_technologies as projects_technologies_dsl, technology_id,
        };
        use crate::schema::technologies::dsl::{
            name as technology_name, technologies as technologies_dsl,
        };

        let conn = &self.conn.pg_conn;
        let query = projects
            .filter(slug.eq(slug_value))
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
            match query.first(conn).optional()? {
                Some(value) => value,
                None => return Ok(None),
            };
        Ok(Some(domain::Project::from(
            project,
            technologies_list.into_iter().filter_map(|v| v).collect(),
        )))
    }

    pub fn find(
        &self,
        filter: GetAllProjectsFilter,
        sort: Option<ProjectSortType>,
        pagination: PaginationOptions,
    ) -> Result<(Vec<domain::Project>, i64), diesel::result::Error> {
        use crate::schema::projects::dsl::{
            id as project_id, projects as projects_dsl, published as project_published,
        };
        use crate::schema::projects_technologies::dsl::{
            project_id as projects_technologies_project_id,
            projects_technologies as projects_technologies_dsl,
            technology_id as projects_technologies_technology_id,
        };
        use crate::schema::technologies::dsl::{
            name as technology_name, technologies as technologies_dsl,
        };

        let q = projects_dsl
            .left_join(projects_technologies_dsl.inner_join(technologies_dsl))
            .group_by(project_id)
            .select((
                projects_dsl::all_columns(),
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

        let q = if let Some(sort_type) = sort {
            match sort_type {
                ProjectSortType::CreatedAtAsc => q.order(projects::created_at.asc()),
                ProjectSortType::CreatedAtDesc => q.order(projects::created_at.desc()),
                ProjectSortType::NameAsc => q.order(projects::name.asc()),
                ProjectSortType::NameDesc => q.order(projects::name.desc()),
            }
        } else {
            q
        };

        let q = if let (Some(page), Some(page_size)) = (pagination.page, pagination.page_size) {
            q.offset((page - 1) * page_size).limit(page_size)
        } else {
            q
        };

        let conn = &self.conn.pg_conn;
        let results: Vec<(db_models::Project, Vec<Option<String>>, i64)> = q.load(conn)?;

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
