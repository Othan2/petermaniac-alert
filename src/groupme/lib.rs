#![warn(missing_docs, missing_debug_implementations, missing_copy_implementations, trivial_casts,
        trivial_numeric_casts, unsafe_code, unstable_features, unused_import_braces,
        unused_qualifications)]
extern crate reqwest;
extern crate serde;
extern crate serde_json;

pub use self::groupme::Groupme;
pub use self::bot::Bot;
pub use self::error::GroupmeError;
pub use self::bot_builder::BotBuilder;

mod groupme;
mod bot;
mod error;
mod client;
mod bot_builder;
