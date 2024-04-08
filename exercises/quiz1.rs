// quiz1.rs
//
// Il s'agit d'un quiz pour les sections suivantes :
// - Variables
// - Les fonctions
// - Si
//
// Marie achète des pommes. Le prix d'une pomme est calculé comme suit :
// - Une pomme coûte 2 rustbucks.
// - Si Marie achète plus de 40 pommes, chaque pomme ne coûte que 1 rustbuck !
// Ecrivez une fonction qui calcule le prix d'une commande de pommes en fonction de 
// la quantité achetée.
//
// Pas d'indices cette fois-ci ;)

// Placez votre fonction ici !
fn calculate_price_of_apples(prix :i32) -> i32{
    if prix > 40 {
        prix * 1
    } else {
        prix * 2
    }
}

// fn calculer_le_prix_des_pommes {
// Ne modifiez pas cette fonction !
#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(41, price3);
    assert_eq!(65, price4);
}
