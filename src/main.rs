#[macro_use]
extern crate rocket;

use csv::ReaderBuilder;
use rocket::serde::{json::Json, Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
struct Iris {
    sepal_length: f64,
    sepal_width: f64,
    petal_length: f64,
    petal_width: f64,
    species: String,
}

#[get("/data")]
async fn get_data() -> Json<Vec<Iris>> {
    let file = File::open("data/iris.csv").expect("Cannot open file");
    let reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new().from_reader(reader);

    let mut records = Vec::new();
    for result in csv_reader.deserialize() {
        let record: Iris = result.expect("Cannot deserialize record");
        records.push(record);
    }

    Json(records)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![get_data])
}
