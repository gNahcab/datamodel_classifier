pub struct GuiAttributes {
    hlist_name: String
}
pub enum GuiElement {
    List(GuiAttributes),
    Date,
    Searchbox,
}