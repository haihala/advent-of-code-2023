use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/input.txt").unwrap();
    println!("{}", compute(input));
}

fn hascii(input: &str) -> usize {
    input.chars().fold(0, |acc, char| {
        assert!(char.is_ascii());
        ((char as usize + acc) * 17) % 256
    })
}

fn hasciis(input: &str) -> Vec<usize> {
    input.trim().split(",").map(hascii).collect()
}

fn compute(input: String) -> String {
    hasciis(&input).into_iter().sum::<usize>().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example() {
        let input = "HASH".to_owned();
        assert_eq!(52, hascii(&input));
    }

    #[test]
    fn test_init() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_owned();
        assert_eq!(
            vec![30, 253, 97, 47, 14, 180, 9, 197, 48, 214, 231],
            hasciis(&input)
        );

        assert_eq!("1320", compute(input));
    }
}
