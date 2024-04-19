// primitive_types5.rs
//
// Déstructure le tuple `cat` pour que le println fonctionne.
//
// Exécutez `rustlings hint primitive_types5` ou utilisez la sous-commande `hint` watch
// pour un indice.

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let ( name, age ) /* votre modèle ici */ = cat;

    println!("{} is {} years old.", name, age);
}
