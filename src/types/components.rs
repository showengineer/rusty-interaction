#[cfg(feature = "handler")]
use log::warn;
use serde::{Deserialize, Serialize};

use serde_repr::*;
use serde_with::*;



#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// Message components are a framework for adding interactive elements to the messages your app or bot sends. They're accessible, customizable, and easy to use.
pub struct MessageComponent {
    /// Type of component
    r#type: ComponentType,
    style: Option<ComponentButtonStyle>,
    label: Option<String>,
    emoji: Option<PartialEmoji>,
    custom_id: Option<String>,
    url: Option<String>,
    disabled: Option<bool>,
    options: Option<Vec<ComponentSelectOption>>,
    placeholder: Option<String>,
    min_values: Option<u8>,
    max_values: Option<u8>,
    components: Option<Vec<MessageComponent>>,
}

impl Default for MessageComponent {
    fn default() -> Self {
        Self {
            r#type: ComponentType::ActionRow,
            style: None,
            label: None,
            custom_id: None,
            url: None,
            disabled: None,
            emoji: None,
            options: None,
            placeholder: None,
            max_values: None,
            min_values: None,
            components: None,
        }
    }
}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
#[non_exhaustive]
/// Represents a type of component
pub enum ComponentType {
    /// A container for other components
    ActionRow = 1,
    /// A clickable button
    Button = 2,
    /// A select menu for picking from choices
    SelectMenu = 3,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// A partial Emoji structure for Select Menu Options
pub struct PartialEmoji{
    id: u64,
    name: String,
    animated: bool,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// An option for select menu options
pub struct ComponentSelectOption{
    label: String,
    value: String,
    description: Option<String>,
    emoji: Option<PartialEmoji>,
    default: Option<bool>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// A select menu
pub struct ComponentSelectMenu{
    custom_id: String,
    options: Vec<ComponentSelectOption>,
    placeholder: Option<String>,
    min_values: u8,
    max_values: u8,
}

#[cfg(feature = "handler")]
impl Default for ComponentSelectMenu{
    fn default() -> Self{
        Self{
            custom_id: String::new(),
            options: Vec::new(),
            placeholder: None,

            // documented defaults
            min_values: 1,
            max_values: 1
        }
    }
}

#[cfg(feature = "handler")]
impl From<ComponentSelectMenu> for MessageComponent{
    fn from(t: ComponentSelectMenu) -> Self{
        let mut o = MessageComponent::default();

        o.r#type = ComponentType::SelectMenu;
        o.custom_id = Some(t.custom_id);
        o.options = Some(t.options);
        o.placeholder = t.placeholder;
        o.min_values = Some(t.min_values);
        o.max_values = Some(t.max_values);

        o
    }
}

#[cfg(feature = "handler")]
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
/// A button
pub struct ComponentButton{
    style: Option<ComponentButtonStyle>,
    label: Option<String>,
    emoji: Option<PartialEmoji>,
    custom_id: Option<String>,
    url: Option<String>,
    disabled: Option<bool>,
}
#[cfg(feature = "handler")]
impl Default for ComponentButton{
    fn default() -> Self{
        Self{
            style: Some(ComponentButtonStyle::Secondary),
            label: None,
            emoji: None,
            custom_id: None,
            url: None,
            disabled: None,
        }
    }
}
#[cfg(feature = "handler")]
impl From<ComponentButton> for MessageComponent{
    fn from(t: ComponentButton) -> Self{
        let mut o = MessageComponent::default();
        
        o.r#type = ComponentType::Button;
        o.style = t.style;
        o.label = t.label;
        o.emoji = t.emoji;
        o.custom_id = t.custom_id;
        o.url = t.url;
        o.disabled = t.disabled;
        
        o
    }
}

#[derive(Clone, Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
#[non_exhaustive]
/// How a button looks
pub enum ComponentButtonStyle {
    /// blurple
    Primary = 1,
    /// grey
    Secondary = 2,
    /// green
    Success = 3,
    /// red
    Danger = 4,
    /// grey with outgoing link icon
    Link = 5,
}



/// Builder for creating a Component Action Row

#[cfg(feature = "handler")]
#[derive(Clone, PartialEq, Debug)]
pub struct ComponentRowBuilder {
    obj: MessageComponent,
}
#[cfg(feature = "handler")]
impl Default for ComponentRowBuilder {
    fn default() -> Self {
        Self {
            obj: MessageComponent::default(),
        }
    }
}
#[cfg(feature = "handler")]
impl ComponentRowBuilder {
    /// Add a button
    pub fn add_button(mut self, button: ComponentButton) -> Self {
        match self.obj.components.as_mut() {
            None => {
                self.obj.components = Some(vec![button.into()]);
            }
            Some(c) => {
                c.push(button.into());
            }
        }
        self
    }

    /// Add a select menu to the row
    pub fn add_select_menu(mut self, menu: ComponentSelectMenu) -> Self{

        match self.obj.components.as_mut() {
            None => {
                self.obj.components = Some(vec![menu.into()]);
            }
            Some(c) => {
                c.push(menu.into());
            }
        }
        self
    }

    /// Finish building this row (returns a [`MessageComponent`])
    pub fn finish(self) -> MessageComponent {
        self.obj
    }
}
#[cfg(feature = "handler")]
#[derive(Clone, PartialEq, Debug)]
/// Builder for making an button component
pub struct ComponentButtonBuilder {
    obj: ComponentButton,
}

#[allow(clippy::field_reassign_with_default)]
#[cfg(feature = "handler")]
impl Default for ComponentButtonBuilder {
    fn default() -> Self {
        let ob = ComponentButton::default();
        Self { obj: ob }
    }
}
#[cfg(feature = "handler")]
impl ComponentButtonBuilder {
    /// Finish building this button
    pub fn finish(self) -> ComponentButton {
        match self.obj.clone().style.unwrap() {
            ComponentButtonStyle::Link => {
                if self.obj.url.is_none() {
                    warn!("The button style is set to 'Link', but no url was specified.");
                }
            }
            _ => {
                if self.obj.custom_id.is_none() {
                    warn!("No custom_id was supplied for this button!")
                }
            }
        }
        self.obj
    }

    /// Set the button style. Takes a [`ComponentButtonStyle`]
    pub fn style(mut self, s: &ComponentButtonStyle) -> Self {
        self.obj.style = Some(s.clone());
        self
    }
    /// Set the button label
    pub fn label(mut self, l: impl Into<String>) -> Self {
        let lab = l.into();

        self.obj.label = Some(lab);
        self
    }
    /// Set a custom id (required for all styles except `ComponentButtonStyle::Link`)
    pub fn custom_id(mut self, id: impl Into<String>) -> Self {
        let i = id.into();

        self.obj.custom_id = Some(i);
        self
    }
    /// Set a URL (required if style is set to `ComponentButtonStyle::Link`)
    pub fn url(mut self, url: impl Into<String>) -> Self {
        let u = url.into();

        self.obj.url = Some(u);
        self
    }
    /// Disables/deactivates a button
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.obj.disabled = Some(disabled);
        self
    }
}
