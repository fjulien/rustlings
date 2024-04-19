// move_semantics2.rs
//
// Faire passer le test en trouvant un moyen de garder les deux Vecs séparées !
//
// Exécutez `rustlings hint move_semantics2` ou utilisez la sous-commande `hint` watch
// pour un indice.

#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(vec0.clone());

    assert_eq!(vec0, vec![22, 44, 66]);
    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}
