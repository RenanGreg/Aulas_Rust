fn main() {
    let frutas = vec!["maçã", "banana", "laranja"];
    let procura = "banana";

    if frutas.contains(&procura) {
        println!("{} está no vetor.", procura);
    } else {
        println!("{} não está no vetor.", procura);
    }
}
