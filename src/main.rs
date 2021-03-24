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
    let summoner_dto: dtos::SummonerDTO =
        utils::get_summoner_dto(info, SERVER, api_key.to_string())
            .await
            .unwrap(); // Panic if any error

    let league_entries_dto: dtos::LeagueEntryDTO =
        utils::get_league_entries_dto(summoner_dto.id.clone(), SERVER, api_key.to_string())
            .await
            .unwrap();

    println!("{:?}\n", summoner_dto);
    println!("{:?}\n", league_entries_dto);
}
