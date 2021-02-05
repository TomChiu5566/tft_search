pub struct Info {
    summoner_name: String,
}

impl Info {
    pub fn new(summoner_name: String) -> Info {
        Info {
            summoner_name
        }
    }

    pub fn get_summoner_name(&self) -> &String {
        &self.summoner_name
    }
}