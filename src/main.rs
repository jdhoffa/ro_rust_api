#[macro_use]
extern crate rocket;

use csv::ReaderBuilder;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket_okapi::{openapi, openapi_get_routes};
use schemars::JsonSchema;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
struct Iris {
    sepal_length: f64,
    sepal_width: f64,
    petal_length: f64,
    petal_width: f64,
    species: String,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
struct Boston {
    crim: f64,
    zn: f64,
    indus: f64,
    chas: f64,
    nox: f64,
    rm: f64,
    age: f64,
    dis: f64,
    rad: f64,
    tax: f64,
    ptratio: f64,
    b: f64,
    lstat: f64,
    medv: f64,
}

async fn get_data<T: for<'de> Deserialize<'de>>(file_path: &str) -> Json<Vec<T>> {
    let file = File::open(file_path).expect("Cannot open file");
    let reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new().from_reader(reader);

    let mut records = Vec::new();
    for result in csv_reader.deserialize() {
        let record: T = result.expect("Cannot deserialize record");
        records.push(record);
    }

    Json(records)
}

#[openapi]
#[get("/data/iris")]
async fn get_iris_data() -> Json<Vec<Iris>> {
    get_data::<Iris>("data/iris.csv").await
}

#[openapi]
#[get("/data/boston")]
async fn get_boston_data() -> Json<Vec<Boston>> {
    get_data::<Boston>("data/boston.csv").await
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", openapi_get_routes![get_iris_data, get_boston_data])
        .mount(
            "/docs",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_string(),
                ..Default::default()
            }),
        )
}
