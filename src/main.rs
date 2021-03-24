use std::env;
use tokio;

mod dtos;
mod info;
mod utils;

pub use info::Info;

const SERVER: &str = "na1.api.riotgames.com";

#[tokio::main]
async fn main() {
    let api_key = env::var("API_KEY").unwrap_or_else(|_| {
        panic!("Please export API_KEY");
    });
    let info = utils::get_info_from_cli();
    let summoner_dto: dtos::SummonerDTO = utils::get_summoner_dto(info, SERVER, api_key)
        .await
        .unwrap();

    println!("{:?}", summoner_dto);
}
