mod indexing;
use indexing::index_folder;

pub fn update() {
    println!("Updating indexes. Please wait.");
    // fetch list of folders to index from database
    let _res = index_folder("/Users/ratulmaharaj/Desktop");
    // loop over folders and index each one
    println!("Updating of indexes complete.");
}
