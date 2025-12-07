use actix_web::{
    HttpRequest, HttpResponse, cookie::{Cookie, time::Duration}, post, web::{Data, Json}
};

use crate::{
    auth::{decode_jwt, generate_jwt, is_valid},
    db_service::Dbservice,
    model::{Login, NewCustomer},
    password_helper::hash_password,
};

// the db and JSON are the extractor like in axum STATE and JSON
#[post("/register")]
pub async fn register(db: Data<Dbservice>, customer: Json<NewCustomer>) -> HttpResponse {
    let mut data = customer.into_inner();
    let hash = hash_password(data.password);
    data.password = hash;

    match db.create_customer(data) {
        Ok(_) => HttpResponse::Ok().body("success"),
        Err(e) => {
            eprintln!("Error Registering the user.. {:?}", e);
            HttpResponse::InternalServerError().body("try again!!!!")
        }
    }
}

#[post("/login")]
pub async fn login_user(db: Data<Dbservice>, login: Json<Login>) -> HttpResponse {
    let mut data = login.into_inner();
    let hash = hash_password(data.password);
    data.password = hash;

    // since the current login() fn returns a Option we will handle using Some
    match db.login(data) {
        Some(details) => {
            let cookie = Cookie::build(
                "token",
                // generate_jwt(details.member_id, &details.email).unwrap(),
                generate_jwt(details.member_id, "MY secret DEV").unwrap(),
            )
            .path("/")
            .max_age(Duration::seconds(3600))
            .same_site(actix_web::cookie::SameSite::None)
            .http_only(true)
            .finish();

            HttpResponse::Ok().cookie(cookie).finish()
        }
        None => HttpResponse::InternalServerError().body("Try again"),
    }
}


// we will redirect user to the profile page once is loggedin and the cookie is passed in the request
#[post("/profile")] 
pub async fn profile(db: Data<Dbservice>, request: HttpRequest) -> HttpResponse {
    if let Some(cookie) = request.cookie("token") {
        match decode_jwt(cookie.value(), "MY secret DEV") {
            Ok(token_data) => {
                if is_valid(token_data.&claims) {
                    match db.get_by_memberid(&token_data.claims.sub) {
                        Some()
                    }
                }
            }
        }
    }

    todo!()
}
