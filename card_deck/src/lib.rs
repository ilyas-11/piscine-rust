use rand::Rng;


#[derive(Debug, PartialEq)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}


#[derive(Debug, PartialEq)]
pub enum Rank {
    Ace,
    Number(u8),
    Queen,
    Jack,
    King,
}


impl Suit {

    pub fn random() -> Suit {

        let value = rand::thread_rng()
            .gen_range(1..=4);

        Suit::translate(value)
    }


    pub fn translate(value:u8)->Suit {

        match value {

            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            _ => Suit::Club,

        }
    }
}



impl Rank {

    pub fn random()->Rank {

        let value = rand::thread_rng()
            .gen_range(1..=13);

        Rank::translate(value)
    }


    pub fn translate(value:u8)->Rank {

        match value {

            1 => Rank::Ace,

            2..=10 => Rank::Number(value),

            11 => Rank::Jack,

            12 => Rank::Queen,

            _ => Rank::King,

        }
    }
}



#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}



pub fn winner_card(card:&Card)->bool {

    match (&card.suit,&card.rank){

        (Suit::Spade, Rank::Ace)=>true,

        _=>false,

    }

}