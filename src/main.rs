fn main() {
    let dna = "ABCD";
    println!("{}", dna_strand(dna));
}

fn dna_strand(dna: &str) -> String {
    dna.split("")
        .map(|c| match c {
            "A" => "T",
            "T" => "A",
            "G" => "C",
            "C" => "G",
            _ => c,
        })
        .collect()
}
