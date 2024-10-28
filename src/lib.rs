mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // absolute path
    // Ruta completa que comienza desde la raiz de un crate; para el codigo
    // de un crate externo, la ruta absoluta comienza con el nombre del crate,
    // y para el codigo de un crate actual, comienza con el crate literal.
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    // Comienza desde le modulo actual y utiliza self, super, o un identificador
    // del modulo actual.
    front_of_house::hosting::add_to_waitlist();
}
