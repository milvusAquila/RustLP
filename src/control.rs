use iced::{
    widget::{button, column, pick_list, row, scrollable, text, Column},
    Element, Length,
};
// use iced_aw::menu::{self, Item};
use rusqlite::Result;

use crate::{db::Sort, Message};
use crate::{style, App};

impl App {
    pub fn view_control(&self) -> Element<Message> {
        // Header
/*         let menu_tpl = |items| {
            menu::Menu::new(items)
                .max_width(180.0)
                .offset(5.0)
                .spacing(5.0)
        };

        let open = button(text("Open")).style(style::header_button);

        let editor = button(text("Edit"))
            // .on_press(Message::OpenEditor)
            .style(style::header_button);

        #[rustfmt::skip]
        let header = iced_aw::menu_bar!(
            (button(text("File"))
                .style(style::header_button),
            {
                menu_tpl(iced_aw::menu_items!(
                    (open)
                    (editor)
                )).width(Length::Shrink)
            })
            (button(text("Settings"))
                .style(style::header_button),
            {
                self.view_settings() // see in src/settings.rs
            })
        ); */

        // Main
        let index = column![
            text("Library"),
            pick_list(Sort::ALL, self.sort, Message::SortChanged)
                .style(style::themed_pick_list)
                .width(Length::FillPortion(20)),
            scrollable(self.load_index().expect("ERROR: Failed to load index"))
                .width(Length::FillPortion(20))
                .height(Length::Fill),
        ];
        let preview = text("TODO").width(Length::FillPortion(30));
        let direct = text("TODO").width(Length::FillPortion(30));
        let service = text("TODO").width(Length::FillPortion(20));
        let main = row![index, preview, direct, service].spacing(10).padding(5);

        // Final
        Element::from(column![/* header, */ main,])
    }
    fn load_index(&self) -> Result<Column<'_, Message>> {
        let mut index = column![];
        let mut query = match self.sort.unwrap() {
            Sort::Title => self.db.prepare("SELECT title FROM songs ORDER BY title;")?,
            // Sort::Songbook => self.db.prepare("SELECT book FROM songs ORDER BY title;")?,
            _ => self.db.prepare("SELECT title FROM songs ORDER BY title;")?,
        };
        let mut titles = query.query([])?;
        while let Ok(Some(i)) = titles.next() {
            index = index.push(
                button(text(format!("{}", i.get::<_, String>(0)?))).style(style::header_button),
            );
        }
        Ok(index)
    }
}
