use super::errors::OutOfBoundariesError;
use crate::features::mine::domain::models::MineBox;
use std::fmt::Display;
use std::result::Result;

#[derive(Debug)]
pub struct MineBoxPanel {
    box_array: Vec<Vec<MineBox>>,
    size: usize,
}

#[derive(Debug)]
pub struct MineBoxPanelPosition {
    longitude: u8,
    latitude: u8,
}

impl MineBoxPanel {
    pub fn new(size: usize) -> MineBoxPanel {
        return MineBoxPanel {
            box_array: vec![vec![MineBox::new(); size]; size],
            size,
        };
    }

    /// Return panel as a string representation
    pub fn to_string(&self) -> String {
        let mut panel_string: String = String::new();

        // Appending characters to panel_string
        for row in self.box_array.iter() {
            for element in row.iter() {
                // Shorrounding elements with space
                panel_string.push(' ');
                panel_string.push_str(&element.to_string());
                panel_string.push(' ');
            }

            // Every time a row finish add a newline
            panel_string.push('\n');
        }

        // returning panel_string
        return panel_string;
    }

    pub fn mark_mine(&self, position: MineBoxPanelPosition) -> Result<(), OutOfBoundariesError> {
        if self.size < position.longitude.into() || self.size < position.latitude.into() {
            return Result::Err(OutOfBoundariesError::new());
        }

        let row = match self.box_array.get::<usize>(position.longitude.into()) {
            Some(value) => value,
            None => panic!("Row in position {} not defined!", position.longitude),
        };

        let mine_box = match row.get::<usize>(position.latitude.into()) {
            Some(value) => value,
            None => panic!("Mine box in position {} not defined!", position.latitude),
        };

        return Result::Ok(());
    }
}

impl Display for MineBoxPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
