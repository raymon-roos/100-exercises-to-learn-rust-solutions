// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

#[derive(Debug)]
pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order {
    fn validate_name(s: &str) -> &str {
        match s.len() {
            0 => panic!("Product name cannot be empty."),
            300.. => panic!("Product name cannot be longer than 300 bytes."),
            _ => s,
        }
    }

    fn validate_quantity(u: u32) -> u32 {
        match u {
            0 => panic!("Quantity must be greater than zero."),
            1.. => u,
        }
    }

    fn validate_price(u: u32) -> u32 {
        match u {
            0 => panic!("Unit price must be greater than zero."),
            1.. => u,
        }
    }

    pub fn set_product_name(&mut self, name: String) {
        Self::validate_name(&name);
        self.product_name = name;
    }

    pub fn set_quantity(&mut self, quantity: u32) {
        Self::validate_quantity(quantity);
        self.quantity = quantity;
    }

    pub fn set_unit_price(&mut self, price: u32) {
        Self::validate_price(price);
        self.unit_price = price;
    }

    pub fn product_name(&self) -> &str {
        &self.product_name
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Self {
        Self::validate_name(&product_name);
        Self::validate_price(quantity);
        Self::validate_price(unit_price);

        Self {
            product_name,
            quantity,
            unit_price,
        }
    }

    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }
}
