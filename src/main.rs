mod member;
use crate::member::AllMemberResponse;

mod config;
use crate::config::get_config;
use crate::config::Config;

use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Config {
        base_url,
        version,
        api_key,
    } = get_config();

    let get_members_url = format!("{0}/{1}/member?api_key={2}", base_url, version, api_key);
    let api = reqwest::Client::new();
    let res = api.get(get_members_url).send().await?;

    println!("{:#?}", res);

    let res_json = res.json::<AllMemberResponse>().await?;

    println!("{:#?}", res_json);
    Ok(())
}
