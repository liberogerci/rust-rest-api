// Importing modules, libryries && framework
use std::fs;
use rocket::serde::json::serde_json;
use crate::model::Movie;

// Defining storage path from movies in json file
static MOVIES_DB: &str = "data/movies.json";

// Function to record movies at file system
fn _write_movies(movies: Vec<Movie>) {
    let data = serde_json::to_string(&movies).expect("Failed to turn movies into serde string");
    
    fs::write(MOVIES_DB, data).expect("Failed to write data.");
}

// Function to reads at file system movies file
pub fn _movies() -> Result<Vec<Movie>, serde_json::Error> {
    let data = fs::read_to_string(MOVIES_DB).expect("Error reading from file");

    let movies: Result<Vec<Movie>, serde_json::Error> = serde_json::from_str(&data);

    movies
}

// Function to consult all movies
pub fn read_movies() -> Option<Vec<Movie>> {
    match _movies() {
        Ok(movies) => Some(movies),
        Err(_) => None
    }
}

// Function to consult  movie by title
pub fn read_movie(title: String) -> Option<Movie> {
    match _movies() {
        Ok(movies) => {
            
            let index = movies.iter().position(|m| m.title == title);

            match index {
                Some(x) => Some(movies[x].clone()),
                None => None,
            }
        },
        Err(_) => None
    }
}

// Function for add a new movie at the data
pub fn insert_movie(movie: Movie) -> Option<Movie> {
    match _movies() {
        Ok(mut movies) => {
            movies.push(movie.clone());
            _write_movies(movies);
            Some(movie)
        },
        Err(_) => None
    }
}

// Function for delete a new movie at the data by title
pub fn delete_movie(title: String) -> bool {
    match _movies() {
        Ok(mut movies) => {
            let index = movies.iter().position(|m| m.title == title);

            match index {
                Some(x) => {
                    movies.remove(x);
                    _write_movies(movies);
                    true
                },
                None => false,
            }
        },
        Err(_) => false
    }
}