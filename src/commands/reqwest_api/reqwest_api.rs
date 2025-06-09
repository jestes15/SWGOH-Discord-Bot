use crate::commands::reqwest_api::guild_schema;
use crate::commands::reqwest_api::swgohgg_globals::AsyncWithLock;
use crate::commands::reqwest_api::swgohgg_globals::SWGOHGG_CLIENT;
use crate::commands::reqwest_api::user_schema;

pub async fn request_user_data(ally_code: &i64) -> user_schema::Root {
    let response = SWGOHGG_CLIENT.async_with_lock(|client| {
        client
            .get(format!("https://swgoh.gg/api/player/{}/", ally_code))
            .send()
    });

    response
        .await
        .await
        .unwrap()
        .json::<user_schema::Root>()
        .await
        .unwrap()
}

pub async fn request_guild_data(guild_id: &String) -> guild_schema::Root {
    let response = SWGOHGG_CLIENT.async_with_lock(|client| {
        client
            .get(format!("https://swgoh.gg/api/guild-profile/{}/", guild_id))
            .send()
    });

    response
        .await
        .await
        .unwrap()
        .json::<guild_schema::Root>()
        .await
        .unwrap()
}
