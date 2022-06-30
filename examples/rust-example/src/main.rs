use rust::gen::github::v3;

#[tokio::main]
async fn main() -> Result<(), rust::gen::reproto::Error> {
    let client = reqwest::Client::new();
    let github = v3::Github_Reqwest::new(client, None)?;

    let rate_limit = github.get_rate_limit().await?;
    println!("{:?}", rate_limit);

    let gists = github.get_user_gists("udoprog".to_string()).await?;

    for g in gists {
        println!("{:?}", g);
    }

    Ok(())
}
