use super::DbPool;

use actix_web::{ get, post, put, delete, web, Error, HttpResponse};
use diesel::prelude::*;
use uuid::Uuid;

use crate::models::{NewArtist, Artist, ArtistPayload};

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[post("/api/artists/")]
async fn create(
  pool: web::Data<DbPool>,
  payload: web::Json<ArtistPayload>,
) -> Result<HttpResponse, Error> {
  let artist = web::block(move || {
    let conn = pool.get()?;
    add_an_artist(&payload.name,&payload.genre, &conn)
  })
  .await?
  .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(HttpResponse::Ok().json(artist))
}

#[get("/api/artists/")]
async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
  let artists = web::block(move || {
    let conn = pool.get()?;
    find_all(&conn)
  })
  .await?
  .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(HttpResponse::Ok().json(artists))
}

#[get("/api/artists/{id}")]
async fn show(id: web::Path<String>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
  let artist = web::block(move || {
    let conn = pool.get()?;
    find_by_id(id.to_string(), &conn)
  })
  .await?
  .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(HttpResponse::Ok().json(artist))
}

#[put("/api/artists/{id}")]
async fn update(
  id: web::Path<String>,
  payload: web::Json<ArtistPayload>,
  pool: web::Data<DbPool>,
) -> Result<HttpResponse, Error> {
  let an_artist = web::block(move || {
    let conn = pool.get()?;
    update_artist(id.to_string(), payload.name.clone(), payload.genre.clone(), &conn)
  })
  .await?
  .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(HttpResponse::Ok().json(an_artist))
}

#[delete("/api/artists/{id}")]
async fn destroy(id: web::Path<String>, pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
  let result = web::block(move || {
    let conn = pool.get()?;
    delete_artist(id.to_string(), &conn)
  })
  .await?
  .map(|artist| HttpResponse::Ok().json(artist))
  .map_err(actix_web::error::ErrorInternalServerError)?;

  Ok(result)
}


fn add_an_artist(_name: &str, _genre: &str, conn: &PgConnection) -> Result<Artist, DbError> {
    use crate::schema::artist::dsl::*;
    let my_uuid_string = Uuid::new_v4().to_string();
    let new_artist = NewArtist {
      id: &my_uuid_string,
      name: _name,
      genre: _genre,
    };
  
    let res = diesel::insert_into(artist)
      .values(&new_artist)
      .get_result(conn)?;
    Ok(res)
  }

  fn find_all(conn: &PgConnection) -> Result<Vec<Artist>, DbError> {
    use crate::schema::artist::dsl::*;
  
    let items = artist.load::<Artist>(conn)?;
    Ok(items)
  }


fn find_by_id(artist_id: String, conn: &PgConnection) -> Result<Option<Artist>, DbError> {
    use crate::schema::artist::dsl::*;
  
    let an_artist = artist
      .filter(id.eq(artist_id))
      .first::<Artist>(conn)
      .optional()?;
  
    Ok(an_artist)
  }

  fn update_artist(artist_id: String, _name: String, _genre: String, conn: &PgConnection) -> Result<Artist, DbError> {
    use crate::schema::artist::dsl::*;

    let an_artist = diesel::update(artist.find(artist_id))
      .set( (name.eq(_name), genre.eq(_genre)) )
      .get_result::<Artist>(conn)?;
    Ok(an_artist)
  }

  fn delete_artist(artist_id: String, conn: &PgConnection) -> Result<usize, DbError> {
    use crate::schema::artist::dsl::*;
  
    let count = diesel::delete(artist.find(artist_id)).execute(conn)?;
    Ok(count)
  }
  