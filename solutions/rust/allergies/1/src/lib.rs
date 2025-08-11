#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]

pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Allergen {
    Eggs = 0,
    Peanuts = 1,
    Shellfish = 2,
    Strawberries = 3,
    Tomatoes = 4,
    Chocolate = 5,
    Pollen = 6,
    Cats = 7,
}

impl std::convert::TryFrom<u8> for Allergen {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Allergen::Eggs),
            1 => Ok(Allergen::Peanuts),
            2 => Ok(Allergen::Shellfish),
            3 => Ok(Allergen::Strawberries),
            4 => Ok(Allergen::Tomatoes),
            5 => Ok(Allergen::Chocolate),
            6 => Ok(Allergen::Pollen),
            7 => Ok(Allergen::Cats),
            _ => Err("Invalid allergen index"),
        }
    }
}

impl Allergies {
    #[must_use]
    pub fn new(score: u32) -> Self {
        Allergies { score: score & (0b1111_1111) }
        //todo!("Given the '{score}' score, construct a new Allergies struct.");
    }

    #[must_use]
    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        (self.score >> *allergen as u32) & (1) != 0
        //todo!("Determine if the patient is allergic to the '{allergen:?}' allergen.");
    }

    #[must_use]
    pub fn allergies(&self) -> Vec<Allergen> {
        (0..8)
            .filter_map(|i| {
                if (self.score & (1 << i)) != 0 {
                    Allergen::try_from(i).ok()
                } else {
                    None
                }
            })
            .collect()

        // todo!(
        //     "Return the list of allergens contained within the score with which the Allergies struct was made."
        // );
    }
}
