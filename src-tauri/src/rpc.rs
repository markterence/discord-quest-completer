pub use discord_sdk as ds;
pub use tokio;

/// Application identifier for "Andy's Test App" used in the Discord SDK's
/// examples.
pub struct Client {
    pub discord: ds::Discord,
    pub wheel: ds::wheel::Wheel,
    pub user: ds::user::User,
}

pub async fn make_client(app_id: ds::AppId, subs: ds::Subscriptions) -> Client {
    println!("Creating Discord client with app ID: {}", app_id);
    let (wheel, handler) = ds::wheel::Wheel::new(Box::new(|err| {
        println!("Error: {:?}", err);
    }));

    let mut user = wheel.user();

    let discord = ds::Discord::new(ds::DiscordApp::PlainId(app_id), subs, Box::new(handler))
        .expect("unable to create discord client");
    user.0.changed().await.unwrap();

    let user = match &*user.0.borrow() {
        ds::wheel::UserState::Connected(user) => user.clone(),
        ds::wheel::UserState::Disconnected(err) => panic!("failed to connect to Discord: {}", err),
    };

    println!("connected to Discord, local user is {:#?}", user);

    Client {
        discord,
        wheel,
        user,
    }
}
