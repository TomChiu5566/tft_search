use clap::{App, Arg};
use reqwest;
use serde_json;
use std::io::{Error, ErrorKind};

use crate::dtos;
use crate::Info;

pub fn get_info_from_cli() -> Info {
    let matches = App::new("TFT Search Program")
        .version("0.1.0")
        .author("Tom Chiu <tomchiu@argo.ai>")
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

    let res = client.get(&endpoint).send().await?;

    if res.status().is_success() {
        println!("success!");
    } else {
        println!("{}", res.status());
        let msg = format!("Cannot fetch info using endpoint {}", endpoint);
        Err(Box::new(Error::new(ErrorKind::Other, msg)))?;
    }
    let raw_text = res.text().await?;
    let text = to_snake_case(raw_text);
    let summoner_dto: dtos::SummonerDTO = serde_json::from_str(&text)?;
    Ok(summoner_dto)
}

fn to_snake_case(raw_text: String) -> String {
    raw_text
        .replace("accountId", "account_id")
        .replace("profileIconId", "profile_icon_id")
        .replace("revisionDate", "revision_date")
        .replace("summonerLevel", "summoner_level")
}
