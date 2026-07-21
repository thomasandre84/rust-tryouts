#[derive(Debug)]
pub struct Catalog {
     pub items: Vec<String>,
 }

 impl Catalog {
     pub fn new() -> Self {
         Catalog { items: vec![] }
     }

     pub fn add_item(&mut self, item: String) {
         self.items.push(item);
     }

     pub fn list_items(&self) -> Vec<String> {
         self.items.clone()
     }
 }