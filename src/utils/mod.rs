pub mod download {

    use serde_json::json;
    use std::path::PathBuf;
    use std::time::{Duration, Instant};
    use std::{env, fs};
    use thirtyfour::prelude::*;

    pub async fn download_link(url: &str, album_url: &str) -> WebDriverResult<()> {
        let current_dir = env::current_dir().expect("failed to get current directory");
        let album_id = album_url.split("/a/").last().unwrap();
        let download_dir = current_dir.join("Downloads").join(album_id);
        fs::create_dir_all(&download_dir).expect("Failed to create download dir");

        let mut caps = DesiredCapabilities::chrome();

        let ext_path = format!("{}/src/utils/adblock-extension", current_dir.display());

        let extension_path = PathBuf::from(ext_path).canonicalize()?;

        // Set Chrome download preferences
        let chrome_options = json!({
                   "prefs": {
                       "download.default_directory": download_dir,
                       "download.prompt_for_download": false,
                       "download.directory_upgrade": true,
                       "safebrowsing.enabled": true
                   },
        "args": [
                   format!("--load-extension={}", extension_path.display())
                   // Do NOT add "--headless" here if you're using an extension
               ]
               });

        caps.set_base_capability("goog:chromeOptions", chrome_options)?;
        // caps.set_base_capability(
        //     "goog:chromeOptions",
        //     json!({
        //         "args": [
        //             // "--headless=new",
        //             // "--no-sandbox",
        //             // "--disable-gpu",
        //             // "--window-size=1280,800"
        //         ]
        //     }),
        // )?;

        let driver = WebDriver::new("http://localhost:9515", caps).await?;

        driver.goto(url).await?;

        if let Ok(btn) = driver.find(By::Id("download-btn")).await {
            btn.click().await?;
            println!("Clicked download button...");
        } else {
            println!("Download button not found");
        }

        tokio::time::sleep(Duration::from_secs(3)).await;

        wait_for_download_complete(&download_dir.to_string_lossy(), Duration::from_secs(6000))
            .await;

        println!("✅ Download completed!");
        driver.quit().await?;
        Ok(())
    }

    async fn wait_for_download_complete(path: &str, timeout: Duration) {
        let start = Instant::now();

        loop {
            let files: Vec<_> = fs::read_dir(path).unwrap().filter_map(Result::ok).collect();

            let downloading = files.iter().any(|f| {
                if let Some(name) = f.file_name().to_str() {
                    name.ends_with(".crdownload")
                } else {
                    false
                }
            });

            if !downloading && !files.is_empty() {
                break;
            }

            if start.elapsed() > timeout {
                println!("⚠️  Download timeout exceeded!");
                break;
            }

            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}
