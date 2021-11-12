use reqwest::Error;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Error> {

    let currency_url = format!("https://bank.gov.ua/NBUStatService/v1/statdirectory/dollar_info?json");
    let currency_response = reqwest::get(&currency_url).await?.json::<serde_json::Value>().await?;    

    let t_id = "UA-212467075-1";
    let c_id = Uuid::new_v4().to_hyphenated().to_string(); 
    let rate = (&currency_response[0]["rate"].as_f64().unwrap() * 10000f64) as u64;
    println!("{:#?}", rate);

    let gaurl = format!("https://www.google-analytics.com/collect?v=1&t=event&tid={tId}&cid={cId}&ev={rate}&ec=currency&ea=update&el=rate", tId = &t_id, cId = &c_id, rate = &rate);

    let gamp_response = reqwest::get(&gaurl).await?;
    if gamp_response.status().is_success() {
        println!("success!")
    } else {
        println!("error!")
    }

    Ok(())
}
