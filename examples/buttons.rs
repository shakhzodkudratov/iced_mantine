use iced_mantine::{button, MantineTheme, button::{Size, Radius}};
use iced_widget::{column, row};

fn main() -> iced::Result {
    iced::application("Buttons", App::update, App::view)
        .run_with(|| (
            App {
                theme: MantineTheme::default(),
            },
            iced::Task::none()
        ))
}

type Message = ();

#[derive(Default)]
struct App {
    theme: MantineTheme,
}

impl App {
    fn update(&mut self, _message: Message) {}

    fn view(&self) -> iced::Element<'_, Message> {
        column![
            row![
                button("xs", &self.theme).size(Size::Xs).radius(Radius::Xs),
                button("sx", &self.theme).size(Size::Sm).radius(Radius::Sm),
                button("md", &self.theme).size(Size::Md).radius(Radius::Md),
                button("lg", &self.theme).size(Size::Lg).radius(Radius::Lg),
                button("xl", &self.theme).size(Size::Xl).radius(Radius::Xl),
            ].spacing(12)
        ].spacing(12).into()
    }
}
