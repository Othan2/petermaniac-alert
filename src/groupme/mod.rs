/*
Want to:
- poll for tweets periodically
- receive incoming petermaniac subscriptions
- send tweets if something meets criteria
     */
// extern crate groupme_bot;

// use groupme_bot::Groupme;
// extern crate twitter;
mod bot;
mod groupme;
mod client;
mod error;
mod bot_builder;
pub use crate::twitter::get_tweet_urls_since;
use chrono::NaiveDate;use std::task::{Context, Poll};
use std::future::Future;
use cooked_waker::{IntoWaker as _, WakeRef};

// use std::{thread, time};

use bot::Bot;
use groupme::Groupme;

// struct NoopWaker;

// impl WakeRef for NoopWaker {
//     fn wake_by_ref(&self) {
//         // It's okay to do nothing only because block_on continually polls
//     }
// }


pub fn start_groupme_bot(group_id: String, token: String) {
    println!("starting groupme bot. id: {} token: {}", group_id, token);

    let gm: Groupme = Groupme::new(Some(&token));
    let bot: Bot = gm
        .create_bot("Petermaniac bot", &group_id)
        .unwrap()
        .create()
        .unwrap();


    let res2: Vec<String> =
        get_tweet_urls_since(NaiveDate::from_ymd_opt(2023, 02, 20).unwrap()).unwrap();
    println!("{:?}", res2);
    let post_res = bot.post(&res2[0]);

    // async {
    //     .await.unwrap()
    //         // Some => println!("posting results {:?}", x.unwrap()),
    //         // _ => println!("oh no!")
        
    // };
    
    println!("{}", post_res.unwrap());

    // post_res.unwrap();
}

// pub fn poll_for_tweets
