use actix_web::{delete, get, post, put, web::{self, Json}, HttpResponse};

use crate::{models::{permission::{Permission, PermissionDto}, response::Response}, repos::{self, base::{ICreate, IDelete, IGetById, IRead, IUpdate}}};

#[post("/")]
pub async fn create(repo: web::Data<repos::Combined>, input: Json<PermissionDto>) -> HttpResponse {
    let permission = repo.permission.create(input.0).await;

    match permission {
        Err(e) =>  {
            return HttpResponse::InternalServerError().json(Response::<Permission>::failure(e.to_string()));
        },
        Ok(None) => {
            return HttpResponse::InternalServerError().json(Response::<Permission>::failure("Failed to create permission".to_string()));
        },
        Ok(Some(permission)) => {
            return HttpResponse::Created().json(Response::<Permission>::success(Some(permission), "Permission created successfully".to_string()));
        }
    }
}

#[get("/")]
pub async fn read(repo: web::Data<repos::Combined>) -> HttpResponse {
    let permissions = repo.permission.read().await;

    match permissions {
        Err(e) =>  {
            return HttpResponse::InternalServerError().json(Response::<Vec<Permission>>::failure(e.to_string()));
        },
        Ok(permissions) => {
            return HttpResponse::Ok().json(Response::<Vec<Permission>>::success(Some(permissions), "Permissions retrieved successfully".to_string()));
        }
    }
}

#[put("/{id}")]
pub async fn update(repo: web::Data<repos::Combined>, id: web::Path<String>, input: Json<PermissionDto>) -> HttpResponse {
    let permission = repo.permission.update(id.into_inner(), input.0).await;

    match permission {
        Err(e) =>  {
            return HttpResponse::InternalServerError().json(Response::<Permission>::failure(e.to_string()));
        },
        Ok(None) => {
            return HttpResponse::NotFound().json(Response::<Permission>::failure("Permission not found".to_string()));
        },
        Ok(Some(permission)) => {
            return HttpResponse::Ok().json(Response::<Permission>::success(Some(permission), "Permission updated successfully".to_string()));
        }
    }
}

#[delete("/{id}")]
pub async fn delete(repo: web::Data<repos::Combined>, id: web::Path<String>) -> HttpResponse {
    let permission = repo.permission.delete(id.into_inner()).await;

    match permission {
        Err(e) =>  {
            return HttpResponse::InternalServerError().json(Response::<Permission>::failure(e.to_string()));
        },
        Ok(None) => {
            return HttpResponse::NotFound().json(Response::<Permission>::failure("Permission not found".to_string()));
        },
        Ok(Some(permission)) => {
            return HttpResponse::Ok().json(Response::<Permission>::success(Some(permission), "Permission deleted successfully".to_string()));
        }
    }
}

#[get("/{id}")]
pub async fn get_by_id(repo: web::Data<repos::Combined>, id: web::Path<String>) -> HttpResponse {
    let permission = repo.permission.get_by_id(id.into_inner()).await;

    match permission {
        Err(e) =>  {
            return HttpResponse::InternalServerError().json(Response::<Permission>::failure(e.to_string()));
        },
        Ok(None) => {
            return HttpResponse::NotFound().json(Response::<Permission>::failure("Permission not found".to_string()));
        },
        Ok(Some(permission)) => {
            return HttpResponse::Ok().json(Response::<Permission>::success(Some(permission), "Permission retrieved successfully".to_string()));
        }
    }
}