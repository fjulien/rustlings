// vecs2.rs
//
// Un Vec de nombres pairs est donné. Votre tâche est de compléter la boucle de manière à ce que
// chaque nombre du Vec soit multiplié par 2.
//
// Faites-moi passer le test !
//
// Exécutez `rustlings hint vecs2` ou utilisez la sous-commande watch `hint` pour obtenir un indice.

fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        // TODO : Remplir ceci pour que chaque élément du Vec `v` soit
        // multiplié par 2.
       *element = *element * 2;
    }

    // A ce stade, `v` devrait être égal à [4, 8, 12, 16, 20].
    v
}

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter().map(|element| {
        // TODO : Faites la même chose que ci-dessus - mais au lieu de muter le Vec, vous pouvez simplement renvoyer le nouveau nombre !
        // Vec, vous pouvez simplement retourner le nouveau nombre !
        element * 2 
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
