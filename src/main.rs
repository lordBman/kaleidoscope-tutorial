mod comiplier;
fn main() {
    println!("Hello, world!");

    let mut init = comiplier::construct_lexer("123");

    let (_start, got, _end) = init.next().unwrap().unwrap();

    println!("{:?}", got);
}
