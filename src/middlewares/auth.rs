use actix_web::{body::{EitherBody, MessageBody}, dev::{ServiceRequest, ServiceResponse}, http::header::{self}, middleware::Next, Error, HttpResponse};
use jsonwebtoken::{decode, DecodingKey, Validation};
use dotenv_codegen::dotenv;
use crate::models::{claims::AccessTokenClaims, response::Response};

pub async fn protect<B>(req: ServiceRequest, next: Next<B>) -> Result<ServiceResponse<EitherBody<B>>, Error>
where
    B: MessageBody + 'static,
{
    let secret = dotenv!("JWT_SECRET");
    let token = req.headers().get(header::AUTHORIZATION);
    if token.is_none() {
        let (req, _) = req.into_parts();
        let res = HttpResponse::Unauthorized()
            .json(Response::<()>::failure("please login to access the resources".to_string()))
            .map_into_right_body(); 
        
        return Ok(ServiceResponse::new(req, res));
    }

    let token = token.unwrap().to_str();
    if matches!(token, Err(_)) {
        let (req, _) = req.into_parts();
        let res = HttpResponse::Unauthorized()
            .json(Response::<()>::failure("malformed token 1".to_string()))
            .map_into_right_body(); 
        
        return Ok(ServiceResponse::new(req, res));
    }

    let token = token.unwrap().strip_prefix("Bearer ");
    if token.is_none() {
        let (req, _) = req.into_parts();
        let res = HttpResponse::Unauthorized()
            .json(Response::<()>::failure("malformed token 2".to_string()))
            .map_into_right_body(); 
        
        return Ok(ServiceResponse::new(req, res));
    }
    let token = token.unwrap();
    let decoded_token = decode::<AccessTokenClaims>(token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default());

    if let Err(e) = decoded_token {
        let (req, _) = req.into_parts();
        let res = HttpResponse::Unauthorized()
            .json(Response::<()>::failure(e.to_string()))
            .map_into_right_body(); 
        
        return Ok(ServiceResponse::new(req, res));
    }

    println!("Decoded token: {:#?}", decoded_token);

    let res = next.call(req).await?.map_into_left_body();

    Ok(res)
}