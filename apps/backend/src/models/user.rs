#[allow(dead_code)]
pub struct User<'a> {
    username: Option<&'a str>,
    password: Option<&'a str>,
    created_at: Option<i64>,
}
