use serenity::client::Context;
use songbird::input::restartable::Restartable;

use super::errors::PlayError;

pub async fn play_url(
    ctx: &Context,
    guild_id: u64,
    channel_id: u64,
    url: &str,
    volume: f32,
) -> Result<(), PlayError> {
    if !url.starts_with("http") {
        return Err(PlayError::InvalidURL(url.to_string()));
    }

    let manager = songbird::get(ctx)
        .await
        .ok_or(PlayError::Unregistered)?
        .clone();
    let _result = manager.join(guild_id, channel_id).await;

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let source = match Restartable::ytdl(url.trim().to_owned(), false).await {
            Ok(source) => source,
            Err(why) => return Err(PlayError::InputError(why)),
        };

        let handle = handler.play_only_source(source.into());
        handle.set_volume(volume).expect("Cannot set volume");

        handle.enable_loop().expect("Cannot enable loop");
    }

    Ok(())
}
