// Importing modules, libryries && framework
#[macro_use] extern crate rocket;
use rocket::serde::json::Json;
use lib::db;
use lib::model::Movie;

// Search all movies
#[get("/")]
fn get_movies() -> Json<Option<Vec<Movie>>> {
    Json(db::read_movies())
}

// Search movie by title
#[get("/<title>")]
fn get_movie(title: &str) -> Json<Option<Movie>> {
    Json(db::read_movie(title.to_string()))
}

// Insert new movie on movies list
#[post("/", data = "<movie>")]
fn create_movie(movie: Json<Movie>) -> Json<Option<Movie>> {
    Json(db::insert_movie(movie.0))
}

// Exclude movie by title
#[delete("/<title>")]
fn delete_movie(title: &str) -> Json<bool> {
    Json(db::delete_movie(title.to_string()))
}

// Laucnh server && define routes
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/movies", routes![get_movies, get_movie, create_movie, delete_movie])
}
 
 