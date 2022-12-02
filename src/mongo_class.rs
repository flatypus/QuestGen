use mongodb::{options::ClientOptions, Client, Database};


pub async fn set_up_database() -> Database {
    // Parse your connection string into an options struct
    let mut client_options =
        ClientOptions::parse("mongodb://100.90.120.61:30001/data?directConnection=true")
            .await.unwrap();
    // Manually set an option
    client_options.app_name = Some("Rust Demo".to_string());
    // Get a handle to the cluster
    let client = Client::with_options(client_options).unwrap();
    let db = client.database("data");
    db
}

