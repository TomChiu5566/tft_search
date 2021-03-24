use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SummonerDTO {
    pub id: String,
    account_id: String,
    puuid: String,
    name: String,
    profile_icon_id: i32,
    revision_date: i64,
    summoner_level: i32,
}

#[derive(Deserialize, Debug)]
pub struct LeagueEntryDTO {
    league_id: String,
    summoner_id: String,
    summoner_name: String,
    queue_type: String,
    tier: String,
    rank: String,
    league_points: i32,
    wins: i32,
    losses: i32,
    hot_streak: bool,
    veteran: bool,
    fresh_blood: bool,
    inactive: bool,
}
