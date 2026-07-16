use rand::{rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let mut cards = Vec::new();
        let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
        let ranks = ["2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King", "Ace"];

        for suit in suits {
            for rank in ranks {
                cards.push(format!("{} of {}", rank, suit));
            }
        }

        Deck { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = rng();
        self.cards.shuffle(&mut rng);
    }

    fn draw(&mut self) -> Option<String> {
        self.cards.pop()
    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        let mut dealt_cards = Vec::new();
        for _ in 0..num_cards {
            if let Some(card) = self.draw() {
                dealt_cards.push(card);
            } else {
                break;
            }
        }
        dealt_cards
    }
}

fn main() {
    let mut deck = Deck::new();

    deck.shuffle();

    let card = deck.draw();
    let dealt_cards = deck.deal(5);

    println!("Drew card: {:#?}", card);
    println!("Remaining deck: {:#?}", deck);
    println!("Dealt cards: {:#?}", dealt_cards);
}
