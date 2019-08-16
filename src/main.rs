use crate::entity::create_entity;

mod entity;

fn main() {
    let e = create_entity();
    println!("{}", e.ident);
}
