use clap::{Arg, App};
use reqwest;
use serde_json;

use crate::Info;
use crate::dtos;

pub fn get_info_from_cli() -> Info {
    let matches = App::new("TFT Search Program")
        .version("0.1.0")
        .author("Tom Chiu <tomchiu@argo.ai>")
        .about("Simple program to look up TFT summoner's history")
        .arg(Arg::with_name("name")
        .short("n")
        .long("name")
        .takes_value(true)
        .required(true)
        .help("Summoner's name to search"))
        .get_matches();
    let summoner_name = matches.value_of("name").unwrap_or_else(|| {"SSomeGuyy"});

    Info::new(summoner_name.to_string())
}

pub async fn get_summoner_dto(info: Info, server: &str, api_key: String)
-> Result<dtos::SummonerDTO, serde_json::Error> {
    let client = reqwest::Client::new();
    let endpoint = format!("https://{}/tft/summoner/v1/summoners/by-name/{}?api_key={}", server, info.get_summoner_name(), api_key);

    println!("Endpoint = {}", endpoint);

    let res = client.get(&endpoint).send().await;
    let res = match res {
        Ok(r) => r,
        Err(err) => return Ok(dtos::SummonerDTO::new()),
    };

    if res.status().is_success() {
        println!("success!");
    } else {
        println!("{}", res.status());
    }
    let raw_text = res.text().await.unwrap_or("".to_string());
    let text = to_snake_case(raw_text);
    println!("{}", text);
 /*
    let summoner_dto: dtos::SummonerDTO = match serde_json::from_str(&text) {
        Ok(_summoner_dto) => _summoner_dto,
        Err(err) => dtos::SummonerDTO::new(),
    };
    */
    let summoner_dto: dtos::SummonerDTO = serde_json::from_str(&text)?;
    Ok(summoner_dto)  // TODO: keep this for now
}

fn to_snake_case(raw_text: String) -> String {
    raw_text
        .replace("accountId", "account_id")
        .replace("profileIconId", "profile_icon_id")
        .replace("revisionDate", "revision_date")
        .replace("summonerLevel", "summoner_level")
}