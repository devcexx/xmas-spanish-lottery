mod awards;
mod currency;
pub use awards::*;
pub use currency::*;
use derive_new::new;

pub type Amount = MonetaryAmount<Euro>;
pub type LotteryNumber = u32;

/// Represents a number that has been played on the lottery, with an specific bet.
#[derive(Clone, Debug, new)]
pub struct PlayedNumber {
    number: LotteryNumber,
    bet: Amount,
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        fs::File,
        io::{BufRead, BufReader},
        path::Path,
    };

    use super::ExtractedAwardedNumberSliceExt;
    use super::*;

    struct NumAward {
        number: LotteryNumber,
        amount_per_serie: Amount,
    }

    pub fn read_csv(path: &Path) -> impl Iterator<Item = Vec<String>> {
        let lines = BufReader::new(File::open(path).unwrap()).lines();

        lines.into_iter().map(|result| result.unwrap()).map(|line| {
            line.split(",")
                .into_iter()
                .map(|elem| elem.to_owned())
                .collect::<Vec<String>>()
        })
    }

    #[test]
    fn it_correctly_calculates_awards_amounts() {
        let awarded_nums = read_csv(Path::new("test-data/awarded-nums-2021.csv"))
            .map(|entry| {
                println!("{:?}", entry);
                let elem: &str = &entry[1];
                let kind = match elem {
                    "first" => ExtractedAwardKind::First,
                    "second" => ExtractedAwardKind::Second,
                    "third" => ExtractedAwardKind::Third,
                    "fourth" => ExtractedAwardKind::Fourth,
                    "fifth" => ExtractedAwardKind::Fifth,
                    "little" => ExtractedAwardKind::Little,
                    other => panic!("Invalid award type on test data: {}", other),
                };
                let number = entry[0].parse().unwrap();
                ExtractedAwardedNumber::new(kind, number)
            })
            .collect::<Vec<ExtractedAwardedNumber>>();

        let all_awards = read_csv(Path::new("test-data/all-awards-2021.csv"))
            .map(|entry| {
                let num = entry[0].parse().unwrap();
                (
                    num,
                    NumAward {
                        number: num,
                        amount_per_serie: Amount::from_euros(entry[1].parse().unwrap()),
                    },
                )
            })
            .collect::<HashMap<LotteryNumber, NumAward>>();

        for playing_number in 0..100000 {
            let derived_awards = (&awarded_nums[..])
                .get_derived_awards(&PlayedNumber::new(playing_number, Amount::from_euros(200)));
            let total_earned: Amount = derived_awards
                .iter()
                .fold(Default::default(), |l, r| l + r.get_total_earned());
            let expected_award = all_awards
                .get(&playing_number)
                .map(|award| award.amount_per_serie)
                .unwrap_or_default();

            assert_eq!(expected_award, total_earned, "Expected number {} to be awarded with {}, but got {} instead. Gotten awards for this number: {:?}", playing_number, expected_award, total_earned, derived_awards);
        }
    }
}
