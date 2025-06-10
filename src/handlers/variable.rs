use actix_web::{delete, get, post, put, web::{self, Json}, HttpResponse};

use crate::{models::{variable::{Variable, VariableDto}, response::Response}, repos::{self, base::{ICreate, IDelete, IGetById, IRead, IUpdate}}};

#[post("/")]
pub async fn create(repo: web::Data<repos::Combined>, input: Json<VariableDto>) -> HttpResponse {
    let variable = repo.variable.create(input.0).await;

    match variable {
        Err(e) =>  {
            return HttpResponse::InternalServerError().json(Response::<Variable>::failure(e.to_string()));
        },
        Ok(None) => {
            return HttpResponse::InternalServerError().json(Response::<Variable>::failure("Failed to create variable".to_string()));
        },
        Ok(Some(variable)) => {
            return HttpResponse::Created().json(Response::<Variable>::success(Some(variable), "Variable created successfully".to_string()));
        }
    }
}

#[get("/")]
pub async fn read(repo: web::Data<repos::Combined>) -> HttpResponse {
    let variables = repo.variable.read().await;

    match variables {
        Err(e) =>  {
            return HttpResponse::InternalServerError().json(Response::<Vec<Variable>>::failure(e.to_string()));
        },
        Ok(variables) => {
            return HttpResponse::Ok().json(Response::<Vec<Variable>>::success(Some(variables), "Variables retrieved successfully".to_string()));
        }
    }
}

#[put("/{id}")]
pub async fn update(repo: web::Data<repos::Combined>, id: web::Path<String>, input: Json<VariableDto>) -> HttpResponse {
    let variable = repo.variable.update(id.into_inner(), input.0).await;

    match variable {
        Err(e) =>  {
            return HttpResponse::InternalServerError().json(Response::<Variable>::failure(e.to_string()));
        },
        Ok(None) => {
            return HttpResponse::NotFound().json(Response::<Variable>::failure("Variable not found".to_string()));
        },
        Ok(Some(variable)) => {
            return HttpResponse::Ok().json(Response::<Variable>::success(Some(variable), "Variable updated successfully".to_string()));
        }
    }
}

#[delete("/{id}")]
pub async fn delete(repo: web::Data<repos::Combined>, id: web::Path<String>) -> HttpResponse {
    let variable = repo.variable.delete(id.into_inner()).await;

    match variable {
        Err(e) =>  {
            return HttpResponse::InternalServerError().json(Response::<Variable>::failure(e.to_string()));
        },
        Ok(None) => {
            return HttpResponse::NotFound().json(Response::<Variable>::failure("Variable not found".to_string()));
        },
        Ok(Some(variable)) => {
            return HttpResponse::Ok().json(Response::<Variable>::success(Some(variable), "Variable deleted successfully".to_string()));
        }
    }
}

#[get("/{id}")]
pub async fn get_by_id(repo: web::Data<repos::Combined>, id: web::Path<String>) -> HttpResponse {
    let variable = repo.variable.get_by_id(id.into_inner()).await;

    match variable {
        Err(e) =>  {
            return HttpResponse::InternalServerError().json(Response::<Variable>::failure(e.to_string()));
        },
        Ok(None) => {
            return HttpResponse::NotFound().json(Response::<Variable>::failure("Variable not found".to_string()));
        },
        Ok(Some(variable)) => {
            return HttpResponse::Ok().json(Response::<Variable>::success(Some(variable), "Variable retrieved successfully".to_string()));
        }
    }
}