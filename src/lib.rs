#[derive(Debug, Clone, PartialEq)]
pub struct Slide {
    pub title: String,
    pub notes: String,
}

pub struct PresentationDeck {
    pub slides: Vec<Slide>,
}

impl Default for PresentationDeck {
    fn default() -> Self {
        Self::new()
    }
}

impl PresentationDeck {
    pub fn new() -> Self {
        Self { slides: Vec::new() }
    }

    pub fn add_slide(&mut self, title: impl Into<String>, notes: impl Into<String>) {
        self.slides.push(Slide { title: title.into(), notes: notes.into() });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presentation_deck() {
        // Arrange
        let mut deck = PresentationDeck::new();
        // Act
        deck.add_slide("Introduction to SigmaOS", "Welcome slide");
        // Assert
        assert_eq!(deck.slides.len(), 1);
    }
}
