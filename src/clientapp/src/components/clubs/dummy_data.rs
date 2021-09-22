use crate::types::*;
use rand::{self, Rng}; // 0.8.0
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;


pub struct DummyData {
    pub club_details: Vec<ClubDetails>,
}

lazy_static! {
    static ref NAMES: Vec<&'static str> = vec![
        "Big Chungus",
        "Hot Dog",
        "Sans Undertale",
        "Cris Koutsegeras",
        "Percival Ulysses Cox",
        "Justin Woodring",
        "ẖ̵̳̳͉̀͌͗̾̏̍̉̕͝͝i̷̘̪͖̯̖̖̜͖͔̪͊ṁ̷̡͍̦̪͖͇̳̻̹̝̥̏̌̈̿̏",
        "Spongebob Squarepants",
        "Aang Avatar",
        "Mr. Volkswagen",
        "A Melted Piece of Chocolate",
        "Ethically Sourced Cruelty",
        "Kuo Pao-Yang",
        "John \"The W stands for W\" Burris",
        "Ashlynn Martell",
        "Brennan Forrest",
        "Link",
        "Nintendo Mario",
        "Nintendo Xbox PS4",
        "Nintendo Luigi",
        "Kit Fisto",
        "Darth Vader",
        "Lego Yoda",
        "Lego Batman",
        "Lego Princess Leia (Slave Outfit)",
    ];

    static ref CLUB_NAMES: Vec<&'static str> = vec! [
        "CIA",
			"FBI",
			"Chess Club",
			"Tennis Club",
			"Ouran High School Host Club",
			"Billy Club",
			"Tentacle Enthusiasts",
			"Seedless Watermelons Against Catholics",
			"B.U.R.P.",
			"Bring Us The Beef",
			"Catholics Against Seedless Watermelons",
			"Bring back the red stick that used to be in the pizza lunchables",
			"Ice Cream Enjoyers",
			"People People",
			"The International Association of Hot Doggers",
			"Cool Guys",
			"Nintendo Fans",
			"Nintendo Fans (Non-Smoking)",
    ];
}

pub fn get_name() -> String {
    String::from((*NAMES)[get_rand(0, (*NAMES).len() as i64) as usize])
}

pub fn get_club_name() -> String {
    String::from((*CLUB_NAMES)[get_rand(0, (*CLUB_NAMES).len() as i64) as usize])
}

pub fn get_body() -> String {
    String::from("Fart")
}

pub fn get_head_mod() -> crate::types::UserDetails  {
    UserDetails {
        email: String::from("ceo@business.net"),
        picture: String::from(""),
        first_name: String::from("Adrian"),
        last_name: String::from("Brody"),
    }
}

pub fn get_rand(low: i64, high: i64) -> i64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(low..high)
}

impl DummyData {
    pub fn new() -> Self {
        let mut details = vec![];

        for i in 0..20 {
            details.push(ClubDetails {
                id: i,
                name: get_name(),
                body: get_body(),
                member_count: get_rand(0, 15),
                publish_date: DateTime::parse_from_rfc2822("Tue, 1 Jul 2003 10:52:37 +0200").unwrap().into(),
                expiry_date: DateTime::parse_from_rfc2822("Tue, 1 Jul 2003 10:52:37 +0200").unwrap().into(),
                is_member: false,
                is_moderator: String::from("false"),
                head_moderator: get_head_mod(),
            });
        }

        DummyData {
            club_details: details,
        }
    }
}