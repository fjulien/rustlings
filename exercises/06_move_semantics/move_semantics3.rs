// move_semantics3.rs
//
// Faites-moi compiler sans ajouter de nouvelles lignes -- juste en changeant les lignes existantes ! (non
// lignes avec plusieurs points-virgules nécessaires !)
//
// Exécutez `rustlings hint move_semantics3` ou utilisez la sous-commande `hint` watch
// pour un indice.

#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(vec0);

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);

    vec
}
