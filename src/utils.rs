use clap::{App, Arg};
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
                .required(true)
                .help("Summoner's name to search"),
        )
        .get_matches();
    let summoner_name = matches.value_of("name").unwrap_or_else(|| "SSomeGuyy");

    Info::new(summoner_name.to_string())
}

pub async fn get_summoner_dto(
    info: Info,
    server: &str,
    api_key: String,
) -> Result<dtos::SummonerDTO, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let endpoint = format!(
        "https://{}/tft/summoner/v1/summoners/by-name/{}?api_key={}",
        server,
        info.get_summoner_name(),
        api_key
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
    let league_entries_dto: dtos::LeagueEntryDTO = serde_json::from_str(&text[1..text.len() - 1])?;
    Ok(league_entries_dto)
}

async fn call_api(
    client: reqwest::Client,
    endpoint: String,
) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
    let res = client.get(&endpoint).send().await?;

    if res.status().is_success() {
        println!("{}: success!", endpoint);
    } else {
        println!("{}", res.status());
        let msg = format!("Cannot fetch info using endpoint {}", endpoint);
        Err(Box::new(Error::new(ErrorKind::Other, msg)))?;
    }
    Ok(res)
}

fn to_snake_case(raw_text: String) -> String {
    // Can refactor it to be a general function
    raw_text
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
