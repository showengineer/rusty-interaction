use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};

use super::components::MessageComponent;
use serde_with::*;

#[cfg(feature = "builder")]
use crate::Builder;
#[cfg(feature = "builder")]
use log::warn;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Modal {
    custom_id: String,
    title: String,
    components: Vec<MessageComponent>,
}

#[cfg(feature = "builder")]
#[derive(Clone, Debug, Default)]
pub struct ModalBuilder {
    obj: Modal,
}

#[cfg(feature = "builder")]
impl ModalBuilder {
    pub fn custom_id(mut self, id: String) -> Self {
        if id.len() > 100 {
            warn!("Exceeding maximum id char count (100), ignoring");
            return self;
        }
        self.obj.custom_id = id;
        self
    }

    pub fn title(mut self, title: String) -> Self {
        self.obj.title = title;
        self
    }

    pub fn add_component(mut self, component: impl Into<MessageComponent>) -> Self {
        if self.obj.components.len() >= 5 {
            warn!("Exceeding maximum modal component count (5), ignoring");
            return self;
        }
        self.obj.components.push(component.into());
        self
    }
}

#[cfg(feature = "builder")]
#[derive(Clone, Debug)]
pub enum ModalConversionError {
    MissingCustomId,
    MissingTitle,
    MissingComponents,
    TooMuchComponents,
}

#[cfg(feature = "builder")]
impl fmt::Display for ModalConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ModalConversionError::MissingCustomId => {
                write!(f, "Missing a custom id for modal!")
            }
            ModalConversionError::MissingTitle => {
                write!(f, "Missing a title for modal!")
            }
            ModalConversionError::MissingComponents => {
                write!(f, "Modal does not contain any components!")
            }
            ModalConversionError::TooMuchComponents => {
                write!(f, "Modal contains too much components!")
            }
        }
    }
}

#[cfg(feature = "builder")]
impl Error for ModalConversionError {}

#[cfg(feature = "builder")]
impl Builder<Modal> for ModalBuilder {
    type Error = ModalConversionError;

    fn build(self) -> Result<Modal, Self::Error> {
        if self.obj.custom_id.len() < 1 {
            return Err(ModalConversionError::MissingCustomId);
        }
        if self.obj.title.len() < 1 {
            return Err(ModalConversionError::MissingTitle);
        }
        if self.obj.components.len() < 1 {
            return Err(ModalConversionError::MissingComponents);
        }
        if self.obj.components.len() > 5 {
            return Err(ModalConversionError::TooMuchComponents);
        }

        return Ok(self.obj);
    }
}
