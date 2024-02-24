pub fn get_home<'a>() -> &'a str {
    include_str!("../../../static/index.html")
}
