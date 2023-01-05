use lazy_static::lazy_static;

lazy_static! {
    pub static ref STANDARD_GAMER_LEVELS: Vec<&'static str> = [
        "Genius",
        "Magnificent",
        "Impressive",
        "Splendid",
        "Great",
        "Phew"
    ]
    .to_vec();
}
