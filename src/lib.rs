mod rna {
pub fn trans(dna : &String) -> String {
    fn replace(acid : char) -> char {
        if acid == 'T' {return 'U'}
        else {return acid}
    } // we create a function to replace T with U
    dna.chars().map(|x| replace(x)).collect() // first we call the chars() method to iterate
    // then we use map to apply replace function on each chara. Then we use collect to make a new string
    // that has the properties that map has been applied 
}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rnatest1() {
        let mut dna : String = String::from("GATGGAACTTGACTACGTAAATT");
        let correct: String = String::from("GAUGGAACUUGACUACGUAAAUU");
        let result: String = rna::trans(&mut dna);
        assert_eq!(result, correct);
    }
    #[test]
    fn rnatest2() {
        let mut dna: String = String::from("ACTGCAGTCTGAACGGCCAGCTCAAGAGATC");
        let correct: String = String::from("ACUGCAGUCUGAACGGCCAGCUCAAGAGAUC");
        let result: String = rna::trans(&mut dna);
        assert_eq!(result, correct);
    }
}

