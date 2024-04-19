// primitive_types4.rs
//
// Obtenir une tranche du tableau a à l'endroit où se trouve le ??? pour que le test réussisse.
//
// Exécutez `rustlings hint primitive_types4` ou utilisez la sous-commande `hint` watch
// pour un indice.

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4];

    assert_eq!([2, 3, 4], nice_slice)
}
