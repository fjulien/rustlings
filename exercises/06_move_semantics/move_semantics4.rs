// move_semantics4.rs
//
// Refondre ce code pour qu'au lieu de passer `vec0` dans la fonction `fill_vec`
// le vecteur est créé dans la fonction elle-même et renvoyé à la fonction
// la fonction principale.
//
// Exécutez `rustlings hint move_semantics4` ou utilisez la sous-commande `hint` watch
// pour un indice.

#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(vec0);

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

// `fill_vec()` ne prend plus `vec : Vec<i32>` comme argument - ne changez pas cela !
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    // Au lieu de cela, créons et remplissons le Vec ici - comment faire ?
    let mut responce = vec.clone();
    responce.push(88);
    responce
}
