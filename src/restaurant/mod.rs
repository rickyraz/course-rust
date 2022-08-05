mod pizza_order{
    pub struct Pizza {
        pub dough : String,
        pub cheese : String,
         pub topping : String,
    }
    impl Pizza {
        pub fn lunch(topping: &str) -> Pizza{
            Pizza  {
                dough: String::from("Regular dough"),
                cheese: String::from("mozarella"),
                topping:String::from(topping),
            }
        }
    }

    pub mod help_cutomer {
        fn seat_at_a_table(){
            println!("Customer seated at a table");
        }
        pub fn take_order(){
            seat_at_a_table();
            let cust_pizza : super::Pizza = super::Pizza::lunch("veggies");
            serve_customer(cust_pizza)
        }
        fn serve_customer(cutz_pizza: super::Pizza){
            println!("The customer is served a regular pizza with {}", cutz_pizza.topping);
        }
    }
}

pub fn order_food(){
    crate::restaurant::pizza_order::help_cutomer::take_order();
}
