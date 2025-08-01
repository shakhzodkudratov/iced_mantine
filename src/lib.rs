use iced::Element;

use crate::button::Button;

pub mod colors;
pub mod button;
pub mod theme;
pub mod utils;

pub use theme::MantineTheme;
pub use theme::Mode;

/// Creates a new [`Button`] with the provided content.
///
/// # Example
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } }
/// # pub type State = ();
/// # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
/// use iced::widget::button;
///
/// #[derive(Clone)]
/// enum Message {
///     ButtonPressed,
/// }
///
/// fn view(state: &State) -> Element<'_, Message> {
///     button("Press me!").on_press(Message::ButtonPressed).into()
/// }
/// ```
pub fn button<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
    theme: &'a MantineTheme
) -> Button<'a, Message, Theme, Renderer>
where
    Renderer: iced_core::Renderer,
{
    Button::new(content, theme)
}

