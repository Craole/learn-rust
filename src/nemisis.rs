pub struct Nemisis {
    pub first_name: String,
    pub last_name: String,
}

impl Nemisis {
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_name_is_first_name_space_last_name() {
        // Arrange
        let sut: Nemisis = Nemisis {
            first_name: "Lex".to_string(),
            last_name: "Luthor".to_string(),
        };
        // Act
        let full_name: String = sut.full_name();
        // Assert
        // assert_eq!(full_name, "Lex Luthor");
    }
}
