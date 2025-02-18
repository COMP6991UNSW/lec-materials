#![allow(unused)]

struct Card {
    rank: Rank,
    suit: Suit,
}

enum Rank {
    FaceRank(FaceRank),
    Number(i32),
}

enum FaceRank {
    Ace,
    Jack,
    Queen,
    King,
}

#[derive(Clone, Copy)]
enum Suit {
    Clubs,
    Hearts,
    Diamonds,
    Spades,
}

#[derive(Clone, Copy)]
struct Foo {
    x: i32,
    y: bool,
    z: char,
    a: [char; 10],
}

#[derive(Clone, Copy)]
struct Bar {
    foo: Foo,
}

fn print_string(string: String) -> String {
    println!("{string}");
    string
}

fn print_string2(string: String) {
    println!("{string}");
}

fn main() {
    let string1 = String::from("hello");
    let string1 = print_string(string1);
    println!("{string1}");

    let string1 = String::from("hello");
    print_string(string1.clone());
    println!("{string1}");

    // i32, u32, i64, u64, i16, u16, i8, u8, i128, u128, f32, f64, char, bool
    // 'x',
    // true, false

    let suit = Suit::Clubs;

    let card = Card {
        rank: Rank::Number(3),
        suit,
    };

    let colour = match suit {
        Suit::Clubs | Suit::Spades => "black",
        Suit::Hearts | Suit::Diamonds => "red",
    };


    // Rank::FaceRank { rank }   <=>    Rank::FaceRank { rank: FaceRank::Ace },
    // rank = FaceRank::Ace
    //
    // Rank::FaceRank(rank)   <=>    Rank::FaceRank(FaceRank::Ace),
    // rank = FaceRank::Ace
    //
    // x + 3 = 42 * 2
    // x = 81
    //
    // (x, y) = (12, 50)
    // let (x, y) = (12, 50);

    let rank_name = match card.rank {
        Rank::FaceRank(rank) => {
            match rank {
                FaceRank::Ace => "A",
                FaceRank::Jack => "J",
                FaceRank::Queen => "Q",
                FaceRank::King => "K",
            }.to_string()
        }
        Rank::Number(rank) => rank.to_string(),
    };

    let suit_name = match card.suit {
        Suit::Clubs => '♣',
        Suit::Hearts => '♥',
        Suit::Diamonds => '♦',
        Suit::Spades => '♠',
    };

    println!("{rank_name}{suit_name}");
}
