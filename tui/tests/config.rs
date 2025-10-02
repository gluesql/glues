mod tester;
use tester::Tester;

use color_eyre::Result;

#[tokio::test]
async fn config_isolation_test() -> Result<()> {
    use std::path::PathBuf;

    let home_config = PathBuf::from(std::env::var("HOME").unwrap_or_default()).join(".glues");
    let home_config_existed_before = home_config.exists();

    let _t = Tester::new().await?;

    glues_tui::config::update("last_csv_path", "/test/path").await;
    let value = glues_tui::config::get("last_csv_path").await;
    assert_eq!(value, Some("/test/path".to_string()));

    let home_config_exists_after = home_config.exists();
    assert_eq!(
        home_config_existed_before, home_config_exists_after,
        "Test should not create .glues in user's home directory"
    );

    if let Ok(config_dir) = std::env::var("GLUES_CONFIG_DIR") {
        let test_config = PathBuf::from(config_dir);
        assert!(
            test_config.exists(),
            "Test config directory should exist at GLUES_CONFIG_DIR"
        );
        assert_ne!(
            test_config, home_config,
            "Test config should not be in user's home directory"
        );
    }

    Ok(())
}
