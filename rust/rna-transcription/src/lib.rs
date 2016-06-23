
#[derive(Debug, PartialEq)]
pub struct RibonucleicAcid {
    nucleotides: String,
}

impl RibonucleicAcid {
    pub fn new(nucleotides: &str) -> RibonucleicAcid {
        RibonucleicAcid { nucleotides: nucleotides.to_string() }
    }
}

#[derive(Debug, PartialEq)]
pub struct DeoxyribonucleicAcid {
    nucleotides: String,
}

impl DeoxyribonucleicAcid {
    pub fn new(nucleotides: &str) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid { nucleotides: nucleotides.to_string() }
    }

    pub fn to_rna(&self) -> RibonucleicAcid {
        let rna_nucleotides = self.nucleotides
            .chars()
            .map(|n| match n {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                _ => unreachable!(),
            })
            .collect::<String>();
        RibonucleicAcid::new(rna_nucleotides.as_ref())
    }
}
