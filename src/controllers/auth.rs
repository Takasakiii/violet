use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse,
};
use serde::Deserialize;
use validator::Validate;

use crate::{
    database::{
        users::{self, Users, UsersErrors, UsersGetByUsernameErrors},
        Database,
    },
    jwt::Jwt,
};

#[derive(Deserialize, Validate, Debug)]
pub struct SingUpDto {
    #[validate(length(
        min = 3,
        max = 255,
        message = "Username must be between 3 and 255 characters"
    ))]
    username: String,
    #[validate(length(
        min = 8,
        max = 255,
        message = "Password must be between 8 and 255 characters"
    ))]
    password: String,
    #[validate(must_match(other = "password", message = "Passwords do not match"))]
    repeat_password: String,
}

#[post("/singup")]
pub async fn sing_up(sing_up: Json<SingUpDto>, database: Data<Database>) -> HttpResponse {
    let sing_up = sing_up.into_inner();

    if let Err(e) = sing_up.validate() {
        return HttpResponse::UnprocessableEntity().json(e);
    }

    let password = match bcrypt::hash(sing_up.password, 10) {
        Ok(password) => password,
        Err(error) => {
            log::error!("Failed to hash password: {}", error);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let db_request = Users {
        username: sing_up.username,
        password,
        last_token: None,
    };

    match users::create(&*database, db_request).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(UsersErrors::DuplicateUsername) => HttpResponse::Conflict().json(serde_json::json!({
            "error": "Username already exists"
        })),
        Err(UsersErrors::Generic(e)) => {
            log::error!("Failed to create user: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(Deserialize)]
pub struct LoginDto {
    username: String,
    password: String,
}

#[post("/login")]
pub async fn login(
    login_dto: Json<LoginDto>,
    database: Data<Database>,
    jwt: Data<Jwt>,
) -> HttpResponse {
    let login = login_dto.into_inner();

    let user = match users::get_by_username(&*database, login.username).await {
        Ok(user) => user,
        Err(UsersGetByUsernameErrors::NotFound) => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Username or password is incorrect"
            }));
        }
        Err(UsersGetByUsernameErrors::Generic(e)) => {
            log::error!("Failed to get user: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    match bcrypt::verify(login.password, &user.password) {
        Ok(true) => {
            let token = jwt.create_jwt(user.username.clone());
            let token = match token {
                Ok(token) => {
                    if let Err(err) =
                        users::add_last_token(&*database, token.clone(), user.username).await
                    {
                        log::error!("Failed to add last token: {}", err);
                        return HttpResponse::InternalServerError().finish();
                    }

                    token
                }
                Err(err) => {
                    log::error!("Failed to create token: {}", err);
                    return HttpResponse::InternalServerError().finish();
                }
            };

            HttpResponse::Ok().json(serde_json::json!({ "token": token }))
        }
        Ok(false) => HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Username or password is incorrect"
        })),
        Err(e) => {
            log::error!("Failed to verify password: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
