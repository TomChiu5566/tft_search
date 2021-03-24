use env_logger;
use log::debug;
use std::env;
use tokio;

mod dtos;
mod info;
mod utils;

pub use info::Info;

const PLATFORM: &str = "na1.api.riotgames.com";
const REGIONAL: &str = "americas.api.riotgames.com";

#[tokio::main]
async fn main() {
    env_logger::init();

    let api_key = env::var("API_KEY").unwrap_or_else(|_| {
        panic!("Please export API_KEY");
    });
    let info = utils::get_info_from_cli();
    let summoner_dto: dtos::SummonerDTO =
        utils::get_summoner_dto(info.summoner_name.clone(), PLATFORM, api_key.to_string())
            .await
            .unwrap(); // Panic if any error

    let league_entries_dto: dtos::LeagueEntryDTO =
        utils::get_league_entries_dto(summoner_dto.id.clone(), PLATFORM, api_key.to_string())
            .await
            .unwrap();

    let matches = utils::get_matches(
        summoner_dto.puuid.clone(),
        REGIONAL,
        api_key.to_string(),
        info.match_count.clone(),
    )
    .await
    .unwrap();
    assert_eq!(matches.len() as u32, info.match_count);

    debug!("{:?}\n", summoner_dto);
    debug!("{:?}\n", league_entries_dto);
    debug!("{:?}\n", matches);
}
