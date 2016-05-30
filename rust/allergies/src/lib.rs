
pub struct Allergies {
    value: u32,
}

impl Allergies {
    pub fn new(value: u32) -> Allergies {
        Allergies {
            // Ignore parts of value above maximum allergies test value.
            value: value % Allergies::max_value(),
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies().contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut remainder_to_test = self.value; // TODO rename
        let mut allergies = vec![];

        // Enumerate allergens in reverse order as need to test larger values first.
        for (i, allergen) in Allergen::ordered().iter().enumerate().rev().collect::<Vec<_>>() {
            let allergen_test_value = Allergen::value_of(i);
            if remainder_to_test >= allergen_test_value {
                remainder_to_test -= allergen_test_value;
                allergies.push((*allergen).clone());
            }
        }

        // Tested allergies in reverse order so reverse back in order.
        allergies.reverse();
        allergies
    }

    // The maximum value of an allergies test is that when all allergies are present.
    fn max_value() -> u32 {
        let allergens = Allergen::ordered();
        Allergen::value_of(allergens.len())
    }
}

#[derive(Clone, Debug, PartialEq)]
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

impl Allergen {
    fn ordered() -> &'static [Allergen] {
        const ORDERED_ALLERGENS: &'static [Allergen] = &[
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ];
        ORDERED_ALLERGENS
    }

    fn value_of(allergen_position: usize) -> u32 {
        (2 as u32).pow(allergen_position as u32)
    }
}
