#[cfg(feature = "handler")]
use log::warn;
use serde::{Deserialize, Serialize};

#[cfg(feature = "handler")]
use crate::Builder;

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

impl Default for ComponentSelectOption{
    fn default() -> Self{
        Self{
            label: String::new(),
            value: String::new(),
            emoji: None,
            description: None,
            default: None,
        }
    }
}

impl ComponentSelectOption{
    /// Sets the option label
    pub fn label(mut self, lab: impl Into<String>) -> Self{
        self.label = lab.into();
        self
    }
    /// Sets the option value
    pub fn value(mut self, value: impl Into<String>)-> Self{
        self.value = value.into();
        self
    }
    /// Sets the option description
    pub fn description(mut self, des: impl Into<String>)-> Self{
        self.description = Some(des.into());
        self
    }
    /// Sets the default checked 
    pub fn set_default(mut self, default: bool) -> Self{
        self.default = Some(default);
        self
    }

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
impl Builder<MessageComponent> for ComponentRowBuilder{
    fn build(self) -> Result<MessageComponent, String>{
        Ok(self.obj)
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

    #[deprecated(
        since = "0.1.9",
        note = "Use the `build()` function instead"
    )]
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
    #[deprecated(
        since = "0.1.9",
        note = "Use the `build()` function instead"
    )]
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

#[cfg(feature = "handler")]
impl Builder<ComponentButton> for ComponentButtonBuilder{
    fn build(self) -> Result<ComponentButton, String>{
        if self.obj.style.is_none(){
            return Err("Style is None.".to_string())
        }
        match self.obj.clone().style.unwrap() {
            ComponentButtonStyle::Link => {
                if self.obj.url.is_none() {
                    return Err("The button style is set to 'Link', but no url was specified.".to_string());
                }
            }
            _ => {
                if self.obj.custom_id.is_none() {
                    return Err("No custom_id was supplied for this button!".to_string());
                }
            }
        }
        Ok(self.obj)
    }
}

#[cfg(feature = "handler")]
#[derive(Clone, Debug)]
/// Builder pattern for creating menu components.
pub struct ComponentSelectMenuBuilder{
    obj: ComponentSelectMenu
}

#[cfg(feature = "handler")]
impl Default for ComponentSelectMenuBuilder{
    fn default() -> Self{
        Self{
            obj: ComponentSelectMenu::default(),
        }
    }
}

#[cfg(feature = "handler")]
impl ComponentSelectMenuBuilder{
    /// The custom developer identifier. **SETTING THIS IS MANDATORY!**
    pub fn custom_id(mut self, id: impl Into<String>) -> Self{
        self.obj.custom_id = id.into();
        self
    }

    /// custom placeholder text if nothing is selected, max 100 characters
    pub fn placeholder(mut self, ph: impl Into<String>) -> Self{
        let p = ph.into();

        if p.chars().count() > 100 {
            warn!("Menu placeholder exceeded 100 characters, ignoring");
            return self;
        }
        self.obj.placeholder = Some(p);
        self
    }

    /// Add a menu choice 
    pub fn add_option(mut self, opt: ComponentSelectOption) -> Self{
        if self.obj.options.len() >= 25{
            warn!("This menu already contains 25 elements, ignoring");
            return self;
        }
        self.obj.options.push(opt);
        self
    }

    /// he minimum number of items that must be chosen; default 1, min 0, max 25
    pub fn min_values(mut self, min: impl Into<u8>) -> Self{
        self.obj.min_values = min.into();
        self
    }

    /// the maximum number of items that can be chosen; default 1, max 25
    pub fn max_values(mut self, max: impl Into<u8>) -> Self{
        self.obj.max_values = max.into();
        self
    }
}

#[cfg(feature = "handler")]
impl Builder<ComponentSelectMenu> for ComponentSelectMenuBuilder{
    fn build(self) -> Result<ComponentSelectMenu, String>{
        if self.obj.custom_id.is_empty(){
            return Err("custom_id is empty!".to_string());
        }
        if self.obj.options.len() > 25{
            return Err("Menu more than 25 options".to_string());
        }
        if &self.obj.min_values > &25{
            return Err("min_values cannot be more than 25".to_string());
        }
        if &self.obj.max_values > &25{
            return Err("max_values cannot be more than 25".to_string());
        }
        Ok(self.obj)
    }
}

