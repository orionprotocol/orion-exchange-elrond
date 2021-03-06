use common::require;
use elrond_codec::*;

imports!();

static INVALID_ORDER: &str = "Invalid Order Info";
static ORDER_CANCELLED_OR_EXPIRED: &str = "Order cancelled or expired";

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Order<BigUint: BigUintApi> {
    pub sender_address: Address,
    pub matcher_address: Address,
    pub base_asset: Address,
    pub quote_asset: Address,
    pub matcher_fee_asset: Address,
    pub amount: BigUint,
    pub price: BigUint,
    pub matcher_fee: BigUint,
    pub nonce: BigUint,
    pub expiration: u64,
    pub side: OrderSide,
    pub signature: H256,
}

impl<BigUint: BigUintApi> Encode for Order<BigUint> {
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

impl<BigUint: BigUintApi> Decode for Order<BigUint> {
    fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        Result::Ok(Order {
            sender_address: Address::dep_decode(input)?,
            matcher_address: Address::dep_decode(input)?,
            base_asset: Address::dep_decode(input)?,
            quote_asset: Address::dep_decode(input)?,
            matcher_fee_asset: Address::dep_decode(input)?,
            amount: BigUint::dep_decode(input)?,
            price: BigUint::dep_decode(input)?,
            matcher_fee: BigUint::dep_decode(input)?,
            nonce: BigUint::dep_decode(input)?,
            expiration: u64::dep_decode(input)?,
            side: OrderSide::dep_decode(input)?,
            signature: H256::dep_decode(input)?,
        })
    }
}

impl<BigUint: BigUintApi> Order<BigUint> {
    pub fn validate(&self) -> SCResult<()> {
        // TODO: Actually validate order signatures
        Ok(())
    }

    pub fn check_orders_info(
        buy_order: &Order<BigUint>,
        sell_order: &Order<BigUint>,
        sender: &Address,
        filled_amount: BigUint,
        filled_price: BigUint,
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
