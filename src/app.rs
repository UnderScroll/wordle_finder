use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

use iced::{
    Background, Color, Element,
    Length::{self, Fill},
    Padding, Theme,
    border::rounded,
    widget::{
        column, container, row, scrollable, text, text_editor,
        text_editor::{Action, Content},
    },
};
use iced_core::text::LineHeight;
use iced_widget::{container::Style, toggler};

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone)]
pub enum Message {
    PositionEditAction(usize, Action),
    IncludingEditAction(Action),
    ExcludingEditAction(Action),
    ToggleCommonWords,
}

pub struct App {
    words: Vec<String>,
    filtered_words: Vec<String>,
    sorted_common_words: Vec<String>,
    common_words: HashSet<String>,
    position_content: [Content; 5],
    including_content: Content,
    excluding_content: Content,
    only_show_common: bool,
}

impl App {
    pub fn new() -> Self {
        const ALL_WORDS_FILE_PATH: &str = "data/all_words.csv";
        const COMMON_WORDS_FILE_PATH: &str = "data/common_words.csv";

        // Load word list
        let mut all_word_file = File::open(ALL_WORDS_FILE_PATH)
            .expect("Can't find word list at [{ALL_WORDS_FILE_PATH}]");
        // Read file
        let mut text = String::new();
        all_word_file
            .read_to_string(&mut text)
            .expect("Failed to read string from file.");

        // Extract words
        let mut words = Vec::with_capacity(14294);
        for (index, word) in text.lines().enumerate() {
            if word.len() != 5 {
                panic!(
                    "Invalid word in during word exctraction: At line [{}], the word [{word}] wasn't exacly five characters in length",
                    index + 1
                )
            }
            words.push(word.to_string());
        }

        /* Mark common words */
        // Load word list
        let mut common_word_file = File::open(COMMON_WORDS_FILE_PATH)
            .expect("Can't find word list at [{COMMON_WORDS_FILE_PATH}]");
        // Read file
        let mut text = String::new();
        common_word_file
            .read_to_string(&mut text)
            .expect("Failed to read string from file.");

        // Extract common words
        let mut common_words = HashSet::with_capacity(3240);
        let mut sorted_common_words = Vec::with_capacity(3240);
        for (index, word) in text.lines().enumerate() {
            if word.len() != 5 {
                panic!(
                    "Invalid word in during word exctraction: At line [{}], the word [{word}] wasn't exacly five characters in length",
                    index + 1
                )
            }
            common_words.insert(word.to_string());
            sorted_common_words.push(word.to_string());
        }

        // Init filtered words
        let filtered_words = words.clone();

        Self {
            words,
            filtered_words,
            common_words,
            sorted_common_words,
            position_content: [
                Content::new(),
                Content::new(),
                Content::new(),
                Content::new(),
                Content::new(),
            ],
            including_content: Content::new(),
            excluding_content: Content::new(),
            only_show_common: false,
        }
    }

    fn rare_word_badge_style(theme: &Theme) -> Style {
        let palette = theme.extended_palette();

        let base_background_color = palette.background.weak.color;

        Style {
            text_color: Some(palette.background.weak.text),
            background: Some(Background::Color(Color::from_rgb(
                base_background_color.r * 0.4,
                base_background_color.g * 0.4,
                base_background_color.b * 0.4,
            ))),
            border: rounded(15),
            ..Style::default()
        }
    }

    fn common_word_badge_style(theme: &Theme) -> Style {
        let palette = theme.extended_palette();

        Style {
            text_color: Some(palette.background.weak.text),
            background: Some(palette.background.weak.color.into()),
            border: rounded(15),
            ..Style::default()
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let position = column![
            text!("Position"),
            row![
                text_editor(&self.position_content[0])
                    .on_action(|action| Message::PositionEditAction(0, action)),
                text_editor(&self.position_content[1])
                    .on_action(|action| Message::PositionEditAction(1, action)),
                text_editor(&self.position_content[2])
                    .on_action(|action| Message::PositionEditAction(2, action)),
                text_editor(&self.position_content[3])
                    .on_action(|action| Message::PositionEditAction(3, action)),
                text_editor(&self.position_content[4])
                    .on_action(|action| Message::PositionEditAction(4, action)),
            ]
        ];

        let including = column![
            text!("Including").center(),
            text_editor(&self.including_content).on_action(Message::IncludingEditAction),
        ];

        let excluding = column![
            text!("Excluding"),
            text_editor(&self.excluding_content).on_action(Message::ExcludingEditAction)
        ];

        let word_lines = self.filtered_words.chunks(10).map(|word_line| {
            row(word_line.iter().map(|word| {
                let mut badge = container(text(word)).padding(Padding {
                    top: 3.0,
                    right: 10.0,
                    bottom: 3.0,
                    left: 10.0,
                });
                badge = if self.common_words.contains(word) {
                    badge.style(Self::common_word_badge_style)
                } else {
                    badge.style(Self::rare_word_badge_style)
                };
                badge.into()
            }))
            .spacing(10)
            .clip(true)
            .into()
        });

        let words_view = column(word_lines).spacing(10).width(Fill);

        let words_scrollable = container(scrollable(words_view).width(Fill)).padding(Padding {
            top: 10.0,
            right: 10.0,
            bottom: 10.0,
            left: 0.0,
        });

        let common_word_toggle = toggler(self.only_show_common)
            .on_toggle(|_| Message::ToggleCommonWords)
            .label("Only show common")
            .spacing(10)
            .text_line_height(LineHeight::Absolute(iced::Pixels(50.0)))
            .width(Fill);

        let view: Element<'_, Message> = container(
            row![
                column![position, including, excluding, common_word_toggle]
                    .spacing(10)
                    .width(Length::Fixed(250.0))
                    .padding(10),
                words_scrollable
            ]
            .spacing(10),
        )
        .width(Fill)
        .height(Fill)
        .into();

        view // .explain(Color::from_rgb(1.0, 0.0, 0.0))
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::PositionEditAction(idx, action) => {
                if idx >= 5 {
                    return;
                }
                match action {
                    Action::Edit(edit) => match edit {
                        text_editor::Edit::Insert(character) => {
                            if character.is_alphabetic() {
                                // Clear text
                                self.position_content[idx] = Content::new();

                                // Insert character
                                self.position_content[idx].perform(Action::Edit(
                                    text_editor::Edit::Insert(character.to_ascii_uppercase()),
                                ));
                            }
                        }
                        text_editor::Edit::Backspace | text_editor::Edit::Delete => {
                            self.position_content[idx] = Content::new();
                        }
                        _ => (),
                    },
                    _ => self.position_content[idx].perform(action),
                }
            }
            Message::IncludingEditAction(action) => match action {
                Action::Edit(edit) => match &edit {
                    text_editor::Edit::Insert(character) => {
                        if character.is_alphabetic() && self.including_content.text().len() < 5 {
                            self.including_content.perform(Action::Edit(
                                text_editor::Edit::Insert(character.to_ascii_uppercase()),
                            ));
                        }
                    }
                    _ => self.including_content.perform(Action::Edit(edit)),
                },
                _ => self.including_content.perform(action),
            },
            Message::ExcludingEditAction(action) => match action {
                Action::Edit(edit) => match &edit {
                    text_editor::Edit::Insert(character) => {
                        if !character.is_alphabetic() {
                            return;
                        }
                        let uppercase_charcater = character.to_ascii_uppercase();
                        if !self.excluding_content.text().contains(uppercase_charcater) {
                            self.excluding_content.perform(Action::Edit(
                                text_editor::Edit::Insert(uppercase_charcater),
                            ));
                        }
                    }
                    _ => self.excluding_content.perform(Action::Edit(edit)),
                },
                _ => self.excluding_content.perform(action),
            },
            Message::ToggleCommonWords => self.only_show_common = !self.only_show_common,
        }

        self.update_filtered_words();
    }

    fn update_filtered_words(&mut self) {
        self.filtered_words = if self.only_show_common {
            self.sorted_common_words.clone()
        } else {
            self.words.clone()
        };

        // Filter by position
        for (index, character) in self
            .position_content
            .iter()
            .map(|content| {
                content
                    .text()
                    .chars()
                    .next()
                    .map(|character| character.to_ascii_lowercase())
            })
            .enumerate()
        {
            if let Some(character) = character {
                self.filtered_words
                    .retain(|word| word.chars().nth(index).unwrap_or_else(|| panic!("Can't access character at index [{index}]: the word [{word}], doesn't have five letters.")) == character);
            }
        }

        // Filter by exclude
        for character in self
            .excluding_content
            .text()
            .chars()
            .map(|character| character.to_ascii_lowercase())
        {
            self.filtered_words.retain(|word| !word.contains(character))
        }

        // Filter by include
        // Count character frequency
        let mut frequency_map: HashMap<char, usize> = HashMap::new();
        for character in self
            .including_content
            .text()
            .chars()
            .map(|c| c.to_ascii_lowercase())
        {
            frequency_map
                .entry(character)
                .and_modify(|frequency| *frequency += 1)
                .or_insert(1);
        }
        // Filter by frequency
        for (character, frequency) in frequency_map {
            self.filtered_words
                .retain(|word| word.chars().filter(|c| c == &character).count() >= frequency);
        }
    }
}
