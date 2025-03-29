use serde::Serialize;

#[derive(Serialize)]
pub struct FlipCard {
    pub id: i32,
    pub first_side: String,
    pub second_side: String,
    pub category_id: i32,
}
