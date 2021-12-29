use std::{
    fmt::{Debug, Display, Result},
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

/// Represents a monetary amount on a specific currency
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct MonetaryAmount<C> {
    _currency: PhantomData<C>,

    /// The value held by this instance, represented as the smallest
    /// unit that the currency can be divided into.
    pub value: i64,
}

impl<C> MonetaryAmount<C> {
    pub fn new(value: i64) -> MonetaryAmount<C> {
        Self {
            _currency: PhantomData {},
            value,
        }
    }
}

impl<C> Display for MonetaryAmount<C>
where
    C: Currency,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        C::format_currency(self.value, f)
    }
}

impl<C> AddAssign<MonetaryAmount<C>> for MonetaryAmount<C>
where
    C: Currency,
{
    fn add_assign(&mut self, rhs: MonetaryAmount<C>) {
        self.value += rhs.value
    }
}

impl<C> Add<MonetaryAmount<C>> for MonetaryAmount<C>
where
    C: Currency,
{
    type Output = MonetaryAmount<C>;

    fn add(self, rhs: MonetaryAmount<C>) -> Self::Output {
        MonetaryAmount::new(self.value + rhs.value)
    }
}

impl<C> SubAssign<MonetaryAmount<C>> for MonetaryAmount<C>
where
    C: Currency,
{
    fn sub_assign(&mut self, rhs: MonetaryAmount<C>) {
        self.value -= rhs.value
    }
}

impl<C> Sub<MonetaryAmount<C>> for MonetaryAmount<C>
where
    C: Currency,
{
    type Output = MonetaryAmount<C>;

    fn sub(self, rhs: MonetaryAmount<C>) -> Self::Output {
        MonetaryAmount::new(self.value - rhs.value)
    }
}

impl<C> MulAssign<MonetaryAmount<C>> for MonetaryAmount<C>
where
    C: Currency,
{
    fn mul_assign(&mut self, rhs: MonetaryAmount<C>) {
        self.value *= rhs.value
    }
}

impl<C> Mul<MonetaryAmount<C>> for MonetaryAmount<C>
where
    C: Currency,
{
    type Output = MonetaryAmount<C>;

    fn mul(self, rhs: MonetaryAmount<C>) -> Self::Output {
        MonetaryAmount::new(self.value * rhs.value)
    }
}

impl<C> DivAssign<MonetaryAmount<C>> for MonetaryAmount<C>
where
    C: Currency,
{
    fn div_assign(&mut self, rhs: MonetaryAmount<C>) {
        self.value /= rhs.value
    }
}

impl<C> Div<MonetaryAmount<C>> for MonetaryAmount<C>
where
    C: Currency,
{
    type Output = MonetaryAmount<C>;

    fn div(self, rhs: MonetaryAmount<C>) -> Self::Output {
        MonetaryAmount::new(self.value / rhs.value)
    }
}

pub trait Currency {
    fn format_currency(cents: i64, fmt: &mut std::fmt::Formatter<'_>) -> Result;
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Euro;

impl Currency for Euro {
    fn format_currency(cents: i64, fmt: &mut std::fmt::Formatter<'_>) -> Result {
        let euros = cents / 100;
        let cents = (cents % 100).abs();

        write!(fmt, "{}.{:02}€", euros, cents)
    }
}

pub trait CurrencyEuroExt {
    fn from_floating_euros_truncated(euros: f64) -> MonetaryAmount<Euro> {
        MonetaryAmount::new((euros * 100.0).trunc() as i64)
    }

    fn from_euros(euros: i64) -> MonetaryAmount<Euro> {
        MonetaryAmount::new(euros * 100)
    }

    fn from_euros_and_cents(euros: i64, cents: u8) -> MonetaryAmount<Euro> {
        let cents = cents as i64;

        MonetaryAmount::new(if euros == 0 {
            cents
        } else if euros > 0 {
            euros * 100 + cents
        } else {
            euros * 100 - cents
        })
    }
}

impl CurrencyEuroExt for MonetaryAmount<Euro> {}
