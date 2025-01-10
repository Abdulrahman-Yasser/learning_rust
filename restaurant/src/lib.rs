mod front_of_house{
    pub mod hostting{
        pub fn add_to_waitlist() {}
        fn seat_at_table(){}
    }
    mod serving{
        fn take_order(){}
        fn serve_order(){}
        fn take_payment(){}
    }
}


pub fn eat_at_restaurant(){
    crate::front_of_house::hostting::add_to_waitlist();
}