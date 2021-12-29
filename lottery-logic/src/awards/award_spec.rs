use crate::*;
use paste::paste;

// pat is not supported by Rust analyzer yet, so falling back into
// pat_param for now. See
// https://github.com/rust-analyzer/rust-analyzer/issues/9055
macro_rules! define_awards {
    (@single $name:ident, $kind:ident, $max_awards:literal, $($($reason:pat_param)|* => $value:expr),*) => {
	struct $name {}
	impl ExtractedAwardKindSpec for $name {
	    fn derived_award_amount_for_reason(&self, reason: DerivedAwardReason) -> Amount {
		use DerivedAwardReason::*;

		match reason {
		    $($($reason)|* => Amount::from_euros($value),)*
		    #[allow(unreachable_patterns)]
		    _ => Amount::default()
		}
	    }

	    fn max_awarded_numbers(&self) -> u32 {
		$max_awards
	    }

	    fn kind(&self) -> ExtractedAwardKind {
		ExtractedAwardKind::$kind
	    }
	}
    };

    (@count_awards) => {
	0
    };

    (@count_awards $name:ident $(,$tname:ident)*) => {
	1 + define_awards!(@count_awards $($tname),*)
    };

    ($({$name:ident, $kind:ident, $max_awards:literal, {
	$(
	    $($reason:pat_param)|* => $value:expr
	),*
    }}),*) => {
	$(
	    define_awards!(@single $name, $kind, $max_awards, $($($reason)|* => $value),*);
	)*
	pub struct AwardSpecs;
	paste! {
	    #[allow(unused)]
	    impl AwardSpecs {
		$(
		    pub const [<$kind:upper _AWARD>]: &'static dyn ExtractedAwardKindSpec = &$name{};
		)*
		    pub const ALL_AWARDS: [&'static dyn ExtractedAwardKindSpec; define_awards!(@count_awards $($name),*)] = [
			$(
			    Self::[<$kind:upper _AWARD>]
			),*
		    ];
	    }
	}
    }
}

pub trait ExtractedAwardKindSpec {
    fn derived_award_amount_for_reason(&self, reason: DerivedAwardReason) -> Amount;
    fn max_awarded_numbers(&self) -> u32;
    fn kind(&self) -> ExtractedAwardKind;
}

define_awards!(
    {
    FirstAward, First, 1, { // Typical Spanish "El Gordo"
        ExactMatch => 4000000,
        NextToThePrizedNum => 20000,
        SameHundred | MatchesLastTwoDigits => 1000,
        MatchesLastDigit => 200
    }
    },
    {
    SecondAward, Second, 1, {
        ExactMatch => 1250000,
        NextToThePrizedNum => 12500,
        SameHundred | MatchesLastTwoDigits => 1000
    }
    },
    {
    ThirdAward, Third, 1, {
        ExactMatch => 500000,
        NextToThePrizedNum => 9600,
        SameHundred | MatchesLastTwoDigits => 1000
    }
    },
    {
    FourthAward, Fourth, 2, {
        ExactMatch => 200000,
        SameHundred => 1000
    }
    },
    {
    FifthAward, Fifth, 8, {
        ExactMatch => 60000
    }
    },
    {
    LittleAward, Little, 1000, { // Typical Spanish "Pedrea"
        ExactMatch => 1000
    }
    }
);
