use std::sync::Arc;
use crate::{models::project::{Project, ProjectDto}, repos::base::{ICreate, IRead, IUpdate, IDelete, IGetById}};
use surrealdb::{engine::remote::ws::Client, RecordId, Surreal};

const PROJECTS: &str = "projects";

#[derive(Clone, IUpdate, IDelete)]
#[entity("Project")]
#[dto("ProjectDto")]
pub struct Repo {
    db: Arc<Surreal<Client>>,
    table_name: String,
}

impl Repo {
    pub fn new(db: Arc<Surreal<Client>>) -> Self {
        Self {
            db,
            table_name: PROJECTS.to_string(),
        }
    }
    
}

impl ICreate<Project, ProjectDto> for Repo {
    async fn create(&self, input: ProjectDto) -> Result<Option<Project>, crate::error::Error> {
        let query = "
            insert into projects (name, creator)
            values
            ($name, $creator)
        ";

        let user = self.db.query(query)
            .bind(("name", input.name))
            .bind(("creator", input.creator))
            .await?
            .take(0)?;

        Ok(user)
    }
}

impl IRead<Project> for Repo {
    async fn read(&self) -> Result<Vec<Project>, crate::error::Error> {
        let query = "
            select 
                *, 
                creator.* as creator_detail
            from projects
        ";

        let projects = self.db.query(query).await?.take(0)?;

        Ok(projects)
    }
}


impl IGetById<Project> for Repo { 
    async fn get_by_id(&self, id: String) -> Result<Option<Project>, crate::error::Error> {
        let id = RecordId::from((PROJECTS, id));
        let query = "
            select 
                *, 
                creator.* as creator_detail
            from $id
        ";

        let project = self.db.query(query)
            .bind(("id", id))
            .await?
            .take(0)?;

        Ok(project)
    }
}