use std::path::PathBuf;

use config::MinraConfig;
use sea_orm::{Database, DatabaseConnection};
use tauri::{AppHandle, Manager};

mod config;
mod util;

use migration::{Migrator, MigratorTrait};
use util::{InitError, Result};

async fn connect_and_run_migrations(protocol_str: String) -> Result<DatabaseConnection, InitError> {
    let db = Database::connect(protocol_str).await?;

    Migrator::up(&db, None).await?;

    Ok(db)
}

fn connect_to_db(app: &AppHandle, config: &MinraConfig) -> Result<(), InitError> {
    let db_path: &PathBuf = &config.db_path;

    let db_path_str = db_path.to_str().ok_or(InitError::UncategorizedError(
        "Could not convert path to valid unicode string".into(),
    ))?;

    println!("{db_path_str}");

    let protocol_str = format!("sqlite://{db_path_str}?mode=rwc");

    // blocks the thread, and runs the above async function
    let db = tauri::async_runtime::block_on(connect_and_run_migrations(protocol_str))?;

    app.manage(db);

    Ok(())
}

pub fn run() {
    // first we retrieve the configuration file

    tauri::Builder::default()
        .setup(|app| {
            // we parse and load in the config file (as well as creating a few directories)
            let config: MinraConfig = config::load_config(app.handle())?;

            connect_to_db(app.handle(), &config)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
