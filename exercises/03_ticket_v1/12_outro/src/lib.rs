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
pub struct Order {
    product_name: String,
    quantity: i32,
    unit_price: i32
}
impl Order {
    pub fn new(product_name: String, quantity: i32, unit_price: i32) -> Order {
        validate_quantity(&quantity);
        validate_name(&product_name);
        validate_unit_price(&unit_price);

        Order {
            product_name,
            quantity,
            unit_price
        }
    }
    pub fn total(&self) -> i32 {
        return &self.quantity * &self.unit_price
     }
    pub fn product_name(&self) -> &String{ &self.product_name }
    pub fn quantity(&self) -> &i32 { &self.quantity }
    pub fn unit_price(&self) -> &i32 { &self.unit_price }
    pub fn set_product_name(&mut self, new_name: String) {
        validate_name(&new_name);
        self.product_name = new_name;
    }
    pub fn set_quantity(&mut self, new_quantity: i32) {
        validate_quantity(&new_quantity);
        self.quantity = new_quantity;
    }
    pub fn set_unit_price(&mut self, unit_price: i32) {
        validate_unit_price(&unit_price);
        self.unit_price = unit_price;
    }
}
fn validate_name(name: &String) {
    if name == "" {
        panic!("product name cannot be empty!")
    }
    if name.len() > 300 {
        panic!("Product name cannot be longer than 300 characters!")
    }
}
fn validate_quantity(quantity: &i32) {
    if quantity < &1 {
        panic!("the quantity should be greater than 0!")
    }
}
fn validate_unit_price(price: &i32) {
    if price < &1 {
        panic!("the unit price should be greater than 0!")
    }
}