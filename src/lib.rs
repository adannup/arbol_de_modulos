mod back_of_house;
mod front_of_house;

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
