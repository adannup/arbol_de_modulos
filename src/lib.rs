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

mod back_of_house {
    fn fix_incorrect_order() {
        // Usar super nos permite hacer referencia a un item que sabemos que esta en el
        // modulo padre, lo que puede facilitar la reorganizacion del arbol de modulos
        // cuando el modulo esta estrechamente relacionado con el padre.
        super::deliver_order();
        cook_order();
    }

    fn cook_order() {}
}

fn deliver_order() {}

// Al agregar:
use crate::front_of_house::hosting;
// en la raiz del crate, hace que hosting sea ahora un nombre valido
// en ese ambito, como si el modulo hosting hubiera sido definifo en la raiz del crate.

mod customer {
    // Ten en cuenta que use solo crea el atajo para el ambito particular en que ocurre el use
    // por lo que al mover la funcion eat_at_restaurant() dentro de customer
    // hosting ya no estara dentro de este ambito

    // se puede solucionar de dos manera:
    // 1. moviendo use crate::front_of_house::hosting; en este ambito

    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        // 2. añadiendo super a la funcion:
        // super::hosting::add_to_waitlist();
        hosting::add_to_waitlist();

        // relative path
        // Comienza desde le modulo actual y utiliza self, super, o un identificador
        // del modulo actual.
        super::front_of_house::hosting::add_to_waitlist();
    }
}
