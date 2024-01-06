use crate::entities::*;
use crate::AppState;
use actix_web::http::Error;
use actix_web::Responder;
use actix_web::{
    get, post,
    web::{self, Redirect},
    HttpResponse,
};
use nanoid::{alphabet::SAFE, nanoid};
use sea_orm::{prelude::*, *};

#[derive(serde::Deserialize, Debug)]
struct UrlInfo {
    full_url: String,
}

#[post("/")]
async fn create_short_url(
    app_state: web::Data<AppState>,
    body: web::Json<UrlInfo>,
) -> HttpResponse {
    println!("{:?}", body);
    let new_url = url::ActiveModel {
        slug: ActiveValue::Set(nanoid!(8, &SAFE)),
        url: ActiveValue::Set(body.full_url.to_owned()),
        ..Default::default()
    };

    let res = new_url.insert(&app_state.db).await;
    match res {
        Ok(res) => HttpResponse::Ok().json(res),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/{slug}")]
pub async fn redirect(
    app_state: web::Data<AppState>,
    slug: web::Path<String>,
) -> Result<impl Responder, Error> {
    let url = url::Entity::find_by_id(slug.clone())
        .column(url::Column::Url)
        .one(&app_state.db)
        .await;

    if let Ok(Some(url)) = url {
        let _ = url::Entity::update_many()
            .filter(url::Column::Url.eq(slug.clone()))
            .col_expr(url::Column::Views, Expr::col(url::Column::Views).add(1))
            .exec(&app_state.db)
            .await
            .map_err(|_| todo!());

        Ok(Redirect::to(url.url).permanent())
    } else {
        todo!()
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/shorten").service(create_short_url));
}
