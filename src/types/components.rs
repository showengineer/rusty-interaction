use serde::{Deserialize, Serialize};
#[cfg(feature = "handler")]
use log::error;

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
    // pub emoji: Option<Emoji>,
    custom_id: Option<String>,
    url: Option<String>,
    disabled: Option<bool>,
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
pub struct ComponentRowBuilder {
    obj: MessageComponent,
}

impl Default for ComponentRowBuilder {
    fn default() -> Self {
        Self {
            obj: MessageComponent::default(),
        }
    }
}

impl ComponentRowBuilder {
    /// Add a button
    pub fn add_button(mut self, button: MessageComponent) -> Self {
        match self.obj.components.as_mut() {
            None => {
                self.obj.components = Some(vec![button]);
            }
            Some(c) => {
                c.push(button);
            }
        }
        self
    }

    /// Finish building this row (returns a [`MessageComponent`])
    pub fn finish(self) -> MessageComponent {
        self.obj
    }
}

/// Builder for making an button component
pub struct ComponentButtonBuilder {
    obj: MessageComponent,
}

impl Default for ComponentButtonBuilder {
    fn default() -> Self {
        let mut ob = MessageComponent::default();
        ob.r#type = ComponentType::Button;
        ob.style = Some(ComponentButtonStyle::Secondary);
        Self { obj: ob }
    }
}

impl ComponentButtonBuilder {
    /// Finish building this button
    pub fn finish(self) -> MessageComponent {
        match self.obj.clone().style.unwrap() {
            ComponentButtonStyle::Link => {
                if self.obj.url.is_none() {
                    error!("The button style is set to 'Link', but no url was specified.");
                }
            }
            _ => {
                if self.obj.custom_id.is_none() {
                    error!("No custom_id was supplied for this button!")
                }
            }
        }
        self.obj
    }

    /// Set the button style. Takes a [`ComponentButtonStyle`]
    pub fn style(mut self, s: ComponentButtonStyle) -> Self {
        self.obj.style = Some(s);
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
