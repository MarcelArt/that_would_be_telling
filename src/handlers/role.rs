use actix_web::{delete, get, post, put, web::{self, Json}, HttpResponse};

use crate::{models::{role::{Role, RoleDto}, response::Response}, repos::{self, base::{ICreate, IDelete, IGetById, IRead, IUpdate}}};

#[post("/")]
pub async fn create(repo: web::Data<repos::Combined>, input: Json<RoleDto>) -> HttpResponse {
    let role = repo.role.create(input.0).await;

    match role {
        Err(e) =>  {
            return HttpResponse::InternalServerError().json(Response::<Role>::failure(e.to_string()));
        },
        Ok(None) => {
            return HttpResponse::InternalServerError().json(Response::<Role>::failure("Failed to create role".to_string()));
        },
        Ok(Some(role)) => {
            return HttpResponse::Created().json(Response::<Role>::success(Some(role), "Role created successfully".to_string()));
        }
    }
}

#[get("/")]
pub async fn read(repo: web::Data<repos::Combined>) -> HttpResponse {
    let roles = repo.role.read().await;

    match roles {
        Err(e) =>  {
            return HttpResponse::InternalServerError().json(Response::<Vec<Role>>::failure(e.to_string()));
        },
        Ok(roles) => {
            return HttpResponse::Ok().json(Response::<Vec<Role>>::success(Some(roles), "Roles retrieved successfully".to_string()));
        }
    }
}

#[put("/{id}")]
pub async fn update(repo: web::Data<repos::Combined>, id: web::Path<String>, input: Json<RoleDto>) -> HttpResponse {
    let role = repo.role.update(id.into_inner(), input.0).await;

    match role {
        Err(e) =>  {
            return HttpResponse::InternalServerError().json(Response::<Role>::failure(e.to_string()));
        },
        Ok(None) => {
            return HttpResponse::NotFound().json(Response::<Role>::failure("Role not found".to_string()));
        },
        Ok(Some(role)) => {
            return HttpResponse::Ok().json(Response::<Role>::success(Some(role), "Role updated successfully".to_string()));
        }
    }
}

#[delete("/{id}")]
pub async fn delete(repo: web::Data<repos::Combined>, id: web::Path<String>) -> HttpResponse {
    let role = repo.role.delete(id.into_inner()).await;

    match role {
        Err(e) =>  {
            return HttpResponse::InternalServerError().json(Response::<Role>::failure(e.to_string()));
        },
        Ok(None) => {
            return HttpResponse::NotFound().json(Response::<Role>::failure("Role not found".to_string()));
        },
        Ok(Some(role)) => {
            return HttpResponse::Ok().json(Response::<Role>::success(Some(role), "Role deleted successfully".to_string()));
        }
    }
}

#[get("/{id}")]
pub async fn get_by_id(repo: web::Data<repos::Combined>, id: web::Path<String>) -> HttpResponse {
    let role = repo.role.get_by_id(id.into_inner()).await;

    match role {
        Err(e) =>  {
            return HttpResponse::InternalServerError().json(Response::<Role>::failure(e.to_string()));
        },
        Ok(None) => {
            return HttpResponse::NotFound().json(Response::<Role>::failure("Role not found".to_string()));
        },
        Ok(Some(role)) => {
            return HttpResponse::Ok().json(Response::<Role>::success(Some(role), "Role retrieved successfully".to_string()));
        }
    }
}