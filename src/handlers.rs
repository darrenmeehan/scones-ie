use crate::github::github_authorize;
use axum::extract::Multipart;
use axum::{extract::Query, response::Redirect};

use std::collections::HashMap;
use std::{fs::File, io::Write};

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
        println!("Got file!");

        // Grab the name
        let file_name = field.file_name().unwrap();

        // Create a path for the soon-to-be file
        let file_path = format!("{}", file_name);

        // Unwrap the incoming bytes
        let data = field.bytes().await.unwrap();

        // Open a handle to the file
        let mut file_handle = File::create(file_path).expect("Failed to open file handle!");

        // Write the incoming data to the handle
        file_handle.write_all(&data).expect("Failed to write data!");

        println!("Successfull wrote to file")
    }
}
