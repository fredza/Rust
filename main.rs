use std::io;
// sans verif, simple
// Fonction pour vérifier si une année est bissextile
fn est_bissextile(annee: i32) -> bool {
    (annee % 4 == 0 && annee % 100 != 0) || (annee % 400 == 0)
}

fn main() {
    let mut input = String::new();

    println!("Entrez une année :");

    io::stdin()
        .read_line(&mut input)
        .expect("Erreur de lecture");

    let annee: i32 = input.trim().parse().expect("Veuillez entrer un nombre valide");

    // Vérification
    if est_bissextile(annee) {
        println!("{} est une année bissextile ✅", annee);
    } else {
        println!("{} n'est pas une année bissextile ❌", annee);
    }

    // Trouver les 10 prochaines années bissextiles
    println!("\nLes 10 prochaines années bissextiles sont :");

    let mut count = 0;
    let mut annee_test = annee + 1;

    while count < 10 {
        if est_bissextile(annee_test) {
            println!("{}", annee_test);
            count += 1;
        }
        annee_test += 1;
    }
}