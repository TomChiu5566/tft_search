mod utils;
mod info;

pub use info::Info;

fn main() {
    let info = utils::get_info_from_cli();

    println!("{}", info.get_summoner_name());
}