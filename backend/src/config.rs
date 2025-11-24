use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub app_port: u16,
    pub database_url: String,
}

pub fn load() -> anyhow::Result<AppConfig> {
    // Load .env first
    dotenvy::from_filename(".env")?;

    let settings = config::Config::builder()
        .add_source(config::Environment::default().separator("_"))
        .build()?;

    // Map env vars to AppConfig fields
    let app_port: u16 = settings.get::<u16>("app.port")?;
    let db_host: String = settings.get::<String>("postgres.host")?;
    let db_port: u16 = settings.get::<u16>("postgres.port")?;
    let db_user: String = settings.get::<String>("postgres.user")?;
    let db_password: String = settings.get::<String>("postgres.password")?;
    let db_name: String = settings.get::<String>("postgres.db")?;
    
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        db_user, db_password, db_host, db_port, db_name
    );

    Ok(AppConfig {
        app_port,
        database_url,
    })
}
