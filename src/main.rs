
use iced::widget;
use iced::widget::scrollable::Direction;

use iced::Length;
use std::collections::HashMap;
use std::process::Command;
use iced::widget::row;
use iced::widget::column;
use iced::Element;
use iced::Sandbox;
use widget::button;
use widget::Scrollable;
use iced::widget::text;
use tag_fs::Filesystem;
struct App {
	fs: Filesystem,
	// <tag name, Button clicked state>
	tags: HashMap<String, bool>,
	files: Vec<String>,
}
#[derive(Debug)]
#[derive(Clone)]
enum Message {
	Help,
	TagClicked(String),
}
impl Sandbox for App {
	type Message = Message;
	// How Message updates the state
	fn update(&mut self, message: Self::Message) {
		match message {
			Message::Help => {
				// Make popup box with controls or something of said nature and the like.
			},
			Message::TagClicked(name) => {
				// change state of button
				let b = self.tags.get(&name).unwrap();
				self.tags.insert(name, !b);
				// filter files, and replace them
				let tags_selected: Vec<String> = {
					self.tags.iter()
						.filter(|siiyuh| siiyuh.1 == &true)
						.map(|(k, _)| k.clone())
						.collect()
				};
				let new_files = self.fs.filter(tags_selected).unwrap();
				self.files = new_files;
			},
		}
	}
	fn view(&self) -> Element<'_, Self::Message> {
		let tag_bar = column(self.tags.iter().map(|(name, _)| {
			button(text(name))
			 	.on_press(Message::TagClicked(name.into()))
				.width(Length::Fill)
				.into()
		})).spacing(10).padding(5);
		
		let file_bar = column((0..self.files.len()).map(|i| text(format!("{}", self.files[i])).into())).spacing(10);
		
		let top_half = row![
			Scrollable::new(tag_bar)
				.width(230).height(Length::Fill)
				.direction(Direction::Vertical(iced::widget::scrollable::Properties::new())), // change .style() of these 
			Scrollable::new(file_bar)
				.width(Length::Fill).height(Length::Fill)
				.direction(Direction::Vertical(iced::widget::scrollable::Properties::new())),
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

		// Hashmap full of tags and their state
		let mut tag_hs: HashMap<String, bool> = HashMap::new();
		let tags = fs.return_tags().unwrap();
		for s in tags {
			tag_hs.insert(s, false);
		}

		let files = fs.return_files().unwrap().clone();

		return App {
			fs: fs,
			tags: tag_hs,
			files: files,
		};
	}
	fn title(&self) -> String {"Tagging GUI".into()}
	fn theme(&self) -> iced::Theme {
		iced::Theme::CatppuccinFrappe
	}
}
/* 
struct TagStyle;
impl button::StyleSheet for TagStyle {
	type Style = iced::Theme;

	fn active(&self, style: &Self::Style) -> button::Appearance {
		button::Appearance { 
			shadow_offset: iced::Vector::default(),
			background: Some(Background::Color(Color::BLACK)), 
			text_color: (),
			border: (),
			shadow: () 
		}
	}
}
	*/
fn main() -> iced::Result {
	<App as iced::Sandbox>::run(iced::Settings::default())
}

