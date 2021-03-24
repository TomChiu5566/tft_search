pub struct Info {
    pub summoner_name: String,
    pub match_count: u32,
}

impl Info {
    pub fn new(summoner_name: String, match_count: u32) -> Info {
        Info {
            summoner_name,
            match_count,
        }
    }
}
