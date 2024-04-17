//cargo new restaurant --lib --vcs none
//A front_of_house module containing other modules that then contain functions

//use crate::front_of_house
mod front_of_house {
    //use crate::front_of_house::hosting;
    mod hosting {
        //use crate::front_of_house::hosting::add_to_waitlist;
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
