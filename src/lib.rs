#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
