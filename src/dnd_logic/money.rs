use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;


#[derive(Debug, EnumIter, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Currency {
    Copper,
    Silver,
    Electrum,
    Gold,
    Platinum,
}

impl Currency {
    pub fn to_string(&self) -> String {
        match self {
            Currency::Copper => String::from("Copper"),
            Currency::Silver => String::from("Silver"),
            Currency::Electrum => String::from("Electrum"),
            Currency::Gold => String::from("Gold"),
            Currency::Platinum => String::from("Platinum"),
        }
    }

    pub fn to_multiplier(&self) -> i32 {
        match self {
            Currency::Copper => 1,
            Currency::Silver => 10,
            Currency::Electrum => 50,
            Currency::Gold => 100,
            Currency::Platinum => 1000,
        }
    }

    pub fn convert_to(&self, to: &Currency, amount: i32) -> Option<i32> {
        let multiplier = self.to_multiplier();
        let to_multiplier = to.to_multiplier();

        let copper_amount = amount * multiplier;
        let converted_amount = copper_amount / to_multiplier;

        if converted_amount == 0 {
            return None;
        }

        Some(converted_amount)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn copper_to_others() {
        let copper = super::Currency::Copper;
        let silver = super::Currency::Silver;
        let electrum = super::Currency::Electrum;
        let gold = super::Currency::Gold;
        let platinum = super::Currency::Platinum;

        assert_eq!(copper.convert_to(&silver, 100), Some(10));
        assert_eq!(copper.convert_to(&electrum, 100), Some(2));
        assert_eq!(copper.convert_to(&gold, 100), Some(1));
        assert_eq!(copper.convert_to(&platinum, 100), None);
    }

    #[test]
    fn silver_to_others() {
        let copper = super::Currency::Copper;
        let silver = super::Currency::Silver;
        let electrum = super::Currency::Electrum;
        let gold = super::Currency::Gold;
        let platinum = super::Currency::Platinum;

        assert_eq!(silver.convert_to(&copper, 100), Some(1000));
        assert_eq!(silver.convert_to(&electrum, 100), Some(20));
        assert_eq!(silver.convert_to(&gold, 100), Some(10));
        assert_eq!(silver.convert_to(&platinum, 100), Some(1));
    }

    #[test]
    fn electrum_to_others() {
        let copper = super::Currency::Copper;
        let silver = super::Currency::Silver;
        let electrum = super::Currency::Electrum;
        let gold = super::Currency::Gold;
        let platinum = super::Currency::Platinum;

        assert_eq!(electrum.convert_to(&copper, 100), Some(5000));
        assert_eq!(electrum.convert_to(&silver, 100), Some(500));
        assert_eq!(electrum.convert_to(&gold, 100), Some(50));
        assert_eq!(electrum.convert_to(&platinum, 100), Some(5));
    }

    #[test]
    fn gold_to_others() {
        let copper = super::Currency::Copper;
        let silver = super::Currency::Silver;
        let electrum = super::Currency::Electrum;
        let gold = super::Currency::Gold;
        let platinum = super::Currency::Platinum;

        assert_eq!(gold.convert_to(&copper, 100), Some(10000));
        assert_eq!(gold.convert_to(&silver, 100), Some(1000));
        assert_eq!(gold.convert_to(&electrum, 100), Some(200));
        assert_eq!(gold.convert_to(&platinum, 100), Some(10));
    }

    #[test]
    fn platinum_to_others() {
        let copper = super::Currency::Copper;
        let silver = super::Currency::Silver;
        let electrum = super::Currency::Electrum;
        let gold = super::Currency::Gold;
        let platinum = super::Currency::Platinum;

        assert_eq!(platinum.convert_to(&copper, 100), Some(100000));
        assert_eq!(platinum.convert_to(&silver, 100), Some(10000));
        assert_eq!(platinum.convert_to(&electrum, 100), Some(2000));
        assert_eq!(platinum.convert_to(&gold, 100), Some(1000));
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    pub copper: i32,
    pub silver: i32,
    pub electrum: i32,
    pub gold: i32,
    pub platinum: i32,
}

impl Default for Money {
    fn default() -> Self {
        Self {
            copper: 0,
            silver: 0,
            electrum: 0,
            gold: 0,
            platinum: 0,
        }
    }
}

impl Money {
    pub fn new(copper: i32, silver: i32, electrum: i32, gold: i32, platinum: i32) -> Self {
        Self {
            copper,
            silver,
            electrum,
            gold,
            platinum,
        }
    }

    pub fn get_currency_mut(&mut self, currency: &Currency) -> &mut i32 {
        match currency {
            Currency::Copper => &mut self.copper,
            Currency::Silver => &mut self.silver,
            Currency::Electrum => &mut self.electrum,
            Currency::Gold => &mut self.gold,
            Currency::Platinum => &mut self.platinum,
        }
    }

    pub fn add_money(&mut self, currency: &Currency, amount: i32) {
        *self.get_currency_mut(currency) += amount;
    }

    pub fn subtract_money(&mut self, currency: &Currency, amount: i32) {
        let mut cur = self.get_currency_mut(currency);
        if *cur < amount {
            *cur = 0;
        } else {
            *cur -= amount;
        }
    }

    pub fn convert_money(&mut self, amount: i32, from: &Currency, to: &Currency) -> Option<i32> {
        let new_amount = from.convert_to(to, amount);
        
        new_amount
    }
}