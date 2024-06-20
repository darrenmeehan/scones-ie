use crate::github::github_authorize;
use axum::extract::Multipart;
use axum::{extract::Query, response::Redirect};
use chrono::{DateTime, Datelike, Utc};
use std::collections::HashMap;
use std::{fs::File, io::Write};
use tracing::{event, Level};

#[derive(Debug, serde::Deserialize)]
struct Workout {
    #[serde(with = "date_serializer", alias = "Date")]
    date: DateTime<Utc>,
    #[serde(alias = "Workout Name")]
    workout_name: String,
    #[serde(alias = "Exercise Name")]
    exercise_name: String,
    #[serde(alias = "Set Order")]
    set_order: Option<u32>,
    #[serde(alias = "Weight")]
    weight: Option<u32>,
    #[serde(alias = "Weight Unit")]
    weight_unit: Option<String>,
    reps: Option<u32>,
    rpe: Option<u32>,
    distance: Option<u32>,
    #[serde(alias = "Distance Unit")]
    distance_unit: Option<String>,
    seconds: Option<u32>,
    #[serde(alias = "Notes")]
    notes: Option<String>,
    #[serde(alias = "Workout Notes")]
    workout_notes: Option<String>,
    workout_duration: Option<u32>,
}

pub async fn healthcheck_handler() -> String {
    "All's good".to_string()
}

pub async fn authorization_handler(Query(_params): Query<HashMap<String, String>>) -> Redirect {
    github_authorize().await
}

pub async fn error_handler() -> String {
    let message = "Internal Server Error".to_string();
    format!("Something went wrong: {}", message)
}

pub async fn upload_handler(mut multipart: Multipart) {
    while let Some(field) = multipart
        .next_field()
        .await
        .expect("Failed to get next field!")
    {
        if field.name().unwrap() != "fileupload" {
            continue;
        }
        event!(Level::DEBUG, "Got file!");

        // Grab the name
        let file_path = field.file_name().unwrap().to_string();

        // Create a path for the soon-to-be file
        // let file_path = format!("{}", file_name);

        // Unwrap the incoming bytes
        let data = field.bytes().await.unwrap();

        // Open a handle to the file
        let mut file_handle = File::create(&file_path).expect("Failed to open file handle!");

        // Write the incoming data to the handle
        file_handle.write_all(&data).expect("Failed to write data!");

        event!(Level::DEBUG, "Successfull wrote to file");

        extract_data(file_path);
    }
}

fn extract_data(file_path: String) -> () {
    let mut workouts: HashMap<String, Workout> = HashMap::new();
    let mut exercises: Vec<String> = Vec::new();

    let mut weight: u32 = 0;

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .double_quote(false)
        // .escape(Some(b'\\'))
        .flexible(true)
        // .comment(Some(b'#'))
        .from_path(file_path)
        .unwrap();

    let headers = reader.headers().unwrap();
    event!(Level::INFO, "{:?}", headers);

    for result in reader.deserialize::<Workout>() {
        match result {
            Ok(result) => {
                let workout: Workout = result;
                // Create a list of all exercises I've ever done
                if !exercises.contains(&workout.exercise_name) {
                    exercises.push(workout.exercise_name.clone());
                }

                // Count the total weight I've ever lifted
                if workout.weight.is_some() {
                    if workout.weight_unit == Some("kg".to_string()) {
                        weight += workout.weight.unwrap();
                        event!(
                            Level::DEBUG,
                            "Adding {}kg to total weight lifted",
                            workout.weight.unwrap()
                        );
                    } else {
                        event!(
                            Level::DEBUG,
                            "Skipping weight of {}{} as it's not in kg",
                            workout.weight.unwrap(),
                            workout.weight_unit.clone().unwrap()
                        );
                    }
                } else {
                    event!(
                        Level::DEBUG,
                        "No weight in for workout on {}. Weight is : {:?}",
                        workout.date.format("%a"),
                        workout.weight
                    );
                }

                if !workouts.contains_key(&workout.date.to_string()) {
                    workouts.insert(workout.date.to_string(), workout);
                }
            }
            Err(error) => {
                event!(Level::ERROR, "Error parsing workout. Error: {:?}", error);
            }
        }
    }
    // Print out the workouts from this year
    event!(Level::INFO, "Workouts this year:");
    for (_, workout) in workouts.iter() {
        if !workout.date.year().eq(&Utc::now().year()) {
            event!(
                Level::DEBUG,
                "{:?} on {}",
                workout.workout_name,
                workout.date.format("%a %d %b")
            );
        }
    }
    event!(Level::INFO, "{} workouts", workouts.len());

    event!(
        Level::INFO,
        "{:?} different exercises tried out",
        exercises.len()
    );

    event!(Level::INFO, "Total weight ever lifted: {}kg", weight);
}

mod date_serializer {
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    fn time_to_json(t: DateTime<Utc>) -> String {
        t.to_rfc3339()
    }

    pub fn serialize<S: Serializer>(
        time: &DateTime<Utc>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        time_to_json(time.clone()).serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error> {
        let time: String = Deserialize::deserialize(deserializer)?;

        let workaround_time = &format!("{}{}", &time, " +0000");
        let parsed_time = DateTime::parse_from_str(workaround_time, "%Y-%m-%d %H:%M:%S %z");

        match parsed_time {
            Ok(parsed_time) => Ok(parsed_time.to_utc()),
            Err(error) => {
                println!("Error parsing date {}. Error {}", workaround_time, error);
                Ok(DateTime::from_timestamp(1703984461, 0).unwrap().to_utc())
            }
        }
    }
}
