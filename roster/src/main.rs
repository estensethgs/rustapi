use crate::db::create_player;

use self::models::Player;
use self::schema::players::*;
use diesel::prelude::*;
use rocket::serde::json::Json;
mod db; // Import the `db` module
mod models; // Import the `models` module from the crate root
mod schema;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> Json<Vec<Player>> {
    use crate::dsl::players;
    let connection = &mut db::establish_connection(); // Use the `db` module to establish the connection

    let playername = "Sean";

    let _ = create_player(connection, playername);

    let results = players
        .filter(name.eq("Sean"))
        .limit(5)
        .select(Player::as_select())
        .load(connection)
        .expect("Error loading posts");

    Json(results)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
