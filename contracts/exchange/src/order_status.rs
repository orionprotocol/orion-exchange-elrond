use elrond_wasm::esd_light::*;

pub enum OrderStatus {
	New,
	PartiallyFilled,
	Filled,
	PartiallyCancelled,
	Cancelled
}

impl OrderStatus {
    pub fn to_u8(&self) -> u8 {
        match self {
            OrderStatus::New => 0,
            OrderStatus::PartiallyFilled => 1,
            OrderStatus::Filled => 2,
            OrderStatus::PartiallyCancelled => 3,
            OrderStatus::Cancelled => 4
        }
    }

    fn from_u8(v: u8) -> Result<Self, DecodeError> {
        match v {
            0 => Ok(OrderStatus::New),
            1 => Ok(OrderStatus::PartiallyFilled),
            2 => Ok(OrderStatus::Filled),
            3 => Ok(OrderStatus::PartiallyCancelled),
            4 => Ok(OrderStatus::Cancelled),
            _ => Err(DecodeError::InvalidValue),
        }
    }
}

impl Encode for OrderStatus {
	fn dep_encode_to<O: Output>(&self, dest: &mut O) {
        self.to_u8().dep_encode_to(dest)
	}
}

impl Decode for OrderStatus {
	fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        OrderStatus::from_u8(u8::dep_decode(input)?)
    }
}
