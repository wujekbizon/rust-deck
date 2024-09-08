use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Spades", "Diamonds", "Clubs"];
        let values = [
            "Ace", "King", "Queen", "Jack", "Ten", "Nine", "Eight", "Seven", "Six", "Five", "Four",
            "Three", "Two",
        ];

        let mut cards = vec![];

        for suit in suits {
            for value in values {
                let card = format!("{value} of {suit}");
                cards.push(card);
            }
        }

        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();

    // Need t oadd error handling!
    let hand = deck.deal(6);

    println!("Heres your hand: {:#?}", hand);
    println!("Heres your deck: {:#?}", deck);
}
