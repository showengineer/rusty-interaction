use serde::{Deserialize, Serialize};

use serde_with::*;

use ::chrono::{DateTime, Utc};
use serde_with::*;


use log::{error};

// ======== Structures =========
#[serde_as]
#[skip_serializing_none]
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// An embed in Discord is a way to display rich content in messages
pub struct Embed {
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    #[serde_as(as = "Option<DisplayFromStr>")]
    #[serde(default)]
    pub timestamp: Option<DateTime<Utc>>,
    pub color: Option<u32>,
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    pub thumbnail: Option<EmbedThumbnail>,
    pub video: Option<EmbedVideo>,
    pub provider: Option<EmbedProvider>,
    pub author: Option<EmbedAuthor>,
    pub fields: Option<Vec<EmbedField>>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct EmbedThumbnail {
    url: String,
    proxy_url: String,
    height: i32,
    width: i32,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct EmbedVideo {
    url: String,
    proxy_url: String,
    height: i32,
    witdh: i32,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct EmbedImage {
    url: String,
    proxy_url: String,
    height: i32,
    width: i32,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct EmbedProvider {
    name: String,
    url: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct EmbedAuthor {
    name: Option<String>,
    url: Option<String>,
    icon_url: Option<String>,
    proxy_icon_url: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct EmbedFooter {
    text: String,
    icon_url: Option<String>,
    proxy_icon_url: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct EmbedField {
    name: String,
    value: String,
    inline: Option<bool>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct EmbedBuilder{
    obj: Embed
}
#[derive(Clone, Copy, Debug, PartialEq)]
/// Representing RGB colors. 
pub struct Color{
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}



// ========== IMPLS ===========
impl Default for Color{
    fn default() -> Self{
        Color{
            // ;)
            red: 222,
            green: 165,
            blue: 132,
        }

    }
}

impl From<u32> for Color{
    fn from(a: u32) -> Color{
        Color{
            red: ((a >> 16) & 0xff) as u8,
            green: ((a >> 8) & 0xff) as u8,
            blue: (a & 0xff) as u8,
        }
    }
}

impl Into<u32> for Color{
    fn into(self) -> u32{
        ((self.red as u32) << 16) | ((self.green as u32) << 8) | self.blue as u32
    }
}

impl Default for Embed{
    fn default() -> Self{
        Self{
            title: None,
            description: None,
            url: None,
            timestamp: None,
            color: Some(Color::default().into()),
            footer: None,
            image: None,
            thumbnail: None,
            video: None,
            provider: None,
            author: None,
            fields: None,
        }
    }
}

impl Default for EmbedBuilder{
    fn default() -> Self{
        Self{
            obj: Embed::default(),
        }
    }
}

impl EmbedBuilder{
    /// Set the title of this embed
    pub fn title(mut self, title: impl ToString) -> Self{
        let t = title.to_string();
        // wish this could be checked at compile time :(
        if t.len() > 256{
            panic!("Embed title length is more than 256 characters.")
        }
        self.obj.title = Some(t);
        self
    }

    /// Set the url of the title of this embed
    pub fn url(mut self, url: &str) -> Self{
        self.obj.url = Some(String::from(url));
        self
    }
    /// Set the color of this embed
    pub fn color(mut self, color: impl Into<u32>) -> Self{
        self.obj.color = Some(color.into());
        self
    }

    /// Set the timestamp of this embed
    pub fn timestamp(mut self, timestamp: DateTime<Utc>) -> Self{
        self.obj.timestamp = Some(timestamp);
        self
    }

    /// Set the embed's footer
    pub fn footer(mut self, a: EmbedFooter) -> Self{
        self.obj.footer = Some(a);
        self
    }

    /// Set the embed author
    pub fn author(mut self, author: EmbedAuthor) -> Self{
        self.obj.author = Some(author);
        self
    }

    /// Add an [`EmbedField`] to this embed. 
    pub fn add_field(mut self, field: EmbedField) -> Self{
        match self.obj.fields{
            None => {
                let nf = Some(vec![field]);
                self.obj.fields = nf;
            },
            Some(ref mut f) =>{
                if f.len() >= 25{
                    error!("Field limit reached. Ignoring");
                }
                else{
                    f.push(field);
                }
            }
        }
        self
    }

    /// Build the embed. You can't use the function after this anymore
    pub fn finish(self) -> Embed{
        self.obj
    } 


}

impl Default for EmbedFooter{
    fn default() -> Self{
        Self{
            text: String::from(""),
            icon_url: None,
            proxy_icon_url: None,
        }
    }
}

impl EmbedFooter{
    /// Set the footers text
    pub fn text(mut self, text: impl ToString) -> Self{
        let t = text.to_string();
        if t.len() > 2048{
            panic!("Footer text exceeded 2048 characters")
        }
        self.text = t;
        self
    }

    /// Sets the url to the footer icon
    pub fn icon_url(mut self, url: impl ToString) -> Self{
        let n = url.to_string();

        self.icon_url = Some(n);
        self
    }

    pub fn proxy(mut self, url: impl ToString) -> Self{
        let u = url.to_string();

        self.proxy_icon_url = Some(u);
        self
    }
}

impl Default for EmbedField{
    fn default() -> Self{
        Self{
            value: String::from(""),
            name: String::from(""),
            inline: None,
        }
    }
}

impl EmbedField{
    /// Set the field name
    pub fn name(mut self, name: impl ToString) -> Self{
        let n = name.to_string();
        if n.len() > 256{
            panic!("Field name is above 256 characters.")
        }
        self.name = n;
        self
    }

    /// Set the text of this field
    pub fn value(mut self, text: impl ToString) -> Self{
        let t = text.to_string();

        if t.len() > 1024{
            panic!("Field value is above 1024 characters")
        }
        self.value = t;
        self
    }
    /// Set if the field should display inline
    pub fn inline(mut self, inline: bool) -> Self {
        self.inline = Some(inline);
        self
    }
}