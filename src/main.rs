#[derive(Debug)]

struct Deck {
    cards: Vec<String>,
}

fn main() {
    let deck: Deck = Deck { cards: Vec::new() };

    println!("Heres your deck: {:?}", deck);
}
