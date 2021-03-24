use clap::{App, Arg};
use log::{error, trace};
use reqwest;
use serde_json;
use std::io::{Error, ErrorKind};

use crate::dtos;
use crate::Info;

pub fn get_info_from_cli() -> Info {
    let matches = App::new("TFT Search Program")
        .version("0.1.0")
        .author("Tom Chiu <ms0705718@gmail.com>")
        .about("Simple program to look up TFT summoner's history")
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .takes_value(true)
                .required(false)
                .help("Summoner's name to search"),
        )
        .arg(
            Arg::with_name("match_count")
                .short("m")
                .long("match-count")
                .takes_value(true)
                .required(false)
                .help("Number of matches to retrieve"),
        )
        .get_matches();
    let summoner_name = matches
        .value_of("name")
        .unwrap_or_else(|| "PittsburghSpirit");
    let match_count = matches
        .value_of("match_count")
        .unwrap_or_else(|| "20")
        .parse::<u32>()
        .unwrap();

    Info::new(summoner_name.to_string(), match_count)
}

pub async fn get_summoner_dto(
    summoner_name: String,
    server: &str,
    api_key: String,
) -> Result<dtos::SummonerDTO, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let endpoint = format!(
        "https://{}/tft/summoner/v1/summoners/by-name/{}?api_key={}",
        server, summoner_name, api_key
    );

    let res = call_api(client, endpoint).await?;
    let raw_text = res.text().await?;
    let text = to_snake_case(raw_text);
    let summoner_dto: dtos::SummonerDTO = serde_json::from_str(&text)?;
    Ok(summoner_dto)
}

pub async fn get_league_entries_dto(
    id: String,
    server: &str,
    api_key: String,
) -> Result<dtos::LeagueEntryDTO, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let endpoint = format!(
        "https://{}/tft/league/v1/entries/by-summoner/{}?api_key={}",
        server, id, api_key
    );

    let res = call_api(client, endpoint).await?;
    let raw_text = res.text().await?;
    let text = to_snake_case(raw_text);
    let league_entries_dto: dtos::LeagueEntryDTO = serde_json::from_str(&text)?;
    Ok(league_entries_dto)
}

pub async fn get_matches(
    puuid: String,
    server: &str,
    api_key: String,
    count: u32,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let endpoint = format!(
        "https://{}/tft/match/v1/matches/by-puuid/{}/ids?count={}&api_key={}",
        server, puuid, count, api_key
    );

    let res = call_api(client, endpoint).await?;
    let raw_text = res.text().await?;
    let text = to_snake_case(raw_text);
    let matches: Vec<String> = text
        .trim()
        .split(',')
        .map(|m| &m[1..m.len() - 1])
        .map(String::from)
        .collect();
    Ok(matches)
}

async fn call_api(
    client: reqwest::Client,
    endpoint: String,
) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    let res = client.get(&endpoint).send().await?;

    if res.status().is_success() {
        trace!("{}: success!", endpoint);
    } else {
        error!("{}", res.status());
        let msg = format!("Cannot fetch info using endpoint {}", endpoint);
        Err(Box::new(Error::new(ErrorKind::Other, msg)))?;
    }
    Ok(res)
}

fn to_snake_case(raw_text: String) -> String {
    // Can refactor it to be a general function
    raw_text
        .replace("[", "")
        .replace("]", "")
        .replace("accountId", "account_id")
        .replace("profileIconId", "profile_icon_id")
        .replace("revisionDate", "revision_date")
        .replace("summonerLevel", "summoner_level")
        .replace("leagueId", "league_id")
        .replace("queueType", "queue_type")
        .replace("summonerId", "summoner_id")
        .replace("summonerName", "summoner_name")
        .replace("leaguePoints", "league_points")
        .replace("freshBlood", "fresh_blood")
        .replace("hotStreak", "hot_streak")
}
