#[get("/")]
fn list_item() -> Json<JsonValue> {
    Json(json!([
        {
            "name": "a disk",
            "description": "useful item to store information",
            "price": 16
        },
        {
            "name": "b disk",
            "description": "another useful item",
            "price": 13
        }
    ]))
}