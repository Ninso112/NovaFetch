//! Active media player status via MPRIS (Spotify, VLC, etc.).

/// Returns current track as "🎵 Artist - Song", or None if no player or DBus unavailable.
pub fn get_media_status() -> Option<String> {
    let finder = mpris::PlayerFinder::new().ok()?;
    let player = finder.find_active().ok()?;
    let metadata = player.get_metadata().ok()?;

    let artist = metadata
        .artists()
        .and_then(|a| a.first().copied())
        .unwrap_or("Unknown");
    let title = metadata.title().unwrap_or("Unknown");

    Some(format!("🎵 {} - {}", artist, title))
}
