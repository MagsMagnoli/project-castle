use tauri::{App, Runtime};
use tide::prelude::*;
use tide::Request;
mod models;
use models::RequestBody;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    run_app(tauri::Builder::default(), |_app| {})
}

#[tokio::main]
pub async fn run_app<R: Runtime, F: FnOnce(&App<R>) + Send + 'static>(
    builder: tauri::Builder<R>,
    setup: F,
) {
    let builder = builder.setup(move |app| {
        tokio::spawn(async {
            run_server().await.expect("Failed to run the server");
        });

        setup(app);
        Ok(())
    });

    builder
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

async fn run_server() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/").post(handle_request);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u16,
}

async fn handle_request(mut req: Request<()>) -> tide::Result {
    let body: RequestBody = req.body_json().await?;
    println!("{:#?}", body);
    Ok("Received data".into())
}
