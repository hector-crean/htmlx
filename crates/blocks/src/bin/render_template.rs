use askama::Template;
use blocks::block::BlocksProps;

fn main() {
    let blocks = BlocksProps::example_blocks(); // instantiate your struct
    println!("{}", blocks.render().unwrap()); // then render it.
}
