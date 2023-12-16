use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/input.txt").unwrap();
    println!("{}", compute(input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operation {
    Remove,
    Add(usize),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Lens {
    label: String,
    operation: Operation,
    lens_box: usize,
}
impl Lens {
    fn from_hascii(input: &str) -> Lens {
        let (operation, label) = if let Some(rest) = input.strip_suffix("-") {
            (Operation::Remove, rest.to_owned())
        } else {
            (0..256)
                .find_map(|i| {
                    if let Some(rest) = input.strip_suffix(&format!("={}", i)) {
                        Some((Operation::Add(i), rest.to_owned()))
                    } else {
                        None
                    }
                })
                .expect(&format!("To find =n in {}", input))
        };

        let lens_box = label.chars().fold(0, |acc, char| {
            assert!(char.is_ascii());
            ((char as usize + acc) * 17) % 256
        });

        Lens {
            label,
            operation,
            lens_box,
        }
    }
}

fn boxes(input: &str) -> Vec<Vec<Lens>> {
    let mut boxes: Vec<Vec<Lens>> = (0..256).map(|_| vec![]).collect();

    for lens in input.trim().split(",").map(Lens::from_hascii) {
        let lens_box = boxes.get_mut(lens.lens_box).unwrap();
        match lens.operation {
            Operation::Remove => lens_box.retain(|l| l.label != lens.label),
            Operation::Add(focal_length) => {
                if let Some(existing) = lens_box.iter_mut().find(|l| l.label == lens.label) {
                    existing.operation = Operation::Add(focal_length);
                } else {
                    lens_box.push(lens);
                }
            }
        }
    }

    boxes
}

fn compute(input: String) -> String {
    boxes(&input)
        .into_iter()
        .enumerate()
        .map(|(box_index, bx)| {
            bx.into_iter()
                .enumerate()
                .fold(0, |acc, (lens_index, lens)| {
                    acc + (box_index + 1) * (lens_index + 1) * {
                        let Operation::Add(focal_length) = lens.operation else {
                            panic!("Added a lense to a box without a focal length")
                        };
                        focal_length
                    }
                })
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_init() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_owned();
        assert_eq!("145", compute(input));
    }
}
