
pub fn about(desc:&str, end:&str, creator:&str) -> String {
    format!("{desc}\nURL:https://portswigger.net/web-security/{end}\nBy: {creator}")
}

pub fn _working_zebra() {
    let banner = r#"
                          + --------------------------------------------------- +
    \\/),               / | We have a zebra working on this lab at the momment, |
   ,'.' /,             +  | please come back again later and check!             |
  (_)- / /,            |  + --------------------------------------------------- +
     /\_/ |__..--,  *  | /                                                     /
    (\ _ /\ \ \ / ).'   + --------------------------------------------------- +
     \(-'./ / (_ //
      \\ \,'--'\_(
      )(_/  )_/ )_)
     (_,'  (_.'(_.'
"#;
    println!("{}", banner);
}

pub fn lets_go(lab: &str) {
    let banner = format!(r#"
                          + --------------------------------------------------- +
    \\/),               / | LAB {} LET'S GOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO |
   ,'.' /,             +  | OOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO! |
  (_)- / /,            |  + --------------------------------------------------- +
     /\_/ |__..--,  *  | /                                                     /
    (\ _ /\ \ \ / ).'   + --------------------------------------------------- +
     \(-'./ / (_ //
      \\ \,'--'\_(
      )(_/  )_/ )_)
     (_,'  (_.'(_.'
+ ============================================================================= +"#, lab);
    println!("{}", banner);
}

use reqwest::Client;
use std::time::Duration;
use tokio::time::sleep;

pub async fn did_i_win(url: &str, lab: &str) -> Result<(), Box<dyn std::error::Error>> {
    let banner = format!(r#"+ ============================================================================= +
                          + --------------------------------------------------- +
    \\/),               / | LAB {} WAS SO EASY,                               |
   ,'.' /,             +  | GOOD JOB!                                           |
  (_)- / /,            |  + --------------------------------------------------- +
     /\_/ |__..--,  *  | /                                                     /
    (\ _ /\ \ \ / ).'   + --------------------------------------------------- +
     \(-'./ / (_ //
      \\ \,'--'\_(
      )(_/  )_/ )_)
     (_,'  (_.'(_.'
"#, lab);
    let client = Client::new();
    println!("Let's check if you won...");
    loop {
        let response = client.get(url).send().await?;
        let text = response.text().await?;
        if text.contains("Congratulations, you solved the lab!") {
            println!("{}", banner);
            return Ok(());
        }
        sleep(Duration::from_secs(5)).await;
    }
}

