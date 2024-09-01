use std::process::Command;

use iced::*;
use iced::widget::row;
use iced::widget::column;

use serde::Serialize;
use widget::button;
use widget::container;
use widget::scrollable::*;
use widget::space;
use widget::Scrollable;
use iced::widget::text;

use tag_fs::Filesystem;
use tag_fs::Tag;
use widget::Space;
struct App {
	tags: Filesystem,
}
#[derive(Debug)]
#[derive(Clone)]
enum Message {
	Help,
	CursorUp,
	CursorLeft,
	CursorDown,
	CursorRight,
}
impl Sandbox for App {
	type Message = Message;
	// How Message updates the state
	fn update(&mut self, message: Self::Message) {
		match message {
			Message::Help => {
				// Make popup box with controls or something of said nature and the like.
			},

			_ => unimplemented!(),
		}
	}
	fn view(&self) -> Element<'_, Self::Message> {
		let xx = column((0..100).map(|i| text(format!("{} vertical scrollable", i + 1)).into()));
		let xy = column((0..100).map(|i| text(format!("{} vertical scrollable", i + 1)).into()));
		let top_half = row![
			Scrollable::new(xx)
				.width(230).height(Length::Fill)
				.direction(Direction::Vertical(Properties::new())), // change .style() of these 	
			Scrollable::new(xy)
				.width(Length::Fill).height(Length::Fill)
				.direction(Direction::Vertical(Properties::new())),
		]
			.height(Length::Fill)
			.spacing(10);
		return top_half.into();
	}
	fn new() -> Self {
		// Check if tagging backend is installed
		let _ = Command::new("tag-backend")
			.output()
			.expect("tag backend not installed");

		// Try to load files
		let tags = Command::new("tag-backend").arg("-F").output().expect("Something went wrong with -F");
		let tags = String::from_utf8_lossy(&tags.stdout);

		let fs = serde_json::from_str::<Filesystem>(&tags).unwrap();
		return App {tags: fs};
	}
	fn title(&self) -> String {"Tagging GUI".into()}
	fn theme(&self) -> Theme {
		iced::Theme::CatppuccinFrappe
	}
}
fn main() -> iced::Result {
	<App as iced::Sandbox>::run(Settings::default())
}