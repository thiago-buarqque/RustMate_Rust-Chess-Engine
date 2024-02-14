use serde::{
    ser::{SerializeStruct, Serializer},
    Deserialize, Deserializer, Serialize,
};

use super::{board_utils::get_move_notation, contants::EMPTY_PIECE};

#[derive(Debug, Clone, PartialEq)]
pub struct PieceMove {
    capture: bool,
    castle: bool,
    from_position: i8,
    en_passant: bool,
    notation: String,
    move_worth: i32,
    piece_value: u8,
    puts_king_in_check: bool,
    promotion: bool,
    promotion_type: u8,
    to_position: i8,
}

impl Serialize for PieceMove {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 3 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("PieceMove", 10)?;
        state.serialize_field("capture", &self.capture)?;
        state.serialize_field("castle", &self.castle)?;
        state.serialize_field("enPassant", &self.en_passant)?;
        state.serialize_field("fromPosition", &self.from_position)?;
        state.serialize_field("notation", &self.get_notation())?;
        state.serialize_field("putsKingInCheck", &self.puts_king_in_check)?;
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
                let mut castle: Option<bool> = None;
                let mut from_position: Option<i8> = None;
                let mut capture: Option<bool> = None;
                let mut en_passant: Option<bool> = None;
                let mut notation: Option<String> = None;
                let mut move_worth: Option<i32> = None;
                let mut piece_value: Option<u8> = None;                
                let mut puts_king_in_check: Option<bool> = None;
                let mut promotion: Option<bool> = None;
                let mut promotion_type: Option<u8> = None;
                let mut to_position: Option<i8> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "castle" => castle = Some(map.next_value()?),
                        "fromPosition" => from_position = Some(map.next_value()?),
                        "capture" => capture = Some(map.next_value()?),
                        "enPassant" => en_passant = Some(map.next_value()?),
                        "promotion" => promotion = Some(map.next_value()?),
                        "moveWorth" => move_worth = Some(map.next_value()?),
                        "pieceValue" => piece_value = Some(map.next_value()?),
                        "putsKingInCheck" => puts_king_in_check = Some(map.next_value()?),
                        "promotionType" => promotion_type = Some(map.next_value()?),
                        "toPosition" => to_position = Some(map.next_value()?),
                        "notation" => notation = Some(map.next_value()?),
                        _ => {}
                    }
                }

                let castle = castle.ok_or_else(|| serde::de::Error::missing_field("castle"))?;

                let from_position = from_position
                    .ok_or_else(|| serde::de::Error::missing_field("from_position"))?;
                let capture = capture.ok_or_else(|| serde::de::Error::missing_field("capture"))?;

                let en_passant =
                    en_passant.ok_or_else(|| serde::de::Error::missing_field("en_passant"))?;

                let notation =
                    notation.ok_or_else(|| serde::de::Error::missing_field("notation"))?;

                let promotion =
                    promotion.ok_or_else(|| serde::de::Error::missing_field("promotion"))?;

                let puts_king_in_check =
                    puts_king_in_check.ok_or_else(|| serde::de::Error::missing_field("puts_king_in_check"))?;

                let move_worth =
                    move_worth.ok_or_else(|| serde::de::Error::missing_field("move_worth"))?;

                let piece_value =
                    piece_value.ok_or_else(|| serde::de::Error::missing_field("piece_value"))?;

                let promotion_type = promotion_type
                    .ok_or_else(|| serde::de::Error::missing_field("promotion_type"))?;

                let to_position =
                    to_position.ok_or_else(|| serde::de::Error::missing_field("to_position"))?;

                Ok(PieceMove {
                    castle,
                    from_position,
                    capture,
                    en_passant,
                    notation,
                    promotion,
                    puts_king_in_check,
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
    pub fn new(from_position: i8, piece_value: u8, to_position: i8) -> Self {
        Self {
            capture: false,
            castle: false,
            en_passant: false,
            from_position,
            move_worth: 0,
            notation: String::new(),
            puts_king_in_check: false,
            piece_value,
            promotion: false,
            promotion_type: EMPTY_PIECE,
            to_position,
        }
    }

    pub fn eq(&self, other: &Self) -> bool {
        self.from_position == other.from_position &&
        self.capture == other.capture &&
        self.castle == other.castle &&
        self.en_passant == other.en_passant &&
        self.promotion == other.promotion &&
        self.move_worth == other.move_worth &&
        self.puts_king_in_check == other.puts_king_in_check &&
        self.piece_value == other.piece_value &&
        self.promotion_type == other.promotion_type &&
        self.to_position == other.to_position
    }

    pub fn get_from_position(&self) -> i8 {
        self.from_position
    }

    pub fn get_move_worth(&self) -> i32 {
        self.move_worth
    }

    pub fn get_piece_value(&self) -> u8 {
        self.piece_value
    }

    pub fn get_promotion_value(&self) -> u8 {
        self.promotion_type
    }

    pub fn get_to_position(&self) -> i8 {
        self.to_position
    }

    pub fn puts_king_in_check(&self) -> bool {
        self.puts_king_in_check
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

    pub fn set_puts_king_in_check(&mut self, puts_king_in_check: bool) {
        self.puts_king_in_check = puts_king_in_check;
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

    pub fn set_promotion_value(&mut self, promotion_type: u8) {
        self.promotion_type = promotion_type;
    }

    pub fn sum_to_move_worth(&mut self, value: i32) {
        self.move_worth += value
    }

    pub fn get_notation(&self) -> String {
        get_move_notation(self)
    }
}
