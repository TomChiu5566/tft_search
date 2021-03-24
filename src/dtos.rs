use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SummonerDTO {
    id: String,
    account_id: String,
    puuid: String,
    name: String,
    profile_icon_id: i32,
    revision_date: i64,
    summoner_level: i32,
}
