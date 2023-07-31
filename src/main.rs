mod first;
mod second;
mod third;

use first::first;
use second::second;
use third::third;

fn main() {
    first();
    second();
    third();
    println!("Hello, world!");
}
