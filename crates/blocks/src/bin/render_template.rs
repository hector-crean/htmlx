use askama::Template;
use blocks::block::BlocksTemplate;

fn main() {
    let blocks = BlocksTemplate::example_blocks(); // instantiate your struct
    println!("{}", blocks.render().unwrap()); // then render it.
}