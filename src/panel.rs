use crate::mine::MineBox;

#[derive(Debug)]
pub struct MineBoxPanel {
    box_array: Vec<Vec<MineBox>>,
}

impl MineBoxPanel {
    pub fn new(size: usize) -> MineBoxPanel {
        return MineBoxPanel {
            box_array: vec![vec![MineBox::new(); size]; size],
        };
    }

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
}

impl std::fmt::Display for MineBoxPanel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
