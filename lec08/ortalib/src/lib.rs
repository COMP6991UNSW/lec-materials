use std::{
    fmt::{Debug, Display, Formatter},
    str::FromStr,
    sync::atomic::{AtomicUsize, Ordering},
};

use enum_iterator::{Sequence, all};
use serde::{Deserialize, Serialize, de::Visitor};

pub type Chips = f64;
pub type Mult = f64;

/// Represents the total inputs of one scoring round in Ortalab.
///
/// Contains:
///
/// * The [`Card`]s that the user has selected to play.
/// * The [`Card`]s that the user has kept held in their hand (i.e. not played).
/// * The player's (up to five) held [`JokerCard`]s.
#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Round {
    pub cards_played: Vec<Card>,

    #[serde(default)]
    pub cards_held_in_hand: Vec<Card>,

    #[serde(default)]
    pub jokers: Vec<JokerCard>,
}

/// Represents one playing card.
///
/// Contains:
///
/// * The [`Rank`] of the card (2-10 J Q K A)
/// * The [`Suit`] (**♥Hearts**, **♣Clubs**, **♦Diamonds**, **♠Spades**)
/// * A possible [`Enhancement`].
/// * A possible [`Edition`].
///
/// `Card`s will **only** compare equal to themselves.
/// i.e. if you have two distinct cards with the same values,
///      but they were not minted from the same `Card`,
///      they will compare non-equal.
///      However, if you `Copy` or `Clone` a `Card`, it will compare
///      equal to the original.
///
/// If you are looking for simple base card equality, instead compare
/// the `rank` / `suit`. Of course, `enhancement` / `edition` can also be
/// compared if needed.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
    pub enhancement: Option<Enhancement>,
    pub edition: Option<Edition>,
    #[doc(hidden)]
    unique_index: usize,
    // Skipped: Seal
}

/// Card Ranks are the thirteen values that can be found on a playing card in Ortalab.
/// These values are Aces, the numerals 2 to 10, and the three face cards: Jacks, Queens and Kings.
/// The value of each card can be accessed with [`Rank::rank_value`] as follows:
///
/// ```
/// # use ortalib::Rank;
/// assert_eq!(Rank::Two.rank_value(),   2.0);
/// assert_eq!(Rank::Five.rank_value(),  5.0);
/// assert_eq!(Rank::Ten.rank_value(),   10.0);
/// assert_eq!(Rank::Jack.rank_value(),  10.0);
/// assert_eq!(Rank::Queen.rank_value(), 10.0);
/// assert_eq!(Rank::King.rank_value(),  10.0);
/// assert_eq!(Rank::Ace.rank_value(),   11.0);
/// ```
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Sequence)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

/// In Ortalab, there are 4 suits: **♥Hearts**, **♣Clubs**, **♦Diamonds**, **♠Spades**.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Sequence)]
pub enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

/// Represents the color of a [`Suit`].
///
/// This is useful for determining which suits are "the same color".
///
/// Specifically, **♠Spades** and **♣Clubs** are black, and **♥Hearts** and **♦Diamonds** are red.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Sequence)]
pub enum SuitColor {
    Black,
    Red,
}

/// Represents one Joker card.
/// Simply includes the Joker effect, and a possible edition.
///
/// `JokerCard`s will **only** compare equal to themselves.
/// i.e. if you have two distinct `JokerCard`s with the same values,
///      but they were not minted from the same `JokerCard`,
///      they will compare non-equal.
///      However, if you `Copy` or `Clone` a `JokerCard`, it will compare
///      equal to the original.
///
/// If you are looking for simple base joker card equality, instead compare
/// the `joker` field. Of course, `edition`` can also be
/// compared if needed.

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct JokerCard {
    pub joker: Joker,
    pub edition: Option<Edition>,
    #[doc(hidden)]
    unique_index: usize,
}

/// The various Joker effects supported in Ortalab.
/// Check the enum variant documentation for details on
/// each of their individual effects.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Sequence)]
pub enum Joker {
    // Basic
    /// **+4 Mult**.
    ///
    /// Activates: **Independent**
    Joker,

    /// **+8 Mult** if played hand contains a **Pair**.
    ///
    /// Activates: **Independent**
    JollyJoker,

    /// **+12 Mult** if played hand contains a **Three of a Kind**.
    ///
    /// Activates: **Independent**
    ZanyJoker,

    /// **+10 Mult** if played hand contains a **Two Pair**.
    ///
    /// Activates: **Independent**
    MadJoker,

    /// **+12 Mult** if played hand contains a **Straight**.
    ///
    /// Activates: **Independent**
    CrazyJoker,

    /// **+10 Mult** if played hand contains a **Flush**.
    ///
    /// Activates: **Independent**
    DrollJoker,

    /// **+50 Chips** if played hand contains a **Pair**.
    ///
    /// Activates: **Independent**
    SlyJoker,

    /// **+100 Chips** if played hand contains a **Three of a Kind**.
    ///
    /// Activates: **Independent**
    WilyJoker,

    /// **+80 Chips** if played hand contains a **Two Pair**.
    ///
    /// Activates: **Independent**
    CleverJoker,

    /// **+100 Chips** if played hand contains a **Straight**.
    ///
    /// Activates: **Independent**
    DeviousJoker,

    /// **+80 Chips** if played hand contains a **Flush**.
    ///
    /// Activates: **Independent**
    CraftyJoker,

    // Easy
    /// **+3 Mult** for each Joker card.
    ///
    /// Activates: **Independent**
    AbstractJoker,

    /// Adds double the [Rank::rank_value] of lowest ranked card held in hand to Mult.
    ///
    /// Activates: **On Held**
    RaisedFist,

    /// **x3 Mult** if all cards held in hand are **♠Spades** or **♣Clubs**.
    ///
    /// Activates: **Independent**
    Blackboard,

    // Each King held in hand gives **x1.5 Mult**.
    ///
    /// Activates: **On Held**
    Baron,

    // Medium
    /// Played cards with **♦Diamond** suit give **+3 Mult** when scored.
    ///
    /// Activates: **On Scored**
    GreedyJoker,

    /// Played cards with **♥Heart** suit give **+3 Mult** when scored.
    ///
    /// Activates: **On Scored**
    LustyJoker,

    /// Played cards with **♠Spade** suit give **+3 Mult** when scored.
    ///
    /// Activates: **On Scored**
    WrathfulJoker,

    /// Played cards with **♣Club** suit give **+3 Mult** when scored.
    ///
    /// Activates: **On Scored**
    GluttonousJoker,

    /// Each played Ace, 2, 3, 5, or 8 gives **+8 Mult** when scored.
    ///
    /// Activates: **On Scored**
    Fibonacci,

    /// Played face cards give +30 Chips when scored.
    ///
    /// Activates: **On Scored**
    ScaryFace,

    /// Played cards with even rank give **+4 Mult** when scored.
    ///
    /// Even ranks: (10, 8, 6, 4, 2)
    ///
    /// Activates: **On Scored**
    EvenSteven,

    /// Played cards with odd rank give +31 Chips when scored
    ///
    /// Odd ranks: (A, 9, 7, 5, 3)
    ///
    /// Activates: **On Scored**
    OddTodd,

    /// First played face card gives **x2 Mult** when scored.
    ///
    /// Activates: **On Scored**
    Photograph,

    /// Played face cards give **+5 Mult** when scored.
    ///
    /// Activates: **On Scored**
    SmileyFace,

    /// **x3 Mult** if poker hand contains a
    /// **♦Diamond** card, **♣Club** card, **♥Heart** card, and **♠Spade** card.
    ///
    /// Activates: **Independent**
    FlowerPot,

    // Hard
    /// All Flushes and Straights can be made with 4 cards.
    ///
    /// Example 1: 4♦ 5♣ 6♦ 7♠ J♣ is a straight (J♣ not counted).
    ///
    /// Example 2: 3♦ 7♦ 9♦ A♦ is a flush.
    ///
    /// Example 3: 6♥ 7♥ 8♥ 9♥ is a straight flush.
    ///
    /// Example 4: 3♥ 4♥ 5♣ 6♥ J♥ is a straight flush,
    /// since it contains both a straight and a flush.
    ///
    /// Activates: **N/A**
    FourFingers,

    /// Allows Straights to be made with gaps of 1 rank
    ///
    /// (e.g. 10 8 6 5 3)
    ///
    /// Activates: **N/A**
    Shortcut,

    /// Retrigger all card held in hand abilities.
    ///
    /// Activates: **On Held**
    Mime,

    /// All cards are considered face cards.
    ///
    /// Activates: **N/A**
    Pareidolia,

    /// Every played card counts in scoring.
    ///
    /// Activates: **N/A**
    Splash,

    /// Retrigger all played face cards.
    ///
    /// Activates: **On Scored**
    SockAndBuskin,

    /// **♥Hearts** and **♦Diamonds** count as the same suit,
    /// **♠Spades** and **♣Clubs** count as the same suit.
    ///
    /// Activates: **N/A**
    SmearedJoker,

    /// Copies the ability of Joker to the right.
    ///
    /// Activates: **Same as copied**
    Blueprint,
    // Skipped: Most of them!
}

/// Editions give [`Card`]s a bonus when scoring them.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Sequence)]
pub enum Enhancement {
    /// Bonus card: **+30 Chips**.
    Bonus,

    /// Mult card: **+4 Mult**.
    Mult,

    /// Wild card: Is considered to be every suit simultaneously.
    Wild,

    /// Glass card: **x2 Mult**.
    //  Skipped: 1 in 4 chance to destroy card after all scoring is finished.
    Glass,

    /// Steel card: **x1.5 Mult** when this card stays in hand.
    Steel,
    // Skipped: Gold, Lucky, Stone
}

/// Editions give [`Card`]s and [`Joker`]s a bonus when scoring them.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Sequence)]
pub enum Edition {
    /// **+50 Chips**.
    Foil,

    /// **+10 Mult**.
    Holographic,

    /// **x1.5 Mult**.
    Polychrome,
    // Skipped: Negative
}

/// Poker Hands are sets of between one and five cards that can be played in Ortalab
/// to obtain **Chips** and **Mult** for scoring.
///
/// This includes all of the standard poker hands, along with three "illegal" poker hands.
/// These hands are possible in Ortalab because players can have duplicate cards,
/// enabling hands that aren't typically possible in poker.
///
/// Higher tier hands take precedence over lower tier hands regardless of their level or scoring.
/// e.g., if your hand is K♦ K♦ K♦ K♦ 2♦, the hand will always be a Four of a Kind and never a Flush.
///
/// `PokerHand` is declared in ascending tier order. You can read the tier numerically as follows:
///
/// ```
/// # use ortalib::PokerHand;
///
/// assert_eq!(PokerHand::HighCard as u8,   0);
/// assert_eq!(PokerHand::Pair as u8,       1);
/// // ...
/// assert_eq!(PokerHand::Flush as u8,      5);
/// // ...
/// assert_eq!(PokerHand::FlushHouse as u8, 10);
/// assert_eq!(PokerHand::FlushFive as u8,  11);
/// ```
///
/// Poker hands also include a base score,
/// which provides initial values for the round's **Chips** and **Mult**.
/// These base scores can be accessed with [`PokerHand::hand_value`] as follows:
///
/// ```
/// # use ortalib::PokerHand;
///
/// let (chips, mult) = PokerHand::Flush.hand_value();
/// assert_eq!(chips, 35.0);
/// assert_eq!(mult, 4.0);
/// ```
///
/// Only the card relevant to the hand are scored. All others are unscored.
///
/// Example 1:
/// You play an ace high with 4 other cards.
/// Only the High card base amount and the aces values are used for this hands score.
/// The other cards are ignored.
///
/// Example 2:
/// You play a pair of 3s and A K Q.
/// Only the pair cards are scored.
/// The other cards ignored.
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Sequence)]
pub enum PokerHand {
    /// When no other hand is possible, the one highest card in your hand.
    /// Aces are considered the highest card.
    ///
    /// e.g. Q♦ 9♥ A♠ 3♥ 4♠ (Q♦ 9♥ 3♥ 4♠ not counted)
    HighCard = 0,

    /// Two cards with a matching rank. Suits may differ.
    ///
    /// e.g. K♠ 9♠ 9♦ 6♥ 3♥ (K♠ 6♥ 3♥ not counted)
    Pair,

    /// Two cards with a matching rank, and two cards with any other matching rank.
    /// Suits may differ.
    ///
    /// e.g. Q♣ Q♦ A♠ 4♦ 4♠ (A♠ not counted)
    TwoPair,

    /// Three cards with a matching rank. Suits may differ.
    ///
    /// e.g. 9♣ 9♦ 9♠ A♠ 3♦ (A♠ 3♦ not counted)
    ThreeOfAKind,

    /// Five cards in consecutive order which are not all from the same suit.
    /// Aces can be counted high or low, but not both at once.
    ///
    /// e.g. A♠ 2♦ 3♣ 4♠ 5♥ and 10♦ J♠ Q♦ K♣ A♠ are straights,
    ///      but Q♦ K♣ A♠ 2♣ 3♠ is not.
    Straight,

    /// Five cards of any rank, all from a single suit.
    ///
    /// e.g. A♥ K♥ 9♥ 5♥ 4♥
    Flush,

    /// Three cards with a matching rank, and two cards with any other matching rank,
    /// with cards from two or more suits.
    ///
    /// e.g. K♥ K♦ K♣ 2♥ 2♣
    FullHouse,

    /// Four cards with a matching rank. Suits may differ.
    ///
    /// e.g. J♠ J♣ J♥ J♦ 3♣ (3♣ not counted)
    FourOfAKind,

    /// Five cards in consecutive order, all from a single suit.
    ///
    /// e.g. Q♠ J♠ 10♠ 9♠ 8♠
    StraightFlush,

    // "Illegal" poker hands
    /// An "illegal" hand.
    ///
    /// Five cards with the same rank which are not all the same suit.
    ///
    /// e.g. A♠ A♥ A♥ A♣ A♠
    FiveOfAKind,

    /// An "illegal" hand.
    ///
    /// Three cards with the same rank, and two cards with the same rank,
    /// all from a single suit.
    ///
    /// e.g. 7♦ 7♦ 7♦ 4♦ 4♦
    FlushHouse,

    /// An "illegal" hand.
    ///
    /// Five cards with the same rank and same suit.
    ///
    /// e.g. A♠ A♠ A♠ A♠ A♠
    FlushFive,
}

impl Card {
    pub fn new(
        rank: Rank,
        suit: Suit,
        enhancement: Option<Enhancement>,
        edition: Option<Edition>,
    ) -> Self {
        static UNIQUE_INDEX: AtomicUsize = AtomicUsize::new(0);

        Self {
            rank,
            suit,
            enhancement,
            edition,
            unique_index: UNIQUE_INDEX.fetch_add(1, Ordering::SeqCst),
        }
    }
}

impl Serialize for Card {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Card {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CardVisitor;
        impl Visitor<'_> for CardVisitor {
            type Value = Card;

            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                write!(f, "Card")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.parse().map_err(|err| serde::de::Error::custom(err))
            }
        }

        deserializer.deserialize_str(CardVisitor)
    }
}

impl Serialize for JokerCard {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for JokerCard {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct CardVisitor;
        impl Visitor<'_> for CardVisitor {
            type Value = JokerCard;

            fn expecting(&self, f: &mut Formatter) -> std::fmt::Result {
                write!(f, "JokerCard")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.parse().map_err(|err| serde::de::Error::custom(err))
            }
        }

        deserializer.deserialize_str(CardVisitor)
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Self { rank, suit, .. } = self;

        write!(f, "{rank}{suit}")?;

        if let Some(enhancement) = self.enhancement {
            write!(f, " {enhancement}")?;
        }

        if let Some(edition) = self.edition {
            write!(f, " {edition}")?;
        }

        Ok(())
    }
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();

        let rank_suit = parts.next().ok_or("Cannot parse empty string")?;

        let enhancement_or_edition_str = parts.next();
        let edition_str = parts.next();

        if let Some(invalid) = parts.next() {
            return Err(format!("Card `{s}` contains too much data: `{invalid}`"));
        }

        // Suit is always exactly 1 char
        let mut reversed = rank_suit.chars().rev();
        let suit_str = reversed
            .next()
            .ok_or_else(|| format!("Card `{s}` missing rank / suit"))?
            .to_string();
        let rank_str: String = reversed.rev().collect();

        let rank = rank_str
            .parse()
            .map_err(|err| format!("Card `{s}` has invalid rank: {err}"))?;

        let suit = suit_str
            .parse()
            .map_err(|err| format!("Card `{s}` has invalid suit: {err}"))?;

        let (enhancement, edition) = {
            let edition = edition_str
                .map(|edition| edition.parse())
                .transpose()
                .map_err(|err| format!("Card `{s}` has invalid edition: {err}"))?;

            if edition.is_some() {
                let enhancement = enhancement_or_edition_str
                    .map(|enhancement| enhancement.parse())
                    .transpose()
                    .map_err(|err| format!("Card `{s}` has invalid enhancement: {err}"))?;

                (enhancement, edition)
            } else if let Some(enhancement_or_edition_str) = enhancement_or_edition_str {
                let enhancement = enhancement_or_edition_str.parse::<Enhancement>();
                let edition = enhancement_or_edition_str.parse::<Edition>();

                match (enhancement, edition) {
                    (Ok(_), Ok(_)) => unreachable!("Enhancements and editions have distinct names"),
                    (Ok(enhancement), _) => (Some(enhancement), None),
                    (Err(_), Ok(edition)) => (None, Some(edition)),
                    (Err(_), Err(_)) => {
                        return Err(format!(
                            "Card `{s}` has invalid enhancement / edition: {enhancement_or_edition_str}"
                        ));
                    }
                }
            } else {
                (None, None)
            }
        };

        Ok(Card::new(rank, suit, enhancement, edition))
    }
}

impl JokerCard {
    pub fn new(joker: Joker, edition: Option<Edition>) -> Self {
        static UNIQUE_INDEX: AtomicUsize = AtomicUsize::new(0);

        Self {
            joker,
            edition,
            unique_index: UNIQUE_INDEX.fetch_add(1, Ordering::SeqCst),
        }
    }
}

impl Debug for JokerCard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for JokerCard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let joker = self.joker;

        write!(f, "{joker}")?;

        if let Some(edition) = self.edition {
            write!(f, " {edition}")?;
        }

        Ok(())
    }
}

impl FromStr for JokerCard {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut joker_str = s;
        let mut edition = None;

        for possible_edition in all::<Edition>() {
            if let Some(leftover) = s.strip_suffix(&possible_edition.to_string()) {
                joker_str = leftover.trim();
                edition = Some(possible_edition);

                break;
            }
        }

        let joker = joker_str
            .parse()
            .map_err(|err| format!("Invalid JokerCard `{s}`: {err}"))?;

        Ok(JokerCard::new(joker, edition))
    }
}

impl Debug for Joker {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for Joker {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use crate::Joker::*;

        #[rustfmt::skip]
        let name = match self {
            Joker =>           "Joker",
            JollyJoker =>      "Jolly Joker",
            ZanyJoker =>       "Zany Joker",
            MadJoker =>        "Mad Joker",
            CrazyJoker =>      "Crazy Joker",
            DrollJoker =>      "Droll Joker",
            SlyJoker =>        "Sly Joker",
            WilyJoker =>       "Wily Joker",
            CleverJoker =>     "Clever Joker",
            DeviousJoker =>    "Devious Joker",
            CraftyJoker =>     "Crafty Joker",
            AbstractJoker =>   "Abstract Joker",
            RaisedFist =>      "Raised Fist",
            Blackboard =>      "Blackboard",
            Baron =>           "Baron",
            GreedyJoker =>     "Greedy Joker",
            LustyJoker =>      "Lusty Joker",
            WrathfulJoker =>   "Wrathful Joker",
            GluttonousJoker => "Gluttonous Joker",
            Fibonacci =>       "Fibonacci",
            ScaryFace =>       "Scary Face",
            EvenSteven =>      "Even Steven",
            OddTodd =>         "Odd Todd",
            Photograph =>      "Photograph",
            SmileyFace =>      "Smiley Face",
            FlowerPot =>       "Flower Pot",
            FourFingers =>     "Four Fingers",
            Shortcut =>        "Shortcut",
            Mime =>            "Mime",
            Pareidolia =>      "Pareidolia",
            Splash =>          "Splash",
            SockAndBuskin =>   "Sock And Buskin",
            SmearedJoker =>    "Smeared Joker",
            Blueprint =>       "Blueprint",
        };

        write!(f, "{name}")
    }
}

impl FromStr for Joker {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use crate::Joker::*;

        #[rustfmt::skip]
        let value = match s {
            "Joker" =>            Joker,
            "Jolly Joker" =>      JollyJoker,
            "Zany Joker" =>       ZanyJoker,
            "Mad Joker" =>        MadJoker,
            "Crazy Joker" =>      CrazyJoker,
            "Droll Joker" =>      DrollJoker,
            "Sly Joker" =>        SlyJoker,
            "Wily Joker" =>       WilyJoker,
            "Clever Joker" =>     CleverJoker,
            "Devious Joker" =>    DeviousJoker,
            "Crafty Joker" =>     CraftyJoker,
            "Abstract Joker" =>   AbstractJoker,
            "Raised Fist" =>      RaisedFist,
            "Blackboard" =>       Blackboard,
            "Baron" =>            Baron,
            "Greedy Joker" =>     GreedyJoker,
            "Lusty Joker" =>      LustyJoker,
            "Wrathful Joker" =>   WrathfulJoker,
            "Gluttonous Joker" => GluttonousJoker,
            "Fibonacci" =>        Fibonacci,
            "Scary Face" =>       ScaryFace,
            "Even Steven" =>      EvenSteven,
            "Odd Todd" =>         OddTodd,
            "Photograph" =>       Photograph,
            "Smiley Face" =>      SmileyFace,
            "Flower Pot" =>       FlowerPot,
            "Four Fingers" =>     FourFingers,
            "Shortcut" =>         Shortcut,
            "Mime" =>             Mime,
            "Pareidolia" =>       Pareidolia,
            "Splash" =>           Splash,
            "Sock And Buskin" =>  SockAndBuskin,
            "Smeared Joker" =>    SmearedJoker,
            "Blueprint" =>        Blueprint,
            _ => return Err(format!("Invalid Joker: `{s}`")),
        };

        Ok(value)
    }
}

impl Debug for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for Rank {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Rank::*;

        write!(f, "{}", match *self {
            Two =>   "2",
            Three => "3",
            Four =>  "4",
            Five =>  "5",
            Six =>   "6",
            Seven => "7",
            Eight => "8",
            Nine =>  "9",
            Ten =>   "10",
            Jack =>  "J",
            Queen => "Q",
            King =>  "K",
            Ace =>   "A",
        })
    }
}

impl FromStr for Rank {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Rank::*;

        #[rustfmt::skip]
        let value = match s {
            "2" =>  Two,
            "3" =>  Three,
            "4" =>  Four,
            "5" =>  Five,
            "6" =>  Six,
            "7" =>  Seven,
            "8" =>  Eight,
            "9" =>  Nine,
            "10" => Ten,
            "J" =>  Jack,
            "Q" =>  Queen,
            "K" =>  King,
            "A" =>  Ace,
            _ => return Err(format!("Invalid Rank: `{s}`")),
        };

        Ok(value)
    }
}

impl Debug for Suit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for Suit {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Suit::*;

        write!(f, "{}", match *self {
            Spades =>   "♠",
            Hearts =>   "♥",
            Clubs =>    "♣",
            Diamonds => "♦",
        })
    }
}

impl FromStr for Suit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Suit::*;

        let value = match s {
            "♠" => Spades,
            "♥" => Hearts,
            "♣" => Clubs,
            "♦" => Diamonds,
            _ => return Err(format!("Invalid Suit: `{s}`")),
        };

        Ok(value)
    }
}

impl Debug for Enhancement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for Enhancement {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Enhancement::*;

        write!(f, "{}", match *self {
            Bonus => "Bonus",
            Mult =>  "Mult",
            Wild =>  "Wild",
            Glass => "Glass",
            Steel => "Steel",
        })
    }
}

impl FromStr for Enhancement {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Enhancement::*;

        #[rustfmt::skip]
        let value = match s {
            "Bonus" => Bonus,
            "Mult" =>  Mult,
            "Wild" =>  Wild,
            "Glass" => Glass,
            "Steel" => Steel,
            _ => return Err(format!("Invalid Enhancement: `{s}`")),
        };

        Ok(value)
    }
}

impl Debug for Edition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for Edition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Edition::*;

        write!(
            f,
            "{}",
            match *self {
                Foil => "Foil",
                Holographic => "Holographic",
                Polychrome => "Polychrome",
            }
        )
    }
}

impl FromStr for Edition {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Edition::*;

        let value = match s {
            "Foil" => Foil,
            "Holographic" => Holographic,
            "Polychrome" => Polychrome,
            _ => return Err(format!("Invalid Edition: `{s}`")),
        };

        Ok(value)
    }
}

impl Debug for PokerHand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl Display for PokerHand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use PokerHand::*;

        write!(
            f,
            "{}",
            match self {
                HighCard => "High Card",
                Pair => "Pair",
                TwoPair => "Two Pair",
                ThreeOfAKind => "Three Of A Kind",
                Straight => "Straight",
                Flush => "Flush",
                FullHouse => "Full House",
                FourOfAKind => "Four Of A Kind",
                StraightFlush => "Straight Flush",
                FiveOfAKind => "Five Of A Kind",
                FlushHouse => "Flush House",
                FlushFive => "Flush Five",
            }
        )
    }
}

impl Rank {
    pub fn rank_value(&self) -> Chips {
        use Rank::*;

        let value = match *self {
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
            Six => 6,
            Seven => 7,
            Eight => 8,
            Nine => 9,
            Ten | Jack | Queen | King => 10,
            Ace => 11,
        };

        value.into()
    }

    /// Returns true if the rank is a face card (Jack, Queen, or King).
    pub fn is_face(&self) -> bool {
        use Rank::*;
        matches!(self, Jack | Queen | King)
    }
}

impl PokerHand {
    pub fn hand_value(&self) -> (Chips, Mult) {
        use PokerHand::*;

        #[rustfmt::skip]
        let (chips, mult) = match *self {
            HighCard =>      (5,   1),
            Pair =>          (10,  2),
            TwoPair =>       (20,  2),
            ThreeOfAKind =>  (30,  3),
            Straight =>      (30,  4),
            Flush =>         (35,  4),
            FullHouse =>     (40,  4),
            FourOfAKind =>   (60,  7),
            StraightFlush => (100, 8),
            FiveOfAKind =>   (120, 12),
            FlushHouse =>    (140, 14),
            FlushFive =>     (160, 16),
        };

        (chips.into(), mult.into())
    }
}

impl Suit {
    pub fn color(&self) -> SuitColor {
        use Suit::*;
        use SuitColor::*;

        match *self {
            Spades | Clubs => Black,
            Hearts | Diamonds => Red,
        }
    }

    pub fn other_suit_of_same_color(&self) -> Suit {
        use Suit::*;

        match *self {
            Spades => Clubs,
            Hearts => Diamonds,
            Clubs => Spades,
            Diamonds => Hearts,
        }
    }
}

impl SuitColor {
    pub fn other_color(&self) -> SuitColor {
        use SuitColor::*;

        match *self {
            Black => Red,
            Red => Black,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use enum_iterator::all;

    #[test]
    fn jokers_to_string_parse() {
        for joker in all::<Joker>() {
            assert_eq!(Ok(joker), joker.to_string().parse())
        }
    }

    #[test]
    fn suits_to_string_parse() {
        for suit in all::<Suit>() {
            assert_eq!(Ok(suit), suit.to_string().parse())
        }
    }

    #[test]
    fn ranks_to_string_parse() {
        for rank in all::<Rank>() {
            assert_eq!(Ok(rank), rank.to_string().parse())
        }
    }

    #[test]
    fn rank_is_face() {
        use Rank::*;

        // Test face cards
        let face_cards = [Jack, Queen, King];
        for card in face_cards {
            assert!(card.is_face(), "{card:?} should be a face card");
        }

        // Test non-face cards
        let non_face_cards = [Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten];
        for card in non_face_cards {
            assert!(!card.is_face(), "{card:?} should not be a face card");
        }
    }

    #[test]
    fn enhancements_to_string_parse() {
        for enhancement in all::<Enhancement>() {
            assert_eq!(Ok(enhancement), enhancement.to_string().parse())
        }
    }

    #[test]
    fn editions_to_string_parse() {
        for edition in all::<Edition>() {
            assert_eq!(Ok(edition), edition.to_string().parse())
        }
    }

    #[test]
    fn serialize_round() {
        use crate::Joker::*;
        use Edition::*;
        use Enhancement::*;
        use Rank::*;
        use Suit::*;

        #[rustfmt::skip]
        let round = Round {
            cards_played: vec![
                Card::new(Ace,   Hearts,   None,        None),
                Card::new(Queen, Clubs,    Some(Bonus), None),
                Card::new(Ten,   Spades,   None,        Some(Holographic)),
                Card::new(Seven, Diamonds, Some(Glass), Some(Polychrome)),
            ],
            cards_held_in_hand: vec![
                Card::new(Ace, Clubs, None, None),
            ],
            jokers: vec![
                JokerCard::new(Joker,         None),
                JokerCard::new(Blueprint,     Some(Foil)),
                JokerCard::new(Photograph,    Some(Holographic)),
                JokerCard::new(SockAndBuskin, Some(Polychrome)),
                JokerCard::new(FlowerPot,     Some(Polychrome)),
            ],
        };

        let serialized = serde_yaml::to_string(&round);
        assert!(serialized.is_ok(), "{serialized:?}");
        let serialized = serialized.unwrap();

        let expected = r#"
cards_played:
- A♥
- Q♣ Bonus
- 10♠ Holographic
- 7♦ Glass Polychrome
cards_held_in_hand:
- A♣
jokers:
- Joker
- Blueprint Foil
- Photograph Holographic
- Sock And Buskin Polychrome
- Flower Pot Polychrome
"#
        .trim_start();

        assert_eq!(serialized, expected);
    }

    #[test]
    fn deserialize_round() {
        use crate::Joker::*;
        use Edition::*;
        use Enhancement::*;
        use Rank::*;
        use Suit::*;

        let serialized = r#"
cards_played:
- A♥
- Q♣ Bonus
- 10♠ Holographic
- 7♦ Glass Polychrome
cards_held_in_hand:
- A♣
jokers:
- Joker
- Blueprint Foil
- Photograph Holographic
- Sock And Buskin Polychrome
- Flower Pot Polychrome
"#
        .trim_start();

        #[rustfmt::skip]
        #[allow(unused)]
        let expected = Round {
            cards_played: vec![
                Card::new(Ace,   Hearts,   None,        None),
                Card::new(Queen, Clubs,    Some(Bonus), None),
                Card::new(Ten,   Spades,   None,        Some(Holographic)),
                Card::new(Seven, Diamonds, Some(Glass), Some(Polychrome)),
            ],
            cards_held_in_hand: vec![
                Card::new(Ace, Clubs, None, None),
            ],
            jokers: vec![
                JokerCard::new(Joker,         None),
                JokerCard::new(Blueprint,     Some(Foil)),
                JokerCard::new(Photograph,    Some(Holographic)),
                JokerCard::new(SockAndBuskin, Some(Polychrome)),
                JokerCard::new(FlowerPot,     Some(Polychrome)),
            ],
        };

        let deserialized = serde_yaml::from_str::<Round>(serialized);
        assert!(deserialized.is_ok(), "{deserialized:?}");
        let deserialized = deserialized.unwrap();

        let reserialized = serde_yaml::to_string(&deserialized);
        assert!(reserialized.is_ok(), "{reserialized:?}");
        let reserialized = reserialized.unwrap();

        assert_eq!(serialized, reserialized);
    }

    #[test]
    fn test_card_serialization() {
        let card = Card::new(
            Rank::Ace,
            Suit::Hearts,
            Some(Enhancement::Bonus),
            Some(Edition::Foil),
        );
        let serialized = serde_yaml::to_string(&card).unwrap();
        assert_eq!(serialized, "A♥ Bonus Foil\n");
    }

    #[test]
    fn test_card_deserialization() {
        let serialized = "A♥ Bonus Foil";
        let card: Card = serde_yaml::from_str(serialized).unwrap();
        let expected = Card::new(
            Rank::Ace,
            Suit::Hearts,
            Some(Enhancement::Bonus),
            Some(Edition::Foil),
        );
        assert_eq!(
            format!("{card:?}").trim(),
            format!("{expected:?}").trim(),
            "Deserialization failed: expected {:?}, got {:?}",
            expected,
            card
        );
    }

    #[test]
    fn test_joker_card_serialization() {
        let joker_card = JokerCard::new(Joker::JollyJoker, Some(Edition::Holographic));
        let serialized = serde_yaml::to_string(&joker_card).unwrap();
        assert_eq!(serialized, "Jolly Joker Holographic\n");
    }

    #[test]
    fn test_joker_card_deserialization() {
        let serialized = "Jolly Joker Holographic";
        let joker_card: JokerCard = serde_yaml::from_str(serialized).unwrap();
        let expected = JokerCard::new(Joker::JollyJoker, Some(Edition::Holographic));
        assert_eq!(
            format!("{joker_card:?}").trim(),
            format!("{expected:?}").trim(),
            "Deserialization failed: expected {:?}, got {:?}",
            expected,
            joker_card
        );
    }

    #[test]
    fn test_round_trip_card() {
        let card = Card::new(Rank::King, Suit::Diamonds, None, None);
        let serialized = serde_yaml::to_string(&card).unwrap();
        let deserialized: Card = serde_yaml::from_str(&serialized).unwrap();
        assert_eq!(
            format!("{card:?}").trim(),
            format!("{deserialized:?}").trim(),
            "Round-trip failed: expected {:?}, got {:?}",
            card,
            deserialized
        );
    }

    #[test]
    fn test_round_trip_joker_card() {
        let joker_card = JokerCard::new(Joker::CraftyJoker, None);
        let serialized = serde_yaml::to_string(&joker_card).unwrap();
        let deserialized: JokerCard = serde_yaml::from_str(&serialized).unwrap();
        assert_eq!(
            format!("{joker_card:?}").trim(),
            format!("{deserialized:?}").trim(),
            "Round-trip failed: expected {:?}, got {:?}",
            joker_card,
            deserialized
        );
    }

    #[test]
    fn test_invalid_card_deserialization() {
        let serialized = "Invalid Card";
        let result: Result<Card, _> = serde_yaml::from_str(serialized);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_joker_card_deserialization() {
        let serialized = "Invalid JokerCard";
        let result: Result<JokerCard, _> = serde_yaml::from_str(serialized);
        assert!(result.is_err());
    }

    #[test]
    fn test_card_serialization_various_cases() {
        let cases = vec![
            (Rank::Two, Suit::Hearts, None, None, "2♥\n"),
            (
                Rank::Three,
                Suit::Clubs,
                Some(Enhancement::Mult),
                None,
                "3♣ Mult\n",
            ),
            (
                Rank::Four,
                Suit::Diamonds,
                None,
                Some(Edition::Foil),
                "4♦ Foil\n",
            ),
            (
                Rank::Five,
                Suit::Spades,
                Some(Enhancement::Bonus),
                Some(Edition::Holographic),
                "5♠ Bonus Holographic\n",
            ),
            (
                Rank::Ace,
                Suit::Hearts,
                Some(Enhancement::Wild),
                Some(Edition::Polychrome),
                "A♥ Wild Polychrome\n",
            ),
        ];

        for (rank, suit, enhancement, edition, expected) in cases {
            let card = Card::new(rank, suit, enhancement, edition);
            let serialized = serde_yaml::to_string(&card).unwrap();
            assert_eq!(serialized, expected);
        }
    }

    #[test]
    fn test_card_deserialization_various_cases() {
        let cases = vec![
            ("2♥", Card::new(Rank::Two, Suit::Hearts, None, None)),
            (
                "3♣ Mult",
                Card::new(Rank::Three, Suit::Clubs, Some(Enhancement::Mult), None),
            ),
            (
                "4♦ Foil",
                Card::new(Rank::Four, Suit::Diamonds, None, Some(Edition::Foil)),
            ),
            (
                "5♠ Bonus Holographic",
                Card::new(
                    Rank::Five,
                    Suit::Spades,
                    Some(Enhancement::Bonus),
                    Some(Edition::Holographic),
                ),
            ),
            (
                "A♥ Steel Polychrome",
                Card::new(
                    Rank::Ace,
                    Suit::Hearts,
                    Some(Enhancement::Steel),
                    Some(Edition::Polychrome),
                ),
            ),
        ];

        for (serialized, expected_card) in cases {
            let card: Card = serde_yaml::from_str(serialized).unwrap();
            assert_eq!(
                format!("{card:?}").trim(),
                format!("{expected_card:?}").trim(),
                "Deserialization failed for {}: expected {:?}, got {:?}",
                serialized,
                expected_card,
                card
            );
        }
    }

    #[test]
    fn test_joker_card_serialization_various_cases() {
        let cases = vec![
            (Joker::Joker, None, "Joker\n"),
            (Joker::JollyJoker, Some(Edition::Foil), "Jolly Joker Foil\n"),
            (
                Joker::ZanyJoker,
                Some(Edition::Holographic),
                "Zany Joker Holographic\n",
            ),
            (
                Joker::MadJoker,
                Some(Edition::Polychrome),
                "Mad Joker Polychrome\n",
            ),
            (Joker::CrazyJoker, None, "Crazy Joker\n"),
        ];

        for (joker, edition, expected) in cases {
            let joker_card = JokerCard::new(joker, edition);
            let serialized = serde_yaml::to_string(&joker_card).unwrap();
            assert_eq!(serialized, expected);
        }
    }

    #[test]
    fn test_joker_card_deserialization_various_cases() {
        let cases = vec![
            ("Joker", JokerCard::new(Joker::Joker, None)),
            (
                "Jolly Joker Foil",
                JokerCard::new(Joker::JollyJoker, Some(Edition::Foil)),
            ),
            (
                "Zany Joker Holographic",
                JokerCard::new(Joker::ZanyJoker, Some(Edition::Holographic)),
            ),
            (
                "Mad Joker Polychrome",
                JokerCard::new(Joker::MadJoker, Some(Edition::Polychrome)),
            ),
            ("Crazy Joker", JokerCard::new(Joker::CrazyJoker, None)),
        ];

        for (serialized, expected_joker_card) in cases {
            let joker_card: JokerCard = serde_yaml::from_str(serialized).unwrap();
            assert_eq!(
                format!("{joker_card:?}").trim(),
                format!("{expected_joker_card:?}").trim(),
                "Deserialization failed for {}: expected {:?}, got {:?}",
                serialized,
                expected_joker_card,
                joker_card
            );
        }
    }

    #[test]
    fn test_invalid_card_deserialization_cases() {
        let invalid_cases = vec![
            "Invalid Card",
            "2X",
            "3♣ Unknown",
            "4♦ Mult Extra",
            "5♠ Bonus Holographic Extra",
        ];

        for serialized in invalid_cases {
            let result: Result<Card, _> = serde_yaml::from_str(serialized);
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_invalid_joker_card_deserialization_cases() {
        let invalid_cases = vec![
            "Invalid JokerCard",
            "Jolly Joker Unknown",
            "Zany Joker Holographic Extra",
            "Mad Joker Polychrome Extra",
        ];

        for serialized in invalid_cases {
            let result: Result<JokerCard, _> = serde_yaml::from_str(serialized);
            assert!(result.is_err());
        }
    }
}
