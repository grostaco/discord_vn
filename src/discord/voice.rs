use serenity::client::Context;
use songbird::input::restartable::Restartable;

use super::errors::PlayError;

pub async fn play_url(
    ctx: &Context,
    guild_id: u64,
    channel_id: u64,
    url: &str,
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

        let source = match Restartable::ytdl(url.trim().to_owned(), true).await {
            Ok(source) => source,
            Err(why) => return Err(PlayError::InputError(why)),
        };

        let handle = handler.play_only_source(source.into());
        handle.enable_loop().expect("Cannot enable loop");
    }

    Ok(())
}
/*
use std::sync::Arc;

macro_rules! cast {
    ($target: expr, $pat: path) => {{
        if let $pat(a) = $target {
            a
        } else {
            panic!("mismatch variant when cast to {}", stringify!($pat))
        }
    }};
}

macro_rules! cast_opt {
    ($target: expr, $pat: path) => {
        $target.map_or(None, |value| Some(cast!(value, $pat)))
    };
}

pub async fn play(
    http: &Arc<Http>,
    ctx: &Context,
    aci: ApplicationCommandInteraction,
) -> Result<(), PlayError> {
    let url = aci
        .data
        .options
        .iter()
        .find(|opt| opt.name == "url")
        .map(|opt| cast_opt!(opt.value.as_ref(), Value::String).unwrap())
        .unwrap();

    if !url.starts_with("http") {
        return Err(PlayError::InvalidURL(url.to_string()));
    }

    aci.create_interaction_response(http, |r| r.interaction_response_data(|d| d.content("A")))
        .await
        .unwrap();

    let manager = songbird::get(ctx)
        .await
        .ok_or(PlayError::Unregistered)?
        .clone();
    let _result = manager.join(887345509990285312, 887345509990285316).await;

    if let Some(handler_lock) = manager.get(887345509990285312) {
        let mut handler = handler_lock.lock().await;

        let source = match Restartable::ytdl(url.trim().to_owned(), false).await {
            Ok(source) => source,
            Err(why) => Err(PlayError::InputError(why))?,
        };
        let handle = handler.play_only_source(source.into());
        handle.enable_loop().expect("Cannot enable loop");
    }

    Ok(())
}
 */
