
pub fn about(desc:&str, end:&str, creator:&str) -> String {
    format!("{desc}\nURL:https://portswigger.net/web-security/{end}\nBy: {creator}")
}

pub fn working_zebra() {
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
