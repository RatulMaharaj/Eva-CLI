extern crate diesel;
extern crate diesel_demo;

use self::diesel::prelude::*;
use self::diesel_demo::*;
use self::models::*;

fn main() {
    use diesel_demo::schema::folders_to_index::dsl::*;

    let connection = establish_connection();
    let results = folders_to_index
        .limit(5)
        .load::<Folder>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for folder in results {
        println!("{}", folder.folder_name);
        println!("----------\n");
    }
}
