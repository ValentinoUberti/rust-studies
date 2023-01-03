use serde_json::Value;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = "Bearer OzW3N52Wu7f0HfqpbXdFu0KS9JOEvQGAYazGuvoE";
    let url =
        "https://example-registry-quay-quay-enterprise.apps.ocphub.lab.seeweb/api/v1/user/";

    let api = reqwest::Client::new()
        .get(url)
        .header("Content-Type", "application/json")
        .header("accept", "application/json")
        .header("Authorization", token);

   // println!("{:#?}", api);
    let response: Value = api.send().await?.json().await?;

   
    for org in response["organizations"].as_array() {
        

    }
    println!("{:#?}", response["organizations"]);
    Ok(())
}
