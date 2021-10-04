use cursive::traits::*;
use cursive::views::{Button, Dialog, DummyView, LinearLayout, SelectView};
use cursive::{Cursive, CursiveExt};

mod character;
mod character_selection;
mod dashboard;
mod data;

use crate::character::Character;
use crate::character_selection::{create_character, delete_character};
use crate::dashboard::get_dashboard;
use crate::data::Data;

fn main() {
    let mut siv = Cursive::new();

    siv.add_global_callback('q', |s| s.quit());
    siv.set_user_data(Data::new());

    let character_list = &siv.user_data::<Data>().unwrap().character_list;

    let character_select = SelectView::<Character>::new()
        .on_submit(get_dashboard)
        .with_all(
            character_list
                .into_iter()
                .map(|(_, v)| (v.display_for_selection(), v.clone())),
        )
        .with_name("character_select")
        .fixed_size((80, 20));

    let character_buttons = LinearLayout::vertical()
        .child(Button::new("Add new", create_character))
        .child(Button::new("Delete", delete_character))
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit));

    siv.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(character_select)
                .child(DummyView)
                .child(character_buttons),
        )
        .title("Select a character"),
    );

    siv.run();
}