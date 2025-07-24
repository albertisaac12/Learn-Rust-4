
#[derive(Debug)]
struct SupermarketItem {
    name: String,
    price: f64
}


#[derive(Debug)]
struct ShoppingCart {
    items: Vec<SupermarketItem>
}

impl ShoppingCart {


    fn traverse_items<F>(&mut self,mut operation:F) where F: FnMut(&mut SupermarketItem) {

        let mut tally = 0;

        while tally< self.items.len() {
            
            operation(&mut self.items[tally]);
            tally+=1;
        }

    }


    fn checkout<F>(self,operation:F) where F: FnOnce(ShoppingCart) {
        operation(self);
    }
}

fn main() {

    let super1 = SupermarketItem {
        name: String::from("candy"),
        price: 100.0
    };
    let super2 = SupermarketItem {
        name: String::from("meow"),
        price: 200.0
    };
    let mut s1 = ShoppingCart {
        items: vec![super1,super2]
    };

    let closure = |item: &mut SupermarketItem|  {
        item.price = item.price - (item.price*(15.0/100.0));
    };

    s1.traverse_items(closure);

    println!("{:?}",s1);


    let mut total_price = 0.0;

    let closure = |input : ShoppingCart| {
        let mut shopping_Cart = input;
        let closure = |input: &mut SupermarketItem| {
            total_price+= input.price;
        };
        shopping_Cart.traverse_items(closure);

    };

    s1.checkout(closure);

    println!("The total Price is {}",total_price);

    // s1.checkout();



}