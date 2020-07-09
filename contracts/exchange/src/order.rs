use elrond_wasm::esd_light::*;

imports!();

use crate::Bytes32;

pub enum OrderSide {
	Buy,
	Sell,
}

impl OrderSide {
    pub fn to_u8(&self) -> u8 {
        match self {
            OrderSide::Buy => 0,
            OrderSide::Sell => 1,
        }
    }

    fn from_u8(v: u8) -> Result<Self, DecodeError> {
        match v {
            0 => Ok(OrderSide::Buy),
            1 => Ok(OrderSide::Sell),
            _ => Err(DecodeError::InvalidValue),
        }
    }
}

impl Encode for OrderSide {
    fn dep_encode_to<O: Output>(&self, dest: &mut O) {
        self.to_u8().dep_encode_to(dest)
    }
}

impl Decode for OrderSide {
    fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        OrderSide::from_u8(u8::dep_decode(input)?)
    }
}

pub struct Order {
	sender_address: Address,
	matcher_address: Address,
	base_asset: Address,
	quote_asset: Address,
	matcher_fee_asset: Address,
	amount: u64,
	price: u64,
	matcher_fee: u64,
	nonce: u64,
	expiration: u64,
	side: OrderSide,
	signature: Bytes32
}

impl Encode for Order {
    fn dep_encode_to<O: Output>(&self, dest: &mut O) {
        self.sender_address.dep_encode_to(dest);
        self.matcher_address.dep_encode_to(dest);
        self.base_asset.dep_encode_to(dest);
        self.quote_asset.dep_encode_to(dest);
        self.matcher_fee_asset.dep_encode_to(dest);
        self.amount.dep_encode_to(dest);
        self.price.dep_encode_to(dest);
        self.matcher_fee.dep_encode_to(dest);
        self.nonce.dep_encode_to(dest);
        self.expiration.dep_encode_to(dest);
        self.side.dep_encode_to(dest);
        self.signature.dep_encode_to(dest);
    }
}

impl Decode for Order {
    fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        Ok(Order {
			sender_address: Address::dep_decode(input)?,
			matcher_address: Address::dep_decode(input)?,
			base_asset: Address::dep_decode(input)?,
			quote_asset: Address::dep_decode(input)?,
			matcher_fee_asset: Address::dep_decode(input)?,
			amount: u64::dep_decode(input)?,
			price: u64::dep_decode(input)?,
			matcher_fee: u64::dep_decode(input)?,
			nonce: u64::dep_decode(input)?,
			expiration: u64::dep_decode(input)?,
			side: OrderSide::dep_decode(input)?,
			signature: Bytes32::dep_decode(input)?,
        })
    }
}

impl Order {
	pub fn get_type_value_hash(&self) -> Bytes32 {
		unimplemented!()
	}

	pub fn validate(&self) -> Result<(), SCError> {
		unimplemented!()
	}
}
