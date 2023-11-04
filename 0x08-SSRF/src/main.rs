use clap::{command, arg, Command};

mod misc;
mod ssrf_0x01;

use misc::{about, working_zebra};



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let banner = r#"
                          + --------------------------------------------- +
    \\/),               / | Scripts suite for PortSwigger's labs on SSRF. |
   ,'.' /,             +  | bY: cbr4l0k from SpyZ.                        |
  (_)- / /,            |  + --------------------------------------------- +
     /\_/ |__..--,  *  | /                                               /
    (\ _ /\ \ \ / ).'   + --------------------------------------------- +
     \(-'./ / (_ //
      \\ \,'--'\_(
      )(_/  )_/ )_)
     (_,'  (_.'(_.'
"#;

    let matches = command!("PortSwigger's SSRF Suite") 
        .author("cbr4l0k from SpyZ")
        .about(banner)
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("0x01")
            .arg_required_else_help(true)
            .about(about("Basic SSRF against the local server.","ssrf/lab-basic-ssrf-against-localhost", "cbr4l0k"))
            .arg(arg!(-u --url <PORTSWIGGER_LAB_URL> "")
                .required(true)
                .help("Homepage of the targeted website.")
            )
            .arg(arg!(-t --target_username <TARGET_USERNAME> "")
                .required(true)
                .help("The target user will be deleted from the system.")
            )
        )
        .subcommand(
            Command::new("0x02")
            // .arg_required_else_help(true)
            .about(about("Basic SSRF against another back-end system.","ssrf/lab-basic-ssrf-against-backend-system", ""))
        )
        .subcommand(
            Command::new("0x03")
            // .arg_required_else_help(true)
            .about(about("Blind SSRF with out-of-band detection.","ssrf/blind/lab-out-of-band-detection", ""))
        )
        .subcommand(
            Command::new("0x04")
            // .arg_required_else_help(true)
            .about(about("SSRF with blacklist-based input filter.","ssrf/lab-ssrf-with-blacklist-filter", ""))
        )
        .subcommand(
            Command::new("0x05")
            // .arg_required_else_help(true)
            .about(about("SSRF with filter bypass via open redirection vulnerability.","ssrf/lab-ssrf-filter-bypass-via-open-redirection", ""))
        )
        .subcommand(
            Command::new("0x06")
            // .arg_required_else_help(true)
            .about(about("Blind SSRF with Shellshock exploitation.","ssrf/blind/lab-shellshock-exploitation", ""))
        )
        .subcommand(
            Command::new("0x07")
            // .arg_required_else_help(true)
            .about(about("SSRF with whitelist-based input filter.","ssrf/lab-ssrf-with-whitelist-filter", ""))
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("0x01") {
        if matches.contains_id("url") && matches.contains_id("target_username") {
            let url = matches.get_one::<String>("url").unwrap();
            let target_username = matches.get_one::<String>("target_username").unwrap();

            ssrf_0x01::delete_user(url, target_username).await?;
        }
    } else if let Some(_matches) = matches.subcommand_matches("0x02") {
        working_zebra();
    } else if let Some(_matches) = matches.subcommand_matches("0x03") {
        working_zebra();
    } else if let Some(_matches) = matches.subcommand_matches("0x04") {
        working_zebra();
    } else if let Some(_matches) = matches.subcommand_matches("0x05") {
        working_zebra();
    } else if let Some(_matches) = matches.subcommand_matches("0x06") {
        working_zebra();
    } else if let Some(_matches) = matches.subcommand_matches("0x07") {
        working_zebra();
    }

    Ok(())
}
