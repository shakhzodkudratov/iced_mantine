use iced::{Background, Border, Color, Event, Length, Padding, Rectangle, Shadow, Vector};
use iced_core::{event, layout, mouse, overlay, renderer, touch, widget::{tree, Operation, Tree}, Clipboard, Element, Layout, Shell, Widget};

use crate::{colors, palettes::Palette, MantineTheme, Mode};

/// The possible status of a [`Button`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// The [`Button`] can be pressed.
    Active,
    /// The [`Button`] can be pressed and it is being hovered.
    Hovered,
    /// The [`Button`] is being pressed.
    Pressed,
    /// The [`Button`] cannot be pressed.
    Disabled,
}

/// The style of a button.
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(Default)]
pub struct Style {
    /// The [`Background`] of the button.
    pub background: Option<Background>,
    /// The text [`Color`] of the button.
    pub text_color: Option<Color>,
    /// The [`Border`] of the button.
    pub border: Option<Border>,
    /// The [`Radius`] of the button.
    pub radius: Option<iced::border::Radius>,
    /// The [`Shadow`] of the button.
    pub shadow: Option<Shadow>,
    /// The inner [`Padding`] of the button.
    pub padding: Option<Padding>,
    /// The outer [`Padding`] of the button.
    pub margin: Option<Padding>
}

impl Style {
    pub fn merge(&self, other: &Self) -> Self {
        Self {
            background: other.background.or(self.background),
            text_color: other.text_color.or(self.text_color),
            border: other.border.or(self.border),
            radius: other.radius.or(self.radius),
            shadow: other.shadow.or(self.shadow),
            padding: other.padding.or(self.padding),
            margin: other.padding.or(self.padding),
        }
    }

    /// Updates the [`Style`] with the given [`Background`].
    pub fn with_background(self, background: impl Into<Background>) -> Self {
        Self {
            background: Some(background.into()),
            ..self
        }
    }
    
    /// Updates the [`Style`] with the given [`Color`].
    pub fn with_text_color(self, color: impl Into<Color>) -> Self {
        Self {
            text_color: Some(color.into()),
            ..self
        }
    }

    pub fn with_border(self, border: impl Into<Border>) -> Self {
        Self {
            border: Some(border.into()),
            ..self
        }
    }

    fn with_padding(self, padding: impl Into<Padding>) -> Style {
        Self {
            padding: Some(padding.into()),
            ..self
        }
    }

    fn with_radius(self, radius: impl Into<iced::border::Radius>) -> Style {
        Self {
            radius: Some(radius.into()),
            ..self
        }
    }
}

/// A styling function for a [`Button`].
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Variant, Size, Radius, Status) -> Style + 'a>;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Variant {
    Default,
    Filled,
    Light,
    Outline,
    Subtle,
    Transparent,
    White
}

impl Variant {
    pub fn from(&self, theme: &MantineTheme, palette: Option<&Palette>, status: Status) -> Style {
        let base = Style::default();

        let palette = palette.unwrap_or(&theme.primary_palette);

        if status == Status::Disabled {
            return match theme.mode {
                Mode::Light => base.with_background(colors::GRAY_2).with_text_color(colors::GRAY_5),
                Mode::Dark => base.with_background(colors::DARK_6).with_text_color(colors::DARK_3),
            };
        }

        match theme.mode {
            Mode::Light => match self {
                Variant::Default => match status {
                    Status::Active => base.with_background(colors::WHITE).with_text_color(colors::BLACK),
                    Status::Hovered | Status::Pressed => base.with_background(colors::GRAY_0).with_text_color(colors::BLACK),
                    _ => base,
                },
                Variant::Filled => match status {
                    Status::Active => base.with_background(palette.6).with_text_color(colors::WHITE),
                    Status::Hovered | Status::Pressed => base.with_background(palette.7).with_text_color(colors::WHITE),
                    _ => base,
                },
                Variant::Light => match status {
                    Status::Active => base.with_background(palette.6).with_text_color(colors::WHITE),
                    Status::Hovered | Status::Pressed => base.with_background(palette.7).with_text_color(colors::WHITE),
                    _ => base,
                },
                Variant::Outline => match status {
                    Status::Active => base.with_background(palette.6).with_text_color(colors::WHITE),
                    Status::Hovered | Status::Pressed => base.with_background(palette.7).with_text_color(colors::WHITE),
                    _ => base,
                },
                Variant::Subtle => match status {
                    Status::Active => base.with_background(palette.6).with_text_color(colors::WHITE),
                    Status::Hovered | Status::Pressed => base.with_background(palette.7).with_text_color(colors::WHITE),
                    _ => base,
                },
                Variant::Transparent => match status {
                    Status::Active | Status::Hovered | Status::Pressed => base.with_text_color(palette.3),
                    _ => base,
                },
                Variant::White => match status {
                    Status::Active | Status::Hovered | Status::Pressed => base.with_background(colors::WHITE).with_text_color(palette.8),
                    _ => base,
                },
            },
            Mode::Dark => match self {
                Variant::Default => match status {
                    Status::Active => base
                        .with_background(colors::DARK_6)
                        .with_text_color(colors::WHITE)
                        .with_border(Border::default().color(colors::DARK_4).width(1.0)),
                    Status::Hovered | Status::Pressed => base
                        .with_background(colors::DARK_5)
                        .with_text_color(colors::WHITE)
                        .with_border(Border::default().color(colors::DARK_4).width(1.0)),
                    _ => base,
                },
                Variant::Filled => match status {
                    Status::Active => base.with_background(palette.6).with_text_color(colors::WHITE),
                    Status::Hovered | Status::Pressed => base.with_background(palette.7).with_text_color(colors::WHITE),
                    _ => base,
                },
                Variant::Light => match status {
                    Status::Active => base.with_background(palette.6.scale_alpha(0.15)).with_text_color(palette.3),
                    Status::Hovered | Status::Pressed => base.with_background(palette.6.scale_alpha(0.2)).with_text_color(palette.3),
                    _ => base,
                },
                Variant::Outline => match status {
                    Status::Active => base
                        .with_background(palette.4.scale_alpha(0.0))
                        .with_text_color(palette.4)
                        .with_border(Border::default().color(palette.4).width(1.0)),
                    Status::Hovered | Status::Pressed => base
                        .with_background(palette.4.scale_alpha(0.01))
                        .with_text_color(palette.4)
                        .with_border(Border::default().color(palette.4).width(1.0)),
                    _ => base,
                },
                Variant::Subtle => match status {
                    Status::Active => base.with_text_color(palette.3),
                    Status::Hovered | Status::Pressed => base.with_background(palette.6.scale_alpha(0.2)).with_text_color(palette.3),
                    _ => base,
                },
                Variant::Transparent => match status {
                    Status::Active | Status::Hovered | Status::Pressed => base.with_text_color(palette.3),
                    _ => base,
                },
                Variant::White => match status {
                    Status::Active | Status::Hovered | Status::Pressed => base.with_background(colors::WHITE).with_text_color(palette.8),
                    _ => base,
                },
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Size {
    Xs,
    Sm,
    Md,
    Lg,
    Xl
}

impl Size {
    pub fn from(self: &Size, theme: &MantineTheme) -> Style {
        let base = Style::default();

        match self {
            Size::Xs => base.with_padding(Padding::default().top(theme.from_rem(0.4375)).bottom(theme.from_rem(0.4375)).left(theme.from_rem(0.875)).right(theme.from_rem(0.875))),
            Size::Sm => base.with_padding(Padding::default().top(theme.from_rem(0.5625)).bottom(theme.from_rem(0.5625)).left(theme.from_rem(1.125)).right(theme.from_rem(1.125))),
            Size::Md => base.with_padding(Padding::default().top(theme.from_rem(0.75)).bottom(theme.from_rem(0.75)).left(theme.from_rem(1.325)).right(theme.from_rem(1.325))),
            Size::Lg => base.with_padding(Padding::default().top(theme.from_rem(0.8125)).bottom(theme.from_rem(0.8125)).left(theme.from_rem(1.625)).right(theme.from_rem(1.625))),
            Size::Xl => base.with_padding(Padding::default().top(theme.from_rem(1.0625)).bottom(theme.from_rem(1.0625)).left(theme.from_rem(2.0)).right(theme.from_rem(2.0))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Radius {
    Xs,
    Sm,
    Md,
    Lg,
    Xl
}

impl Radius {
    pub fn from(&self, theme: &MantineTheme) -> Style {
        let base = Style::default();

        match self {
            Radius::Xs => base.with_radius(iced::border::Radius::from(theme.from_rem(0.125))),
            Radius::Sm => base.with_radius(iced::border::Radius::from(theme.from_rem(0.25))),
            Radius::Md => base.with_radius(iced::border::Radius::from(theme.from_rem(0.5))),
            Radius::Lg => base.with_radius(iced::border::Radius::from(theme.from_rem(1.0))),
            Radius::Xl => base.with_radius(iced::border::Radius::from(theme.from_rem(2.0))),
        }
    }
}

enum OnPress<'a, Message> {
    Direct(Message),
    Closure(Box<dyn Fn() -> Message + 'a>),
}

impl<'a, Message: Clone> OnPress<'a, Message> {
    fn get(&self) -> Message {
        match self {
            OnPress::Direct(message) => message.clone(),
            OnPress::Closure(f) => f(),
        }
    }
}

pub struct Button<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Renderer: iced_core::Renderer,
{
    theme: &'a MantineTheme,
    content: Element<'a, Message, Theme, Renderer>,
    on_press: Option<OnPress<'a, Message>>,
    width: Length,
    height: Length,
    variant: Variant,
    palette: Option<Palette>,
    size: Size,
    radius: Radius,
    clip: bool,
}

impl<'a, Message, Theme, Renderer> Button<'a, Message, Theme, Renderer>
where
    Renderer: iced_core::Renderer,
{
    /// Creates a new [`Button`] with the given content.
    pub fn new(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        theme: &'a MantineTheme
    ) -> Self {
        let content = content.into();
        let size = content.as_widget().size_hint();

        Button {
            content,
            theme,
            on_press: None,
            width: size.width.fluid(),
            height: size.height.fluid(),
            variant: Variant::Default,
            size: Size::Sm,
            radius: Radius::Sm,
            palette: None,
            clip: false,
        }
    }

    /// Sets the width of the [`Button`].
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Button`].
    ///
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    // /// Sets the [`Padding`] of the [`Button`].
    // pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
    //     self.padding = padding.into();
    //     self
    // }

    /// Sets the message that will be produced when the [`Button`] is pressed.
    ///
    /// Unless `on_press` is called, the [`Button`] will be disabled.
    pub fn on_press(mut self, on_press: Message) -> Self {
        self.on_press = Some(OnPress::Direct(on_press));
        self
    }

    /// Sets the message that will be produced when the [`Button`] is pressed.
    ///
    /// This is analogous to [`Button::on_press`], but using a closure to produce
    /// the message.
    ///
    /// This closure will only be called when the [`Button`] is actually pressed and,
    /// therefore, this method is useful to reduce overhead if creating the resulting
    /// message is slow.
    pub fn on_press_with(
        mut self,
        on_press: impl Fn() -> Message + 'a,
    ) -> Self {
        self.on_press = Some(OnPress::Closure(Box::new(on_press)));
        self
    }

    /// Sets the message that will be produced when the [`Button`] is pressed,
    /// if `Some`.
    ///
    /// If `None`, the [`Button`] will be disabled.
    pub fn on_press_maybe(mut self, on_press: Option<Message>) -> Self {
        self.on_press = on_press.map(OnPress::Direct);
        self
    }

    /// Sets whether the contents of the [`Button`] should be clipped on
    /// overflow.
    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    // /// Sets the style class of the [`Button`].
    // #[cfg(feature = "advanced")]
    // #[must_use]
    // pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
    //     self.class = class.into();
    //     self
    // }

    pub fn size(mut self, size: Size) -> Self {
        self.size = size;
        self
    }

    pub fn radius(mut self, radius: Radius) -> Self {
        self.radius = radius;
        self
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn palette(mut self, palette: Palette) -> Self {
        self.palette = Some(palette);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct State {
    is_pressed: bool,
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Button<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + iced_core::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> iced_core::Size<Length> {
        iced_core::Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let style = self.size.from(self.theme);

        layout::padded(
            limits,
            self.width,
            self.height,
            style.padding.unwrap_or_default(),
            |limits| {
                self.content.as_widget().layout(
                    &mut tree.children[0],
                    renderer,
                    limits,
                )
            },
        )
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            self.content.as_widget().operate(
                &mut tree.children[0],
                layout.children().next().unwrap(),
                renderer,
                operation,
            );
        });
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        if let event::Status::Captured = self.content.as_widget_mut().on_event(
            &mut tree.children[0],
            event.clone(),
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        ) {
            return event::Status::Captured;
        }

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if self.on_press.is_some() {
                    let bounds = layout.bounds();

                    if cursor.is_over(bounds) {
                        let state = tree.state.downcast_mut::<State>();

                        state.is_pressed = true;

                        return event::Status::Captured;
                    }
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                if let Some(on_press) = self.on_press.as_ref().map(OnPress::get)
                {
                    let state = tree.state.downcast_mut::<State>();

                    if state.is_pressed {
                        state.is_pressed = false;

                        let bounds = layout.bounds();

                        if cursor.is_over(bounds) {
                            shell.publish(on_press);
                        }

                        return event::Status::Captured;
                    }
                }
            }
            Event::Touch(touch::Event::FingerLost { .. }) => {
                let state = tree.state.downcast_mut::<State>();

                state.is_pressed = false;
            }
            _ => {}
        }

        event::Status::Ignored
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let content_layout = layout.children().next().unwrap();
        let is_mouse_over = cursor.is_over(bounds);

        let status = if self.on_press.is_none() {
            Status::Disabled
        } else if is_mouse_over {
            let state = tree.state.downcast_ref::<State>();

            if state.is_pressed {
                Status::Pressed
            } else {
                Status::Hovered
            }
        } else {
            Status::Active
        };

        let style = self.variant.from(self.theme, self.palette.as_ref(), status)
            .merge(&self.size.from(self.theme))
            .merge(&self.radius.from(self.theme));


        if style.background.is_some()
            || style.border.map(|b| b.width > 0.0).unwrap_or(false)
            || style.radius.is_some()
            || style.shadow.map(|s| s.color.a > 0.0).unwrap_or(false)
        {
            let mut border = style.border.unwrap_or_default();

            if let Some(radius) = style.radius {
                border.radius = radius;
            }

            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border,
                    shadow: style.shadow.unwrap_or_default(),
                },
                style
                    .background
                    .unwrap_or(Background::Color(Color::TRANSPARENT)),
            );
        }

        let viewport = if self.clip {
            bounds.intersection(viewport).unwrap_or(*viewport)
        } else {
            *viewport
        };

        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            &renderer::Style {
                text_color: style.text_color.unwrap_or(Color::BLACK),
            },
            content_layout,
            cursor,
            &viewport,
        );
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let is_mouse_over = cursor.is_over(layout.bounds());

        if is_mouse_over && self.on_press.is_some() {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout.children().next().unwrap(),
            renderer,
            translation,
        )
    }
}

impl<'a, Message, Theme, Renderer> From<Button<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: 'a,
    Renderer: iced_core::Renderer + 'a,
{
    fn from(button: Button<'a, Message, Theme, Renderer>) -> Self {
        Self::new(button)
    }
}
