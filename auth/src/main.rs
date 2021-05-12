use actix_redis::RedisSession;
use actix_session::Session;
use actix_web::HttpResponse;
use actix_web::{error, middleware::Logger, post, web, App, HttpServer, Result};
use log::info;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;
use storage::Storage;

pub mod storage;

#[derive(Clone, Deserialize, Serialize)]
enum Role {
    User,
    Admin,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct User {
    id: Option<uuid::Uuid>,
    email: String,
    password: String,
}

impl User {
    fn new(email: String, password: String) -> Self {
        let id = Uuid::new_v4();

        Self {
            id: Some(id),
            email,
            password,
        }
    }
}

#[derive(Deserialize)]
struct Login {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    email: String,
}

struct AppState {
    users: Mutex<HashMap<String, User>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

#[post("/api/auth/signup")]
async fn signup(data: web::Data<AppState>, body: web::Json<User>) -> Result<web::Json<User>> {
    let mut users = data.users.lock().unwrap(); // <- get counter's MutexGuard

    let user = body.into_inner();
    users.insert(user.email.clone(), user.clone());
    Ok(web::Json(user))
}

async fn auth(session: Session) -> Result<HttpResponse> {
    let id: Option<String> = session.get("user_id")?;

    info!("User id: {:?}", id);

    if let Some(_) = id {
        Ok(HttpResponse::Ok().message_body("Ok".into()))
    } else {
        Err(error::ErrorUnauthorized("wrong email or password"))
    }
}

async fn login(
    data: web::Data<AppState>,
    body: web::Json<Login>,
    session: Session,
) -> Result<HttpResponse> {
    let login = body.into_inner();

    let users = data.users.lock().unwrap(); // <- get counter's MutexGuard
    let user = users.get(&login.email);

    if let Some(user) = user {
        if login.password.eq(&user.password) {
            session.set("user_id", &user.email)?;
            return Ok(HttpResponse::Ok().json(LoginResponse {
                email: user.email.clone(),
            }));
        }
    }

    Err(error::ErrorUnauthorized("wrong email or password"))
}

async fn create_server(users: web::Data<AppState>) -> std::io::Result<()> {

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let private_key = rand::thread_rng().gen::<[u8; 32]>();
    HttpServer::new(move || {
        App::new()
            .app_data(users.clone())
            .wrap(
                RedisSession::new("redis-master:6379", &private_key)
                    .cookie_same_site(actix_redis::SameSite::Lax),
            )
            .wrap(Logger::default())
            .service(signup)
            .service(web::resource("/api/auth/login").route(web::post().to(login)))
            .service(web::resource("/api/auth/auth").route(web::get().to(auth)))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let usermap = web::Data::new(AppState {
        users: Mutex::new(Default::default()),
    });
    let matches = clap::App::new("auth")
        .version("1.0")
        .author("Jan-Gerrit Harms")
        .about("Authentication server for cloud application")
        .arg(
            clap::Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Load a config file")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("redis-host")
                .long("redis.host")
                .value_name("hostname")
                .help("Hostname of a redis instance"),
        )
        .subcommand(
            clap::SubCommand::with_name("create-user")
                .about("Creates a user")
                .arg(
                    clap::Arg::with_name("email")
                        .help("Email address of the new user")
                        .index(1)
                        .required(true),
                )
                .arg(
                    clap::Arg::with_name("password")
                        .help("Password of the new user")
                        .index(2)
                        .required(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("create-user") {
        let email = matches.value_of("email").unwrap();
        let password = matches.value_of("password").unwrap();
        let new_user = User::new(email.into(), password.into());
        let mut users = usermap.users.lock().unwrap(); // <- get counter's MutexGuard

        users.insert(email.into(), new_user.clone());
        println!("Created a user: {:?}", new_user);
        Ok(())
    } else {
        println!("something else");
        create_server(usermap).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[actix_rt::test]
    // async fn test_login() {
    //     let mut users = HashMap::new();
    //     users.insert(
    //         "jan@blocklab.nl".to_string(),
    //         User {
    //             id: None,
    //             firstname: "Jan".to_string(),
    //             lastname: "Harms".to_string(),
    //             username: "janharms".to_string(),
    //             email: "jan@blocklab.nl".to_string(),
    //             password: "helloworld".to_string(),
    //             company: "blocklab".to_string(),
    //             role: Role::User,
    //         },
    //     );
    //     let usermap = web::Data::new(AppState {
    //         users: Mutex::new(users),
    //     });
    //     let resp = login(
    //         usermap,
    //         web::Json(Login {
    //             email: "jan@blocklab.nl".to_string(),
    //             password: "helloworld".to_string(),
    //         }),
    //     )
    //     .await;
    //     assert!(resp.is_ok());
    // }
    // #[actix_rt::test]
    // async fn test_login_wrong_password() {
    //     let mut users = HashMap::new();
    //     users.insert(
    //         "jan@blocklab.nl".to_string(),
    //         User {
    //             id: None,
    //             firstname: "Jan".to_string(),
    //             lastname: "Harms".to_string(),
    //             username: "janharms".to_string(),
    //             email: "jan@blocklab.nl".to_string(),
    //             password: "helloworld".to_string(),
    //             company: "blocklab".to_string(),
    //             role: Role::User,
    //         },
    //     );
    //     let usermap = web::Data::new(AppState {
    //         users: Mutex::new(users),
    //     });
    //     let resp = login(
    //         usermap,
    //         web::Json(Login {
    //             email: "jan@blocklab.nl".to_string(),
    //             password: "helloworl".to_string(),
    //         }),
    //     )
    //     .await;
    //     assert!(resp.is_err());
    //     // assert_eq!(
    //     //     resp,
    //     //     Err(error::ErrorUnauthorized("wrong email or password"))
    //     // );
    // }
}
