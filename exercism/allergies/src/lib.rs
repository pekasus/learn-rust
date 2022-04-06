pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Allergen {
    Eggs, // 0
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergen {
    fn position(&self) -> u8 {
        *self as u8
    }
}

#[cfg(test)]
mod test {
    use super::Allergen;
    #[test]
    fn enum_position() {
        assert_eq!(Allergen::Eggs.position(), 0);
    }
}

const ALLERGENS: [Allergen; 8] = [
    Allergen::Eggs,
    Allergen::Peanuts,
    Allergen::Shellfish,
    Allergen::Strawberries,
    Allergen::Tomatoes,
    Allergen::Chocolate,
    Allergen::Pollen,
    Allergen::Cats,
];

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score & 2u32.pow(allergen.position() as u32) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        ALLERGENS
            .iter()
            .fold(Vec::<Allergen>::new(), |mut vec, &allergen| {
                if self.is_allergic_to(&allergen) {
                    vec.push(allergen);
                }
                vec
            })
    }
}
