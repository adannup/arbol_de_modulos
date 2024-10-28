fn fix_incorrect_order() {
    // Usar super nos permite hacer referencia a un item que sabemos que esta en el
    // modulo padre, lo que puede facilitar la reorganizacion del arbol de modulos
    // cuando el modulo esta estrechamente relacionado con el padre.
    super::deliver_order();
    cook_order();
}

fn cook_order() {}
