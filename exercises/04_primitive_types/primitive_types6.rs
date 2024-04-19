// primitive_types6.rs
//
// Utiliser un index de tuple pour accéder au second élément de `numbers`. Vous pouvez mettre l'expression
// l'expression pour le deuxième élément à l'endroit où se trouve ??? pour que le test réussisse.
//
// Exécutez `rustlings hint primitive_types6` ou utilisez la sous-commande `hint` watch
// pour un indice.

#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Remplacer le texte ci-dessous par la syntaxe d'indexation dU tuple.
    let second = numbers.1;

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}
