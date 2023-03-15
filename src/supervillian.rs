pub struct Supervillian {
    pub first_name: String,
    pub last_name: String,
}
pub trait Megaweapon {
    fn shoot(&self);
}

impl Supervillian {
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    pub fn set_full_name(&mut self, name: &str) {
        let components: Vec<&str> = name.split(" ").collect::<Vec<_>>();
        self.first_name = components[0].to_string();
        self.last_name = components[1].to_string();
    }
    pub fn attack(&self, weapon: impl Megaweapon) {
        weapon.shoot();
    }
}

impl From<&str> for Supervillian {
    fn from(name: &str) -> Self {
        let components: Vec<&str> = name.split(" ").collect::<Vec<_>>();
        Supervillian {
            first_name: components[0].to_string(),
            last_name: components[1].to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    #[test]
    fn full_name_is_first_name_space_last_name() {
        // Arrange
        let badguy: Supervillian = Supervillian {
            first_name: "Lex".to_string(),
            last_name: "Luthor".to_string(),
        };
        // Act
        let full_name: String = badguy.full_name();
        // Assert
        assert_eq!(full_name, "Lex Luthor", "Unexpected fullname");
    }

    #[test]
    fn set_full_name_sets_first_and_last_name() {
        // Arrange
        let mut badguy: Supervillian = Supervillian {
            first_name: "Lex".to_string(),
            last_name: "Luthor".to_string(),
        };
        // Act
        badguy.set_full_name("Darth Vader");
        // Assert
        assert_eq!(badguy.first_name, "Darth");
        assert_eq!(badguy.last_name, "Vader");
    }

    #[test]
    fn from_str_slice_produces_s50upervillian_with_first_and_last_name() {
        // Arrange
        // Act
        let badguy: Supervillian = Supervillian::from("Darth Vader");
        // Assert
        assert_eq!(badguy.first_name, "Darth");
        assert_eq!(badguy.last_name, "Vader");
    }

    // #[test]
    // fn attack_shoots_weapon() {
    //     // Arrange
    //     let badguy: Supervillian = Supervillian {
    //         first_name: "Lex".to_string(),
    //         last_name: "Luthor".to_string(),
    //     };
    //     let weapon: WeaponDouble = WeaponDouble::new();
    //     // Act
    //     badguy.attack(weapon);
    //     // Assert
    //     // assert!(weapon.is_shot)
    // }
    struct WeaponDouble {
        pub is_shot: RefCell<bool>,
    }
    // impl WeaponDouble {
    //     fn new() -> WeaponDouble {
    //         WeaponDouble {
    //             is_shot: RefCell::new(false),
    //         }
    //     }
    // }

    impl Drop for WeaponDouble {
        fn drop(&mut self) {
            if *self.is_shot.borrow() != true {
                panic!("Failed to call shoot()");
            }
        }
    }
}
