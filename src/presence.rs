use discord_rich_presence::activity::*;

pub fn status<'a>(current: &'a String) -> Activity<'a> {
    let activity = Activity::new()
        .buttons(vec![Button::new(
            "GitHub",
            "https://github.com/mikesk8r/prospekt",
        )])
        .state(current.as_str());
    activity
}
