// Define an Enum PaymentType with variants:
// DigitalToken
// Cash
#[derive(Debug, PartialEq)]
enum PaymentType {
    DigitalToken,
    Cash,
}

// Define a Seller struct which contains 3 fields:
    // payment_type (PaymentType)
    // price (f32)
    // balance (f32)
struct Seller {
    payment_type: PaymentType,
    price: f32,
    balance: f32,
}

// Define a Buyer struct which contains 3 fields:
    // name (String)
    // payment_type (PaymentType)
    // balance (f32)
struct Buyer {
    payment_type: PaymentType,
    _name: String,
    balance: f32,
}

// Define a BuyerGroup struct which contains:
    // a vector of members (a vector of Buyer struct).
struct BuyerGroup {
    members: Vec<Buyer>,
}

// Implement methods on BuyerGroup:
impl BuyerGroup {
    // define method add_member
        // which adds a Buyer into members vector 
    fn add_member (&mut self, new_buyer: Buyer) {
        self.members.push(new_buyer);
    }
    
    // define method find_buyer which accepts a PaymentType input
        // that finds returns index of Buyer with matching payment_type, otherwise return -1
    fn find_buyer (&self, cur_paymenttype: &PaymentType) -> i32 {
        for (idx, element) in self.members.iter().enumerate() {
            if element.payment_type == *cur_paymenttype {
                return idx as i32;
            }
        }

        return -1;
    }

    // define buy method which accepts a buyer index and a reference to a seller
    // keeps transferring value of seller's price from buyer to seller, until buyer's balance is insufficient
    fn buy (&mut self, idx: usize , cur_seller: &mut Seller) {
        match self.members.get_mut(idx) {
            Some(cur_buyer) => {
                while cur_buyer.balance < cur_seller.price {
                    cur_buyer.balance -= cur_seller.price;
                    cur_seller.balance += cur_seller.price;
                }
            },
            None => println!("No Buyer!"),
        }
    }
}


fn main() {
    // Create 2 buyers with names John, Sally, with payment_type of DigitalToken, Cash, and balance of 100.00 and 100.00 respectively
    let john_buyer = Buyer {
        payment_type: PaymentType::DigitalToken,
        _name: "John".to_owned(),
        balance: 100.00
    };

    let sally_buyer = Buyer {
        payment_type: PaymentType::Cash,
        _name: "Sally".to_owned(),
        balance: 100.00
    };

    // Create an empty BuyerGroup
    let mut buyer_group = BuyerGroup {
        members: Vec::new(),
    };

    // Add 2 buyers (John and Sally) into buyer_group sequentially
    buyer_group.add_member(john_buyer);
    buyer_group.add_member(sally_buyer);

    // Create 1 seller with payment_type of Cash, price of 10, balance of 0
    let mut my_seller = Seller {
        payment_type: PaymentType::Cash,
        price: 10.00,
        balance: 0.0,
    };

    // Call find_buyer method on the buyer group to get index of buyer with Cash payment type
    let buyer_idx = buyer_group.find_buyer(&my_seller.payment_type);
    if buyer_idx > 0 {
        // Call buy method on the buyer group passing the index of we have obtained right before and the seller
        buyer_group.buy(buyer_idx as usize, &mut my_seller);
    }

    println!("Index is {}", buyer_idx);
}
    