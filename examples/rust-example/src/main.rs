extern crate reqwest;
extern crate rust;

use rust::gen::com::github::service::_3_0_0::Github_Reqwest;

fn main() {
    let client = reqwest::Client::new();
    let github = Github_Reqwest::new(client, None).expect("failed to setup client");
    let gists = github.get_user_gists("udoprog".to_string()).expect("bad response");

    for gist in gists {
        println!("Gist ID: {}", gist.id);
        println!("Gist URL: {:?}", gist.url);
    }
}
