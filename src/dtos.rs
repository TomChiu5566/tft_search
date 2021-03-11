use serde::Deserialize;

#[derive(Deserialize)]
#[derive(Debug)]
pub struct SummonerDTO {
    id: String,
    account_id: String,
    puuid: String,
    name: String,
    profile_icon_id: i32,
    revision_date: i64,
    summoner_level: i32,
}

impl SummonerDTO {
    pub fn new() -> SummonerDTO {
        SummonerDTO {
            id: "-1".to_string(),
            account_id: "-1".to_string(),
            puuid: "-1".to_string(),
            name: "xxx".to_string(),
            profile_icon_id: -1,
            revision_date: -1,
            summoner_level: -1,
        }
    }
}