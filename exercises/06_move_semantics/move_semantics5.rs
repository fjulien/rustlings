// move_semantics5.rs
//
// Faites-moi compiler seulement en réordonnant les lignes dans `main()`, mais sans ajouter, // changer ou supprimer aucune d'entre elles,
// modifier ou supprimer aucune d'entre elles.
//
// Exécutez `rustlings hint move_semantics5` ou utilisez la sous-commande `hint` watch
// pour un indice.

#[test]
fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
