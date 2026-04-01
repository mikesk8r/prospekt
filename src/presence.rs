pub fn status<'a>(current: &'a String) -> discord_rich_presence::activity::Activity<'a> {
    let activity = discord_rich_presence::activity::Activity::new()
        .buttons(vec![discord_rich_presence::activity::Button::new(
            "GitHub",
            "https://github.com/mikesk8r/prospekt",
        )])
        .state(current.as_str());
    activity
}
