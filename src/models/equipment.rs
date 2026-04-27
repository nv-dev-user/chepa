use super::item::Item;

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Equipment {
    item: Item,
    coef_modifier: u32,
}

impl Equipment {
    pub fn new(id: u32, name: String, coef_modifier: u32) -> Self {
        Equipment { item: Item::new(id, name), coef_modifier }
    }

    pub fn new_from_item(item: Item, coef_modifier: u32) -> Self {
        Equipment { item, coef_modifier }
    }

    pub fn get_id(&self) -> u32 {
        self.item.get_id()
    }

    pub fn get_name(&self) -> &str {
        self.item.get_name()
    }

    pub fn get_coef_modifier(&self) -> u32 {
        self.coef_modifier
    }
}
