#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

pub struct Allergies{score: u32}
#[derive(Debug, PartialEq, Clone, Copy, enum_iterator::Sequence)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}
impl Allergies {
    #[must_use]
    pub fn new(score: u32) -> Self {
        Self{score}
    }

    #[must_use]
    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score & (1 << *allergen as u32) != 0
    }

    #[must_use]
    pub fn allergies(&self) -> Vec<Allergen> {
        enum_iterator::all::<Allergen>().filter(|a| self.is_allergic_to(a)).collect()
    }
}
