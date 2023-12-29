use serde::{ser::{SerializeStruct, Serializer}, Deserialize, Serialize, Deserializer};

use super::contants::EMPTY_PIECE;

#[derive(Debug, Clone, PartialEq)]
pub struct PieceMove {
    from_position: i8,
    capture: bool,
    en_passant: bool,
    promotion: bool,
    move_worth: i32,
    piece_value: i8,
    promotion_type: i8,
    to_position: i8,
}

impl Serialize for PieceMove {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("PieceMove", 8)?;
        state.serialize_field("capture", &self.capture)?;
        state.serialize_field("enPassant", &self.en_passant)?;
        state.serialize_field("fromPosition", &self.from_position)?;
        state.serialize_field("moveWorth", &self.move_worth)?;
        state.serialize_field("pieceValue", &self.piece_value)?;
        state.serialize_field("promotion", &self.promotion)?;
        state.serialize_field("promotionType", &self.promotion_type)?;
        state.serialize_field("toPosition", &self.to_position)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for PieceMove {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PieceMoveVisitor;

        impl<'de> serde::de::Visitor<'de> for PieceMoveVisitor {
            type Value = PieceMove;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct PieceMove")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut from_position: Option<i8> = None;
                let mut capture: Option<bool> = None;
                let mut en_passant: Option<bool> = None;
                let mut promotion: Option<bool> = None;
                let mut move_worth: Option<i32> = None;
                let mut piece_value: Option<i8> = None;
                let mut promotion_type: Option<i8> = None;
                let mut to_position: Option<i8> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "fromPosition" => from_position = Some(map.next_value()?),
                        "capture" => capture = Some(map.next_value()?),
                        "enPassant" => en_passant = Some(map.next_value()?),
                        "promotion" => promotion = Some(map.next_value()?),
                        "moveWorth" => move_worth = Some(map.next_value()?),
                        "pieceValue" => piece_value = Some(map.next_value()?),
                        "promotionType" => promotion_type = Some(map.next_value()?),
                        "toPosition" => to_position = Some(map.next_value()?),
                        _ => {
                            return Err(serde::de::Error::unknown_field(
                                key,
                                &["fromPosition", "capture", "enPassant", "promotion", "moveWorth", "pieceValue", "promotionType", "toPosition"],
                            ));
                        }
                    }
                }

                let from_position = from_position.ok_or_else(|| serde::de::Error::missing_field("from_position"))?;
                let capture = capture.ok_or_else(|| serde::de::Error::missing_field("capture"))?;
                let en_passant = en_passant.ok_or_else(|| serde::de::Error::missing_field("en_passant"))?;
                let promotion = promotion.ok_or_else(|| serde::de::Error::missing_field("promotion"))?;
                let move_worth = move_worth.ok_or_else(|| serde::de::Error::missing_field("move_worth"))?;
                let piece_value = piece_value.ok_or_else(|| serde::de::Error::missing_field("piece_value"))?;
                let promotion_type = promotion_type.ok_or_else(|| serde::de::Error::missing_field("promotion_type"))?;
                let to_position = to_position.ok_or_else(|| serde::de::Error::missing_field("to_position"))?;

                Ok(PieceMove {
                    from_position,
                    capture,
                    en_passant,
                    promotion,
                    move_worth,
                    piece_value,
                    promotion_type,
                    to_position,
                })
            }
        }

        deserializer.deserialize_map(PieceMoveVisitor)
    }
}

impl PieceMove {
    pub fn new(from: i8, piece_value: i8, to: i8) -> Self {
        Self {
            from_position: from,
            capture: false,
            en_passant: false,
            promotion: false,
            move_worth: 0,
            piece_value,
            promotion_type: EMPTY_PIECE,
            to_position: to,
        }
    }

    pub fn clone(&self) -> PieceMove {
        PieceMove {
            from_position: self.from_position,
            capture: self.capture,
            en_passant: self.en_passant,
            promotion: self.promotion,
            move_worth: self.move_worth,
            piece_value: self.piece_value,
            promotion_type: self.promotion_type,
            to_position: self.to_position,
        }
    }

    pub fn eq(&self, piece_move: &Self) -> bool {
        (self.from_position == piece_move.from_position)
            && (self.capture == piece_move.capture)
            && (self.en_passant == piece_move.en_passant)
            && (self.promotion == piece_move.promotion)
            && (self.move_worth == piece_move.move_worth)
            && (self.piece_value == piece_move.piece_value)
            && (self.promotion_type == piece_move.promotion_type)
            && (self.to_position == piece_move.to_position)
    }

    pub fn get_from_position(&self) -> i8 {
        self.from_position
    }

    pub fn get_move_worth(&self) -> i32 {
        self.move_worth
    }

    pub fn get_piece_value(&self) -> i8 {
        self.piece_value
    }

    pub fn get_promotion_value(&self) -> i8 {
        self.promotion_type
    }

    pub fn get_to_position(&self) -> i8 {
        self.to_position
    }

    pub fn is_capture(&self) -> bool {
        self.capture
    }

    pub fn is_en_passant(&self) -> bool {
        self.en_passant
    }

    pub fn is_promotion(&self) -> bool {
        self.promotion
    }

    pub fn set_from_position(&mut self, from_position: i8) {
        self.from_position = from_position;
    }

    pub fn set_is_capture(&mut self, is_capture: bool) {
        self.capture = is_capture;
    }

    pub fn set_is_en_passant(&mut self, is_en_passant: bool) {
        self.en_passant = is_en_passant;
    }

    pub fn set_is_promotion(&mut self, is_promotion: bool) {
        self.promotion = is_promotion;
    }

    pub fn set_move_worth(&mut self, move_worth: i32) {
        self.move_worth = move_worth;
    }

    pub fn set_piece_value(&mut self, piece_value: i8) {
        self.piece_value = piece_value;
    }

    pub fn set_promotion_value(&mut self, promotion_type: i8) {
        self.promotion_type = promotion_type;
    }

    pub fn set_to_position(&mut self, to_position: i8) {
        self.to_position = to_position;
    }

    pub fn sum_to_move_worth(&mut self, value: i32) {
        self.move_worth += value
    }
}
