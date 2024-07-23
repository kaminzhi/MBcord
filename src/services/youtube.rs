use anyhow::{anyhow, Result};
use songbird::input::{Input, Restartable};
use tokio::process::Command;

pub async fn search_and_get_source(query: &str) -> Result<Input> {
    let url = search_youtube(query).await?;
    let source = Restartable::ytdl(url, true).await?;
    Ok(source.into())
}

async fn search_youtube(query: &str) -> Result<String> {
    let output = Command::new("youtube-dl")
        .args(&["--get-id", "--default-search", "ytsearch", query])
        .output()
        .await?;

    if !output.status.success() {
        return Err(anyhow!("youtube-dl command failed"));
    }

    let video_id = String::from_utf8(output.stdout)?
        .lines()
        .next()
        .ok_or_else(|| anyhow!("No video ID found"))?
        .trim()
        .to_string();

    Ok(format!("https://www.youtube.com/watch?v={}", video_id))
}

pub async fn get_video_info(url: &str) -> Result<String> {
    let output = Command::new("youtube-dl")
        .args(&["-e", "--get-duration", url])
        .output()
        .await?;

    if !output.status.success() {
        return Err(anyhow!("youtube-dl command failed"));
    }

    let info = String::from_utf8(output.stdout)?;
    let mut lines = info.lines();
    let title = lines.next().ok_or_else(|| anyhow!("No title found"))?;
    let duration = lines.next().ok_or_else(|| anyhow!("No duration found"))?;

    Ok(format!("{} [{}]", title, duration))
}
