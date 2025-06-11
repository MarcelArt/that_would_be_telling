use actix_web::{delete, get, post, put, web::{self, Json}, HttpResponse};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use crate::{models::{claims::{AccessTokenClaims, RefreshTokenClaims}, response::Response, user::{LoginInput, LoginResponse, RefreshInput, User, UserDto}}, repos::{self, base::{ICreate, IDelete, IGetById, IRead, IUpdate}}, utils::{env::get_env, time::{self}}};

#[post("/")]
pub async fn create(repo: web::Data<repos::Combined>, user: Json<UserDto>) -> HttpResponse {    
    let user = user.0;
    let user = repo.user.create(user).await;

    match user {
        Err(e) =>  {
            return HttpResponse::InternalServerError().json(Response::<User>::failure(e.to_string()));
        },
        Ok(None) => {
            return HttpResponse::InternalServerError().json(Response::<User>::failure("Failed to create user".to_string()));
        },
        Ok(Some(u)) => {
            return HttpResponse::Created().json(Response::<User>::success(Some(u), "User created successfully".to_string()));
        }
    }
}

#[get("/")]
pub async fn read(repo: web::Data<repos::Combined>) -> HttpResponse {
    let users = repo.user.read().await;
    
    match users {
        Err(e) =>  {
            return HttpResponse::InternalServerError().json(Response::<Vec<User>>::failure(e.to_string()));
        },
        Ok(us) => {
            return HttpResponse::Ok().json(Response::<Vec<User>>::success(Some(us), "Users retrieved successfully".to_string()));
        }
    }
}

#[put("/{id}")]
pub async fn update(repo: web::Data<repos::Combined>, id: web::Path<String>, input: Json<UserDto>) -> HttpResponse {
    let user = repo.user.update(id.into_inner(), input.0).await;

    match user {
        Err(e) =>  {
            return HttpResponse::InternalServerError().json(Response::<User>::failure(e.to_string()));
        },
        Ok(None) => {
            return HttpResponse::NotFound().json(Response::<User>::failure("User not found".to_string()));
        },
        Ok(Some(u)) => {
            return HttpResponse::Ok().json(Response::<User>::success(Some(u), "User updated successfully".to_string()));
        }
    }
}

#[delete("/{id}")]
pub async fn delete(repo: web::Data<repos::Combined>, id: web::Path<String>) -> HttpResponse {
    let user = repo.user.delete(id.into_inner()).await;

    match user {
        Err(e) =>  {
            return HttpResponse::InternalServerError().json(Response::<User>::failure(e.to_string()));
        },
        Ok(None) => {
            return HttpResponse::NotFound().json(Response::<User>::failure("User not found".to_string()));
        },
        Ok(Some(u)) => {
            return HttpResponse::Ok().json(Response::<User>::success(Some(u), "User deleted successfully".to_string()));
        }
    }
}

#[get("/{id}")]
pub async fn get_by_id(repo: web::Data<repos::Combined>, id: web::Path<String>) -> HttpResponse {
    let user = repo.user.get_by_id(id.into_inner()).await;

    match user {
        Err(e) =>  {
            return HttpResponse::InternalServerError().json(Response::<User>::failure(e.to_string()));
        },
        Ok(None) => {
            return HttpResponse::NotFound().json(Response::<User>::failure("User not found".to_string()));
        },
        Ok(Some(u)) => {
            return HttpResponse::Ok().json(Response::<User>::success(Some(u), "User retrieved successfully".to_string()));
        }
    }
}

#[post("/login")]
pub async fn login(repo: web::Data<repos::Combined>, user: Json<LoginInput>) -> HttpResponse {
    let user = user.0;
    let is_remember = user.is_remember;
    let user = repo.user.login(user.username, user.password).await;

    match user {
        Err(e) => {
            return HttpResponse::InternalServerError().json(Response::<LoginResponse>::failure(e.to_string()));
        },
        Ok(None) => {
            return HttpResponse::Unauthorized().json(Response::<LoginResponse>::failure("Invalid username or password".to_string()));
        },
        Ok(Some(u)) => {
            let exp = (chrono::Utc::now().timestamp() + 5 * time::MINUTE) as usize; // 5 minutes expiration
            let secret = get_env("JWT_SECRET").unwrap_or_default();
            let access_token_claims = AccessTokenClaims {
                username: u.username.clone(),
                user_id: u.id.to_string(),
                exp, 
            };
            let refresh_token_claims = RefreshTokenClaims {
                user_id: u.id.to_string(),
                exp: (chrono::Utc::now().timestamp() + if is_remember { time::MONTH } else { time::DAY }) as usize,
                is_remember,
            };
            let access_token = encode(&Header::default(), &access_token_claims, &EncodingKey::from_secret(secret.as_bytes()));
            let refresh_token = encode(&Header::default(), &refresh_token_claims, &EncodingKey::from_secret(secret.as_bytes()));

            match (access_token, refresh_token) {
                (Ok(access), Ok(refresh)) => {
                    let response = LoginResponse {
                        access_token: access,
                        refresh_token: refresh,
                    };
                    return HttpResponse::Ok().json(Response::<LoginResponse>::success(Some(response), "Login successful".to_string()));
                },
                (Err(e), _) | (_, Err(e)) => {
                    return HttpResponse::InternalServerError().json(Response::<LoginResponse>::failure(e.to_string()));
                }
            }
        }
    }
}

#[post("/refresh")]
pub async fn generate_new_token_pair(repo: web::Data<repos::Combined>, input: Json<RefreshInput>) -> HttpResponse {
    let secret = get_env("JWT_SECRET").unwrap_or_default();
    let decode_res = decode::<RefreshTokenClaims>(&input.refresh_token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default());

    match decode_res {
        Ok(token_data) => {
            let user_id = token_data.claims.user_id;
            let id_part = user_id.split(':').last().unwrap_or_default();
            let is_remember = token_data.claims.is_remember;

            let user = repo.user.get_by_id(String::from(id_part)).await;

            match user {
                Err(e) => {
                    return HttpResponse::InternalServerError().json(Response::<User>::failure(e.to_string()));
                },
                Ok(None) => {
                    return HttpResponse::Unauthorized().json(Response::<User>::failure("Unregistered user".to_string()));
                },
                Ok(Some(u)) => {
                    let exp = (chrono::Utc::now().timestamp() + 5 * time::MINUTE) as usize; // 5 minutes expiration
                    let secret = get_env("JWT_SECRET").unwrap_or_default();
                    let access_token_claims = AccessTokenClaims {
                        username: u.username.clone(),
                        user_id: u.id.to_string(),
                        exp, 
                    };
                    let refresh_token_claims = RefreshTokenClaims {
                        user_id: u.id.to_string(),
                        exp: (chrono::Utc::now().timestamp() + if is_remember { time::MONTH } else { time::DAY }) as usize,
                        is_remember,
                    };
                    let access_token = encode(&Header::default(), &access_token_claims, &EncodingKey::from_secret(secret.as_bytes()));
                    let refresh_token = encode(&Header::default(), &refresh_token_claims, &EncodingKey::from_secret(secret.as_bytes()));
        
                    match (access_token, refresh_token) {
                        (Ok(access), Ok(refresh)) => {
                            let response = LoginResponse {
                                access_token: access,
                                refresh_token: refresh,
                            };
                            return HttpResponse::Ok().json(Response::<LoginResponse>::success(Some(response), "New token pair generated".to_string()));
                        },
                        (Err(e), _) | (_, Err(e)) => {
                            return HttpResponse::InternalServerError().json(Response::<LoginResponse>::failure(e.to_string()));
                        }
                    }
                }
            }
        },
        Err(e) => {
            return HttpResponse::Unauthorized().json(Response::<LoginResponse>::failure(e.to_string()));
        }
    }
}