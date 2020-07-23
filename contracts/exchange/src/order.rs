use common::require;
use elrond_codec::*;

imports!();

static INVALID_ORDER: &str = "Invalid Order Info";
static ORDER_CANCELLED_OR_EXPIRED: &str = "Order cancelled or expired";

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
            0 => Result::Ok(OrderSide::Buy),
            1 => Result::Ok(OrderSide::Sell),
            _ => Result::Err(DecodeError::InvalidValue),
        }
    }
}

impl Encode for OrderSide {
    fn dep_encode_to<O: Output>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.to_u8().dep_encode_to(dest)
    }
}

impl Decode for OrderSide {
    fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        OrderSide::from_u8(u8::dep_decode(input)?)
    }
}

pub struct Order {
    pub sender_address: Address,
    pub matcher_address: Address,
    pub base_asset: Address,
    pub quote_asset: Address,
    pub matcher_fee_asset: Address,
    pub amount: u64,
    pub price: u64,
    pub matcher_fee: u64,
    pub nonce: u64,
    pub expiration: u64,
    pub side: OrderSide,
    pub signature: Bytes32,
}

impl Encode for Order {
    fn dep_encode_to<O: Output>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.sender_address.dep_encode_to(dest)?;
        self.matcher_address.dep_encode_to(dest)?;
        self.base_asset.dep_encode_to(dest)?;
        self.quote_asset.dep_encode_to(dest)?;
        self.matcher_fee_asset.dep_encode_to(dest)?;
        self.amount.dep_encode_to(dest)?;
        self.price.dep_encode_to(dest)?;
        self.matcher_fee.dep_encode_to(dest)?;
        self.nonce.dep_encode_to(dest)?;
        self.expiration.dep_encode_to(dest)?;
        self.side.dep_encode_to(dest)?;
        self.signature.dep_encode_to(dest)
    }
}

impl Decode for Order {
    fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        Result::Ok(Order {
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
    pub fn validate(&self) -> SCResult<()> {
        // TODO: Actually validate order signatures
        Ok(())
    }

    pub fn check_orders_info(
        buy_order: &Order,
        sell_order: &Order,
        sender: &Address,
        filled_amount: u64,
        filled_price: u64,
        current_time: u64,
    ) -> SCResult<()> {
        sc_try!(buy_order.validate());
        sc_try!(sell_order.validate());

        require!(&buy_order.matcher_address == sender, INVALID_ORDER);
        require!(&sell_order.matcher_address == sender, INVALID_ORDER);

        require!(buy_order.base_asset == sell_order.base_asset, INVALID_ORDER);
        require!(
            buy_order.quote_asset == sell_order.quote_asset,
            INVALID_ORDER
        );

        require!(filled_amount <= buy_order.amount, INVALID_ORDER);
        require!(filled_amount <= sell_order.amount, INVALID_ORDER);

        require!(filled_price <= buy_order.price, INVALID_ORDER);
        require!(filled_price >= sell_order.price, INVALID_ORDER);

        require!(
            buy_order.expiration >= current_time,
            ORDER_CANCELLED_OR_EXPIRED
        );
        require!(
            sell_order.expiration >= current_time,
            ORDER_CANCELLED_OR_EXPIRED
        );

        Ok(())
    }
}
