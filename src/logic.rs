use crate::{cards::*, inputs::*};

use reqwest::blocking::get;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::*, result::Result};

struct Player {
    name: String,
    hand: Vec<Card>,
    chips: u32,
    folded: bool,
}

impl Player {
    fn new() -> Player {
        Player {
            name: get_string("What is your name?"),
            hand: Vec::new(),
            chips: 5000,
            folded: false,
        }
    }

    fn bet(&mut self, current_bet: u32) -> u32 {
        loop {
            println!(
                "Your cards: {}",
                self.hand
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            );
            println!(
                "Your chips: {}",
                self.chips.to_string().chars().collect::<String>()
            );

            let input = get_string("Would you like to fold, check, call, or raise?");
            match input.as_str() {
                "fold" => {
                    println!("{} folds.\n", self.name);
                    self.folded = true;
                    return 0;
                }
                "check" => {
                    if current_bet == 0 {
                        println!("{} checks.\n", self.name);
                        return 0;
                    } else {
                        println!("{} calls.\n", self.name);
                        self.chips -= current_bet;
                        return current_bet;
                    }
                }
                "call" => {
                    if current_bet == 0 {
                        println!("{} checks.\n", self.name);
                        return 0;
                    } else {
                        println!("{} calls.\n", self.name);
                        self.chips -= current_bet;
                        return current_bet;
                    }
                }
                "raise" => {
                    let mut raise = get_int("How much would you like to raise?");
                    if raise < current_bet {
                        println!("You must raise at least {}.", current_bet);
                        raise = get_int("How much would you like to raise?");
                    }
                    println!("{} raises {}.\n", self.name, raise);
                    self.chips -= raise;
                    return raise;
                }
                _ => {
                    println!("Invalid input.");
                }
            }
        }
    }

    fn get_cards(&self) -> String {
        self.hand
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join(",")
            .to_string()
    }
}

pub struct Table {
    pot: u32,
    players: Vec<Player>,
    deck: Vec<Card>,
    pub c_cards: Vec<Card>,
    stage: Stage,
    current_bet: u32,
}

enum Stage {
    Preflop,
    Flop,
    Turn,
    River,
    Showdown,
}

impl Table {
    pub fn new() -> Table {
        let mut table = Table {
            pot: 0,
            players: Vec::with_capacity(9),
            deck: new_deck(),
            c_cards: Vec::with_capacity(5),
            stage: Stage::River,
            current_bet: 0,
        };

        let players = get_int("How many players?");

        for _ in 0..players {
            table.players.push(Player::new());
        }

        println!(
            "Table Created. Players: {}\n",
            table
                .players
                .iter()
                .map(|p| p.name.clone())
                .collect::<Vec<String>>()
                .join(", ")
        );

        table.next_round();
        table
    }

    fn deal(&mut self) {
        for _ in 0..2 {
            for player in self.players.iter_mut() {
                player.hand.push(self.deck.pop().unwrap());
            }
        }
    }

    pub fn flop(&mut self) {
        self.current_bet = 0;
        for _ in 0..3 {
            self.c_cards.push(self.deck.pop().unwrap());
        }
    }

    fn turn(&mut self) {
        self.current_bet = 0;
        self.c_cards.push(self.deck.pop().unwrap());
    }

    fn river(&mut self) {
        self.current_bet = 0;
        self.c_cards.push(self.deck.pop().unwrap());
    }

    pub fn next_round(&mut self) {
        self.pot = 0;
        self.current_bet = 0;
        self.c_cards.clear();
        self.current_bet = 50;

        for player in self.players.iter_mut() {
            player.hand.clear();
            player.folded = false;
        }

        self.stage = Stage::Preflop;
        self.deal();
    }

    fn next_stage(&mut self) {
        match self.stage {
            Stage::Preflop => {
                println!("\nFlop:\n");
                self.flop();
                self.stage = Stage::Flop;
            }
            Stage::Flop => {
                println!("\nTurn:\n");
                self.turn();
                self.stage = Stage::Turn;
            }
            Stage::Turn => {
                println!("\nRiver:\n");
                self.river();
                self.stage = Stage::River;
            }
            Stage::River => {
                println!("\nShowdown:\n");
                self.stage = Stage::Showdown;
                self.next_stage();
            }
            Stage::Showdown => {
                self.check_hands();
                self.next_round();
            }
        }
    }

    fn take_bets(&mut self) {
        let mut bets: Vec<Option<u32>> = Vec::new();
        for _ in 0..self.players.len() {
            bets.push(None);
        }
        let max_index = self.players.len() - 1;
        let mut index = 0;

        while bets.iter().any(|&b| b.unwrap_or(9999) != self.current_bet) {
            println!("{}", self);
            println!("{}'s turn.", self.players[index].name);

            let bet = self.players[index].bet(self.current_bet);

            if bet > self.current_bet {
                self.current_bet = bet;
            }

            self.pot += self.current_bet;
            bets[index] = Some(self.current_bet);

            if index == max_index {
                index = 0;
            } else {
                index += 1;
            }

            println!("------------------------------------------\n");
        }
    }

    pub fn play_game(&mut self) {
        loop {
            self.take_bets();
            self.next_stage();
            if self.players.iter().all(|p| p.folded) {
                break;
            }
        }
    }

    fn check_hands(&mut self) {
        let cc = format!(
            "?cc={}",
            self.c_cards
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
        let mut pc = String::new();

        for player in &self.players {
            pc.push_str(format!("&pc[]={}", player.get_cards()).as_str());
        }

        let url = String::from("https://api.pokerapi.dev/v1/winner/texas_holdem") + &cc + &pc;

        println!("{}", &url);

        let result = get(&url).unwrap().json::<ApiResponse>().unwrap();

        let winning_cards = result
            .winners
            .iter()
            .map(|w| (w.cards.clone(), w.result.clone(), w.hand.clone()))
            .collect::<Vec<(String, String, String)>>();

        for player in self.players.iter_mut() {
            if winning_cards
                .iter()
                .map(|w| &w.0)
                .any(|c| player.get_cards().as_str() == c)
            {
                player.chips += self.pot;
                println!(
                    "{} wins with a {}\ncards: {}\n",
                    player.name,
                    winning_cards
                        .iter()
                        .find(|w| player.get_cards() == w.0)
                        .unwrap()
                        .1,
                    winning_cards
                        .iter()
                        .find(|w| player.get_cards() == w.0)
                        .unwrap()
                        .2
                );
            }
        }
    }
}
#[derive(Serialize, Deserialize)]
struct PlayerResponse {
    cards: String,
    hand: String,
    result: String,
}

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    winners: Vec<PlayerResponse>,
    players: Vec<PlayerResponse>,
}

impl Display for ApiResponse {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for player in &self.players {
            write!(f, "hand: {}\n result: {}\n", player.hand, player.result)?;
        }
        Ok(())
    }
}

impl Display for Table {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "Pot: {}\n Cards: {}\nCurrent Bet: {}\n",
            self.pot,
            self.c_cards
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join(" "),
            self.current_bet
        )
    }
}
