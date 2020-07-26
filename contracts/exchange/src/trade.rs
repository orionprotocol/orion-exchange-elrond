use elrond_codec::*;
use elrond_wasm::BigUintApi;

/**
 * Because the BigUint type isn't actually provided at compile time this is the only way we can
 * use it in structs. Using type params allows the actual implementation to be provided later.
 */
pub struct Trade<BigUint: BigUintApi> {
    pub filled_price: BigUint,
    pub filled_amount: BigUint,
    pub fee_paid: BigUint,
    pub timestamp: u64,
}

impl<BigUint: elrond_wasm::BigUintApi> Trade<BigUint> {
    pub fn new(
        filled_price: BigUint,
        filled_amount: BigUint,
        fee_paid: BigUint,
        timestamp: u64,
    ) -> Self {
        Self {
            filled_price,
            filled_amount,
            fee_paid,
            timestamp,
        }
    }
}

// this serialization method taken from
// https://github.com/ElrondNetwork/sc-examples-rs/blob/master/features/src/ser_ex1.rs
// Surely this can be implemented as a derive macro

impl<BigUint: BigUintApi> Encode for Trade<BigUint> {
    fn dep_encode_to<O: Output>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.filled_price.dep_encode_to(dest)?;
        self.filled_amount.dep_encode_to(dest)?;
        self.fee_paid.dep_encode_to(dest)?;
        self.timestamp.dep_encode_to(dest)
    }
}

impl<BigUint: BigUintApi> Decode for Trade<BigUint> {
    fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        Ok(Trade {
            filled_price: BigUint::dep_decode(input)?,
            filled_amount: BigUint::dep_decode(input)?,
            fee_paid: BigUint::dep_decode(input)?,
            timestamp: u64::dep_decode(input)?,
        })
    }
}
