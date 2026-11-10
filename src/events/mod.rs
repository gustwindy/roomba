use poise::serenity_prelude;

use crate::utils::{Data, Error};

mod roomba;
mod welcome;

pub async fn dispatch(
    framework: poise::FrameworkContext<'_, Data, Error>,
    event: &serenity_prelude::FullEvent,
) -> Result<(), Error> {
    roomba::event(framework, event).await?;
    welcome::event(framework, event).await?;

    Ok(())
}
