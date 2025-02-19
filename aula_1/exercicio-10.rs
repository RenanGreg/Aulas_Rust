fn main() {
    let vetor1 = vec![1, 2, 3];
    let vetor2 = vec![4, 5, 6];
    let combinado = [&vetor1[..], &vetor2[..]].concat();

    println!("{:?}", combinado); // imprimi juntando os dois vetores [1, 2, 3, 4, 5, 6]
} 
