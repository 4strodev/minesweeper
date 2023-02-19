use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct MineBox {
    mine_box_type: MineBoxType,
    visible: bool,
    marked: bool,
}

#[derive(Debug, Clone)]
pub enum MineBoxType {
    MINE,
    HINT(i8),
    EMPTY,
}

impl MineBox {
    pub fn new() -> MineBox {
        MineBox {
            mine_box_type: MineBoxType::EMPTY,
            visible: false,
            marked: false,
        }
    }

    pub fn to_string(&self) -> String {
        if !self.visible {
            return "X".to_string();
        }
        if self.marked {
            return "-".to_string();
        }

        let value: String = match self.mine_box_type {
            MineBoxType::EMPTY => "_".to_string(),
            MineBoxType::MINE => "M".to_string(),
            MineBoxType::HINT(hint) => hint.to_string(),
        };

        return value;
    }
}

impl Display for MineBox {
    // add code here
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", self.to_string());
    }
}
