use serde::{Deserialize, Serialize};

use crate::schema::artist;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Artist {
  pub id: String,
  pub name: String,
  pub genre: String,
}

#[derive(Debug, Insertable)]
#[table_name = "artist"]
pub struct NewArtist<'a> {
  pub id: &'a str,  
  pub name: &'a str,
  pub genre: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtistPayload {
  pub name: String,
  pub genre: String,
}
