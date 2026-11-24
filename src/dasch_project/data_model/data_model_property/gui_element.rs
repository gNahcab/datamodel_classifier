use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum GuiElement {
    List,
    Date,
    Searchbox,
    SimpleText,
    Textarea,
    Geolocation,
}