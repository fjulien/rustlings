// primitive_types3.rs
//
// Créez un tableau d'au moins 100 éléments dans lequel se trouve le ???.
//
// Exécutez `rustlings hint primitive_types3` ou utilisez la sous-commande `hint` watch
// pour un indice.

fn main() {
    let a = [1; 101];

    if a.len() >= 100 {
        println!("Wow, c'est un grand nombre !");
    } else {
        println!("Meh, je mange des tableaux comme ça au petit déjeuner.");
        panic!("Le tableau n'est pas assez grand, il faut plus d'éléments")
    }
}
