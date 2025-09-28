#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}

impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        if let Some(product) = s.products.iter().find(|(name, _)| *name == ele) {
            self.items.push(product.clone());
        }
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut items_prices: Vec<f32> = self.items.iter().map(|(_, price)| *price).collect();
        items_prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let original_total: f32 = items_prices.iter().sum();
        let free_items_count = items_prices.len() / 3;
        let discount: f32 = items_prices.iter().take(free_items_count).sum();
        let discounted_total = original_total - discount;

        let adjustment_factor = discounted_total / original_total;
        let mut adjusted_prices: Vec<f32> = items_prices
            .iter()
            .map(|price| (price * adjustment_factor * 100.0).round() / 100.0)
            .collect();

        adjusted_prices.sort_by(|a, b| a.partial_cmp(b).unwrap());
        self.receipt = adjusted_prices.clone();
        adjusted_prices
    }
}
