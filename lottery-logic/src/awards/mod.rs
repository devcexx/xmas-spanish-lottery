mod award_spec;
use crate::{Amount, CurrencyEuroExt, LotteryNumber, PlayedNumber};
pub use award_spec::*;
use derive_new::new;

pub trait ExtractedAwardedNumberSliceExt {
    fn get_derived_awards(&self, played: &PlayedNumber) -> Vec<DerivedAwardedNumber>;
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
/// The different kind of awards that an extracted number can be awarded with.
pub enum ExtractedAwardKind {
    First = 1,
    Second = 2,
    Third = 3,
    Fourth = 4,
    Fifth = 5,
    Little = 6,
}

impl ExtractedAwardKind {
    pub fn get_award_spec(self) -> &'static dyn ExtractedAwardKindSpec {
        match self {
            ExtractedAwardKind::First => AwardSpecs::FIRST_AWARD,
            ExtractedAwardKind::Second => AwardSpecs::SECOND_AWARD,
            ExtractedAwardKind::Third => AwardSpecs::THIRD_AWARD,
            ExtractedAwardKind::Fourth => AwardSpecs::FOURTH_AWARD,
            ExtractedAwardKind::Fifth => AwardSpecs::FIFTH_AWARD,
            ExtractedAwardKind::Little => AwardSpecs::LITTLE_AWARD,
        }
    }
}

/// Represents an awarded number that have been extracted during the
/// lottery.
#[derive(Clone, Debug, new)]
pub struct ExtractedAwardedNumber {
    kind: ExtractedAwardKind,
    number: LotteryNumber,
}

impl ExtractedAwardedNumber {
    /// Get the awards that a played number has gotten based on the fact that self is awarded with an specific award.
    fn get_derived_awards(&self, played: &PlayedNumber) -> DerivedAwardedNumber {
        DerivedAwardedNumber::new(
            self.clone(),
            played.clone(),
            DerivedAwardReason::ALL
                .into_iter()
                .filter_map(|award_reason| {
                    if award_reason.get_check_predicate()(self.number, played.number) {
                        let reason_award = self
                            .kind
                            .get_award_spec()
                            .derived_award_amount_for_reason(award_reason);
                        if reason_award.value > 0 {
                            let amount_awarded =
                                reason_award * played.bet / Amount::from_euros(200);
                            return Some(Award::new(award_reason, amount_awarded));
                        }
                    }
                    None
                })
                .collect(),
        )
    }
}

impl ExtractedAwardedNumberSliceExt for &[ExtractedAwardedNumber] {
    fn get_derived_awards(&self, played: &PlayedNumber) -> Vec<DerivedAwardedNumber> {
        self.iter()
            .map(|awarded_number| awarded_number.get_derived_awards(&played))
            .filter(|derived| !derived.awards.is_empty())
            .collect()
    }
}

/// Represents a monetary award over a specific number.
#[derive(Clone, Debug, new)]
pub struct Award {
    pub reason: DerivedAwardReason,
    pub amount: Amount,
}

/// Represents an awarded number, whose award was computed from an extracted number.
#[derive(Clone, Debug, new)]
pub struct DerivedAwardedNumber {
    pub derived_from_number: ExtractedAwardedNumber,
    pub number: PlayedNumber,
    pub awards: Vec<Award>,
}

impl DerivedAwardedNumber {
    pub fn get_total_earned(&self) -> Amount {
        self.awards
            .iter()
            .fold(Default::default(), |l, r| l + r.amount)
    }
}

/// The reasons why a derived number can be awarded.
#[derive(Clone, Copy, Debug)]
pub enum DerivedAwardReason {
    ExactMatch,
    NextToThePrizedNum,
    SameHundred,
    MatchesLastTwoDigits,
    MatchesLastDigit,
}

impl DerivedAwardReason {
    pub const ALL: [DerivedAwardReason; 5] = [
        DerivedAwardReason::ExactMatch,
        DerivedAwardReason::NextToThePrizedNum,
        DerivedAwardReason::SameHundred,
        DerivedAwardReason::MatchesLastTwoDigits,
        DerivedAwardReason::MatchesLastDigit,
    ];

    pub fn get_check_predicate(&self) -> &'static dyn Fn(LotteryNumber, LotteryNumber) -> bool {
        match *self {
            DerivedAwardReason::ExactMatch => {
                &|awarded_num, checking_num| awarded_num == checking_num
            }
            DerivedAwardReason::NextToThePrizedNum => {
                &|awarded_num, checking_num| (awarded_num as i32 - checking_num as i32).abs() == 1
            }
            DerivedAwardReason::SameHundred => &|awarded_num, checking_num| {
                awarded_num != checking_num && awarded_num / 100 == checking_num / 100
            },
            DerivedAwardReason::MatchesLastTwoDigits => &|awarded_num, checking_num| {
                awarded_num != checking_num && awarded_num % 100 == checking_num % 100
            },
            DerivedAwardReason::MatchesLastDigit => &|awarded_num, checking_num| {
                awarded_num != checking_num && awarded_num % 10 == checking_num % 10
            },
        }
    }
}
