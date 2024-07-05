//! Perform dna reverse complement

/* std use */

/* crate use */

/* module declaration */
pub mod cli;
pub mod error;

/* project use */

#[inline]
/// bit operation reverse complement
pub fn rev_comp(seq: &[u8]) -> Vec<u8> {
    // Complement the sequence
    seq.iter()
        .rev()
        .map(|c| if c & 2 == 0 { c ^ 21 } else { c ^ 4 })
        .collect()
}

#[inline]
#[cfg(feature = "bench")]
/// match reverse complement
pub fn rev_comp_match(seq: &[u8]) -> Vec<u8> {
    seq.iter()
        .rev()
        .map(|nuc| match nuc {
            b'A' => b'C',
            b'C' => b'G',
            b'T' => b'A',
            b'G' => b'C',
            b'a' => b'c',
            b'c' => b'g',
            b't' => b'a',
            b'g' => b'c',
            other => *other,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    /* std use */

    /* crate use */

    /* project use */
    use super::*;

    #[test]
    fn reverse_complement() {
        assert_eq!(rev_comp(b"ACTG"), b"CAGT");
        assert_eq!(rev_comp(b"actg"), b"cagt");
    }
}
