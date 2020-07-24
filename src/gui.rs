use super::TcodStruct;
use super::{CombatStats, Player, GameLog};
use specs::prelude::*;
use tcod::console::*;
use tcod::colors::*;

pub fn draw_ui(ecs: &World, tcod: &mut TcodStruct) {
    tcod.root
        .print_frame(0, 43, 80, 7, false, BackgroundFlag::None, Some("Log"));

    let combat_stats = ecs.read_storage::<CombatStats>();
    let players = ecs.read_storage::<Player>();

    for (_player, stats) in (&players, &combat_stats).join() {
        //let health = format!("HP: {} / {}", stats.curr_hp, stats.max_hp);
        //tcod.root.print(12, 43, &health);

        render_bar(&mut tcod.root, 12, 43, 10, "HP", stats.curr_hp, stats.max_hp, DESATURATED_GREEN, RED);
    }

    let log = ecs.fetch::<GameLog>();

    let mut y = 44;
    for s in log.entries.iter().rev() {
        if y < 49 {tcod.root.print(2, y, s);}
        y += 1;
    }
}

fn render_bar(
    panel: &mut Root,
    x: i32,
    y: i32,
    total_width: i32,
    name: &str,
    value: i32,
    maximum: i32,
    bar_color: Color,
    back_color: Color,
) {
    // render a bar (HP, experience, etc). First calculate the width of the bar
    let bar_width = (value as f32 / maximum as f32 * total_width as f32) as i32;

    // render the background first. Clears the tiles it is drawing over
    panel.set_default_background(back_color);
    panel.rect(x, y, total_width, 1, true, BackgroundFlag::Set);

    // now render the bar on top. Clears the tiles it is drawing over
    panel.set_default_background(bar_color);
    if bar_width > 0 {
        panel.rect(x, y, bar_width, 1, true, BackgroundFlag::Set);
    }

    // finally, some text with the values
    panel.set_default_foreground(WHITE);
    panel.print_ex(
        x + total_width / 2,
        y,
        BackgroundFlag::None,
        TextAlignment::Right,
        &format!("{}: {}/{}", name, value, maximum),
    );

    // Reset the default colors
    panel.set_default_background(BLACK);
    panel.set_default_foreground(WHITE);
}
