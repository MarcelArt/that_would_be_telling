use actix_web::{delete, get, post, put, web::{self, Json}, HttpResponse};

use crate::{models::{environment::{Environment, EnvironmentDto}, response::Response}, repos::{self, base::{ICreate, IDelete, IGetById, IRead, IUpdate}}};

#[post("/")]
pub async fn create(repo: web::Data<repos::Combined>, input: Json<EnvironmentDto>) -> HttpResponse {
    let environment = repo.environment.create(input.0).await;

    match environment {
        Err(e) =>  {
            return HttpResponse::InternalServerError().json(Response::<Environment>::failure(e.to_string()));
        },
        Ok(None) => {
            return HttpResponse::InternalServerError().json(Response::<Environment>::failure("Failed to create environment".to_string()));
        },
        Ok(Some(environment)) => {
            return HttpResponse::Created().json(Response::<Environment>::success(Some(environment), "Environment created successfully".to_string()));
        }
    }
}

#[get("/")]
pub async fn read(repo: web::Data<repos::Combined>) -> HttpResponse {
    let environments = repo.environment.read().await;

    match environments {
        Err(e) =>  {
            return HttpResponse::InternalServerError().json(Response::<Vec<Environment>>::failure(e.to_string()));
        },
        Ok(environments) => {
            return HttpResponse::Ok().json(Response::<Vec<Environment>>::success(Some(environments), "Environments retrieved successfully".to_string()));
        }
    }
}

#[put("/{id}")]
pub async fn update(repo: web::Data<repos::Combined>, id: web::Path<String>, input: Json<EnvironmentDto>) -> HttpResponse {
    let environment = repo.environment.update(id.into_inner(), input.0).await;

    match environment {
        Err(e) =>  {
            return HttpResponse::InternalServerError().json(Response::<Environment>::failure(e.to_string()));
        },
        Ok(None) => {
            return HttpResponse::NotFound().json(Response::<Environment>::failure("Environment not found".to_string()));
        },
        Ok(Some(environment)) => {
            return HttpResponse::Ok().json(Response::<Environment>::success(Some(environment), "Environment updated successfully".to_string()));
        }
    }
}

#[delete("/{id}")]
pub async fn delete(repo: web::Data<repos::Combined>, id: web::Path<String>) -> HttpResponse {
    let environment = repo.environment.delete(id.into_inner()).await;

    match environment {
        Err(e) =>  {
            return HttpResponse::InternalServerError().json(Response::<Environment>::failure(e.to_string()));
        },
        Ok(None) => {
            return HttpResponse::NotFound().json(Response::<Environment>::failure("Environment not found".to_string()));
        },
        Ok(Some(environment)) => {
            return HttpResponse::Ok().json(Response::<Environment>::success(Some(environment), "Environment deleted successfully".to_string()));
        }
    }
}

#[get("/{id}")]
pub async fn get_by_id(repo: web::Data<repos::Combined>, id: web::Path<String>) -> HttpResponse {
    let environment = repo.environment.get_by_id(id.into_inner()).await;

    match environment {
        Err(e) =>  {
            return HttpResponse::InternalServerError().json(Response::<Environment>::failure(e.to_string()));
        },
        Ok(None) => {
            return HttpResponse::NotFound().json(Response::<Environment>::failure("Environment not found".to_string()));
        },
        Ok(Some(environment)) => {
            return HttpResponse::Ok().json(Response::<Environment>::success(Some(environment), "Environment retrieved successfully".to_string()));
        }
    }
}