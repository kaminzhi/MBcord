use anyhow::{anyhow, Result};
use rspotify::{
    model::{SearchType, TrackId},
    prelude::*,
    ClientCredsSpotify, Credentials,
};

pub struct SpotifyService {
    client: ClientCredsSpotify,
}

impl SpotifyService {
    pub async fn new() -> Result<Self> {
        let creds =
            Credentials::from_env().map_err(|_| anyhow!("Failed to get Spotify credentials"))?;
        let client = ClientCredsSpotify::new(creds);
        client.request_token().await?;
        Ok(Self { client })
    }

    pub async fn search_track(&self, query: &str) -> Result<String> {
        let search_result = self
            .client
            .search(query, SearchType::Track, None, None, Some(1), None)
            .await?;

        if let Some(tracks) = search_result.tracks {
            if let Some(track) = tracks.items.first() {
                return Ok(format!(
                    "{} - {}",
                    track.name,
                    track.artists.first().map_or("Unknown Artist", |a| &a.name)
                ));
            }
        }

        Err(anyhow!("No tracks found"))
    }

    pub async fn get_track_url(&self, track_id: &str) -> Result<String> {
        let track = self.client.track(TrackId::from_id(track_id)?).await?;
        Ok(track
            .external_urls
            .get("spotify")
            .ok_or_else(|| anyhow!("No Spotify URL found"))?
            .to_string())
    }
}
