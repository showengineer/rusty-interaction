use serde::{Deserialize, Serialize};

use serde_with::*;

use super::Snowflake;
use super::Builder;
use serde_repr::*;

/// Message components are a framework for adding interactive elements to the messages your app or bot sends. They're accessible, customizable, and easy to use.
pub struct MessageComponent{
    /// Type of component
    r#type: ComponentType,
    style: Option<u8>,
    label: Option<String>,
    // pub emoji: Option<Emoji>,
    custom_id: Option<String>,
    url: Option<String>,
    disabled: Option<bool>,
    components: Option<Vec<MessageComponent>>,
}

impl Default for MessageComponent{
    fn default() -> Self{
        Self{
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
pub enum ComponentType{
    /// A container for other components
    ActionRow = 1,
    /// A clickable button
    Button = 2,
}

/// Builder for making an button component 
pub struct ComponentButtonBuilder{
    obj: MessageComponent,
}

impl Default for ComponentButtonBuilder{
    fn default() -> Self{
        let mut ob = MessageComponent::default();
        ob.r#type = ComponentType::Button;

        Self{
            obj: ob,
        }
    }
}

impl Builder<MessageComponent> for ComponentButtonBuilder{
    fn build(self) -> MessageComponent{
        self.obj
    }
}