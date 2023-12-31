use diesel::prelude::*;

use crate::schema::{alts, players};
use rocket::serde::Serialize;

#[derive(Queryable, Selectable, Identifiable, Debug, PartialEq, Serialize)]
#[diesel(table_name = players)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[serde(crate = "rocket::serde")]
pub struct Player {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Player))]
#[diesel(table_name = alts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Alt {
    pub id: i32,
    pub name: String,
    pub player_id: i32,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = players)]
pub struct NewPlayer<'a> {
    pub name: &'a str,
}
