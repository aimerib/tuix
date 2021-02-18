extern crate tuix;

use tuix::{
    Application, BuildHandler, Button, Display, Event, EventHandler, TabContainer, TabEvent,
};

static THEME: &'static str = include_str!("themes/tabs_theme.css");

fn main() {
    let app = Application::new(|win_desc, state, window| {
        state.add_theme(THEME);

        // Create a tab container
        let (tab_bar, tab_container) = TabContainer::new().build(state, window, |builder| builder);

        // Add a tab to the tab bar
        Button::with_label("First")
            .on_press(Event::new(TabEvent::SwitchTab(0)))
            .build(state, tab_bar, |builder| builder.set_checked(true));

        // Add a widget to contain what will be displayed when tab 1 is selected
        let first = Button::new().build(state, tab_container, |builder| builder.class("item1"));
        // Add a button to this widget
        Button::with_label("First Button").build(state, first, |builder| builder.class("test"));

        Button::with_label("Second")
            .on_press(Event::new(TabEvent::SwitchTab(1)))
            .build(state, tab_bar, |builder| builder);
        let second = Button::new().build(state, tab_container, |builder| {
            builder.class("item2").set_display(Display::None)
        });

        Button::with_label("Second Button").build(state, second, |builder| builder.class("test"));

        Button::with_label("Third")
            .on_press(Event::new(TabEvent::SwitchTab(2)))
            .build(state, tab_bar, |builder| builder);
        let third = Button::new().build(state, tab_container, |builder| {
            builder.class("item1").set_display(Display::None)
        });
        Button::with_label("Third Button").build(state, third, |builder| builder.class("test"));

        win_desc.with_title("Text Input")
    });

    app.run()
}
