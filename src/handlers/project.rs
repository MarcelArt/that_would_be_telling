use actix_web::{delete, get, post, put, web::{self, Json}, HttpMessage, HttpRequest, HttpResponse};
use surrealdb::{RecordId};

use crate::{models::{claims::AccessTokenClaims, project::{Project, ProjectDto}, response::Response}, repos::{self, base::{ICreate, IDelete, IGetById, IRead, IUpdate}}};

#[post("/")]
pub async fn create(repo: web::Data<repos::Combined>, input: Json<ProjectDto>, req: HttpRequest) -> HttpResponse {
    let claims = req.extensions().get::<AccessTokenClaims>().cloned();
    if let None = claims {
        return HttpResponse::Unauthorized().json(Response::<Project>::failure("Unauthorized".to_string()));
    }

    let mut input = input.0;
    let user_id = claims.unwrap().user_id;
    let id_parts: Vec<&str> = user_id.split(":").collect();
    let table_name = id_parts[0];
    let user_id = id_parts[1];
    let user_id = RecordId::from((table_name, user_id));
    input.creator = Some(user_id);

    let project = repo.project.create(input).await;

    match project {
        Err(e) =>  {
            return HttpResponse::InternalServerError().json(Response::<Project>::failure(e.to_string()));
        },
        Ok(None) => {
            return HttpResponse::InternalServerError().json(Response::<Project>::failure("Failed to create project".to_string()));
        },
        Ok(Some(project)) => {
            return HttpResponse::Created().json(Response::<Project>::success(Some(project), "Project created successfully".to_string()));
        }
    }
}

#[get("/")]
pub async fn read(repo: web::Data<repos::Combined>) -> HttpResponse {
    let projects = repo.project.read().await;

    match projects {
        Err(e) =>  {
            return HttpResponse::InternalServerError().json(Response::<Vec<Project>>::failure(e.to_string()));
        },
        Ok(projects) => {
            return HttpResponse::Ok().json(Response::<Vec<Project>>::success(Some(projects), "Projects retrieved successfully".to_string()));
        }
    }
}

#[put("/{id}")]
pub async fn update(repo: web::Data<repos::Combined>, id: web::Path<String>, input: Json<ProjectDto>) -> HttpResponse {
    let project = repo.project.update(id.into_inner(), input.0).await;

    match project {
        Err(e) =>  {
            return HttpResponse::InternalServerError().json(Response::<Project>::failure(e.to_string()));
        },
        Ok(None) => {
            return HttpResponse::NotFound().json(Response::<Project>::failure("Project not found".to_string()));
        },
        Ok(Some(project)) => {
            return HttpResponse::Ok().json(Response::<Project>::success(Some(project), "Project updated successfully".to_string()));
        }
    }
}

#[delete("/{id}")]
pub async fn delete(repo: web::Data<repos::Combined>, id: web::Path<String>) -> HttpResponse {
    let project = repo.project.delete(id.into_inner()).await;

    match project {
        Err(e) =>  {
            return HttpResponse::InternalServerError().json(Response::<Project>::failure(e.to_string()));
        },
        Ok(None) => {
            return HttpResponse::NotFound().json(Response::<Project>::failure("Project not found".to_string()));
        },
        Ok(Some(project)) => {
            return HttpResponse::Ok().json(Response::<Project>::success(Some(project), "Project deleted successfully".to_string()));
        }
    }
}

#[get("/{id}")]
pub async fn get_by_id(repo: web::Data<repos::Combined>, id: web::Path<String>) -> HttpResponse {
    let project = repo.project.get_by_id(id.into_inner()).await;

    match project {
        Err(e) =>  {
            return HttpResponse::InternalServerError().json(Response::<Project>::failure(e.to_string()));
        },
        Ok(None) => {
            return HttpResponse::NotFound().json(Response::<Project>::failure("Project not found".to_string()));
        },
        Ok(Some(project)) => {
            return HttpResponse::Ok().json(Response::<Project>::success(Some(project), "Project retrieved successfully".to_string()));
        }
    }
}