// --- Newtype Pattern ---

#[derive(Debug, Clone, PartialEq)]
pub struct UserId(u32);

#[derive(Debug, Clone, PartialEq)]
pub struct OrderId(u32);

// --- Typestate Pattern ---

// States
pub struct Created;
pub struct Paid;
pub struct Shipped;

// The main struct
pub struct Order<State> {
    id: OrderId,
    amount: u64,
    _state: std::marker::PhantomData<State>,
}

// Implementation for Created state
impl Order<Created> {
    pub fn new(id: OrderId, amount: u64) -> Self {
        Self {
            id,
            amount,
            _state: std::marker::PhantomData,
        }
    }

    pub fn pay(self) -> Order<Paid> {
        println!("Order {:?} is being paid.", self.id);
        Order {
            id: self.id,
            amount: self.amount,
            _state: std::marker::PhantomData,
        }
    }
}

// Implementation for Paid state
impl Order<Paid> {
    pub fn ship(self) -> Order<Shipped> {
        println!("Order {:?} is being shipped.", self.id);
        Order {
            id: self.id,
            amount: self.amount,
            _state: std::marker::PhantomData,
        }
    }
}

// Implementation for Shipped state
impl Order<Shipped> {
    pub fn deliver(self) {
        println!("Order {:?} has been delivered!", self.id);
    }
}

// --- Main Demo ---

fn main() {
    println!("--- Advanced Patterns Demo ---");

    // 1. Newtype Demo
    let user_id = UserId(1);
    let order_id = OrderId(100);
    println!("Newtype: User ID: {:?}, Order ID: {:?}", user_id, order_id);
    // let error: UserId = order_id; // This would cause a compile error!

    // 2. Typestate Demo
    println!("\n--- Typestate Demo ---");
    let order = Order::new(order_id, 5000);
    
    // order.ship(); // COMPILE ERROR: Method `ship` not found in `Order<Created>`
    
    let paid_order = order.pay();
    // paid_order.deliver(); // COMPILE ERROR: Method `deliver` not found in `Order<Paid>`
    
    let shipped_order = paid_order.ship();
    shipped_order.deliver();

    println!("--- Demo Finished ---");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_newtype() {
        let u = UserId(10);
        let o = OrderId(20);
        assert_eq!(u, UserId(10));
        assert_eq!(o, OrderId(20));
    }

    #[test]
    fn test_typestate_flow() {
        let id = OrderId(1);
        let order = Order::new(id, 100);
        let paid = order.pay();
        let shipped = paid.ship();
        // We can't call deliver on 'paid', only on 'shipped'
        // This test implicitly verifies the flow works.
    }
}
