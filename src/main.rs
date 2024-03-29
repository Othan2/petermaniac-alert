mod groupme;
mod twitter;

pub use groupme::start_groupme_bot;
use clap::Parser;
use twitter::twitter_client::TwitterClient;
use postgres::{NoTls};


/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
struct Args {
    /// The groupme api token
   #[arg(short, long)]
    groupme_token: String,
    /// The twitter api token
   #[arg(short, long)]
    twitter_token: String,
    /// The id of the group for the bot
   #[arg(short, long)]
    group_id: String,
    /// The id of the bot
    #[arg(short, long)]
    bot_id: String,
    /// The postgres connection string. Format: https://www.postgresql.org/docs/current/libpq-connect.html#LIBPQ-PARAMKEYWORDS
    #[arg(short, long)]
    postgres_params: String,
}

fn main() {
    let args = Args::parse();
    env_logger::init();

    let postgres_client: postgres::Client = postgres::Client::connect(&args.postgres_params, NoTls).unwrap();

    let twt_client: TwitterClient = TwitterClient::new(&args.twitter_token, postgres_client);

    start_groupme_bot(
        args.group_id,
        args.groupme_token,
        args.bot_id,
        twt_client
    );
}
