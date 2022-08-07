mod indexing;
use indexing::index_folder;

fn main() {
    let res = index_folder("/Users/ratulmaharaj/Desktop");
    // for item in res.iter() {
    //     println!("{:?} \n", item)
    // }
}
