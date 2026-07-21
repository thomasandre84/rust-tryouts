mod content;

use content::media::Media;
use content::catalog::Catalog;

fn main() {
    let mut catalog = Catalog::new();
    catalog.add_item("The Great Gatsby".to_string());
    catalog.add_item("Inception".to_string());
    catalog.add_item("Blade Runner".to_string());

    for item in catalog.list_items() {
        println!("{}", item);
    }
}
