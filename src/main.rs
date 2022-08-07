mod create_tables;
mod indexing;

use create_tables::create_tables;
use indexing::index_folder;

fn main() {
    // Create database and tables if required
    create_tables().ok();
    let _res = index_folder("/Users/ratulmaharaj/Desktop");
    // for item in res.iter() {
    //     println!("{:?} \n", item)
    // }
}
