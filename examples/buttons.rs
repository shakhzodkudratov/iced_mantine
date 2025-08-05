use iced::{Length, Padding};
use iced_mantine::{button, button_core::{Button, Radius, Size, Variant}, palettes, MantineTheme};
use iced_widget::{column, container, row, scrollable, Column, Row, Text};

fn main() -> iced::Result {
    iced::application("Buttons", App::update, App::view)
        .run_with(|| (
            App {
                theme: MantineTheme::default(),
            },
            iced::Task::none()
        ))
}

#[derive(Debug, Clone)]
enum Message {
    None,
}

#[derive(Default)]
struct App {
    theme: MantineTheme,
}

impl App {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> iced::Element<'_, Message> {
        let available_palettes: Vec<(&'static str, palettes::Palette)> = vec![
            ("DARK", palettes::DARK),
            ("GRAY", palettes::GRAY),
            ("RED", palettes::RED),
            ("PINK", palettes::PINK),
            ("GRAPE", palettes::GRAPE),
            ("VIOLET", palettes::VIOLET),
            ("INDIGO", palettes::INDIGO),
            ("BLUE", palettes::BLUE),
            ("CYAN", palettes::CYAN),
            ("TEAL", palettes::TEAL),
            ("GREEN", palettes::GREEN),
            ("LIME", palettes::LIME),
            ("YELLOW", palettes::YELLOW),
            ("ORANGE", palettes::ORANGE),
        ];

        let palette_buttons = available_palettes.into_iter().map(|(label, palette)| {
            Row::new()
                .push(Text::new(label))
                .push(Button::new("default", &self.theme).variant(Variant::Default).on_press(Message::None).palette(palette.clone()))
                .push(Button::new("filled", &self.theme).variant(Variant::Filled).on_press(Message::None).palette(palette.clone()))
                .push(Button::new("light", &self.theme).variant(Variant::Filled).on_press(Message::None).palette(palette.clone()))
                .push(Button::new("outline", &self.theme).variant(Variant::Outline).on_press(Message::None).palette(palette.clone()))
                .push(Button::new("subtle", &self.theme).variant(Variant::Subtle).on_press(Message::None).palette(palette.clone()))
                .push(Button::new("white", &self.theme).variant(Variant::White).on_press(Message::None).palette(palette.clone()))
                .push(Button::new("disabled", &self.theme).palette(palette.clone()))
                .spacing(12)
        }).fold(Column::new(), |column, row| column.push(row)).spacing(12);

        scrollable(
            container(
                column![
                    row![
                        button("xs", &self.theme).size(Size::Xs).radius(Radius::Xs).on_press(Message::None),
                        button("sx", &self.theme).size(Size::Sm).radius(Radius::Sm).on_press(Message::None),
                        button("md", &self.theme).size(Size::Md).radius(Radius::Md).on_press(Message::None),
                        button("lg", &self.theme).size(Size::Lg).radius(Radius::Lg).on_press(Message::None),
                        button("xl", &self.theme).size(Size::Xl).radius(Radius::Xl).on_press(Message::None),
                    ].spacing(12),
                    row![
                        button("default", &self.theme).variant(Variant::Default).on_press(Message::None),
                        button("filled", &self.theme).variant(Variant::Filled).on_press(Message::None),
                        button("light", &self.theme).variant(Variant::Light).on_press(Message::None),
                        button("outline", &self.theme).variant(Variant::Outline).on_press(Message::None),
                        button("subtle", &self.theme).variant(Variant::Subtle).on_press(Message::None),
                        button("transparent", &self.theme).variant(Variant::Transparent).on_press(Message::None),
                        button("white", &self.theme).variant(Variant::White).on_press(Message::None),
                        button("disabled", &self.theme),
                    ].spacing(12),
                    palette_buttons,
                ].spacing(12)
            ).padding(Padding::new(8.0))
        ).width(Length::Fill).into()
    }
}
