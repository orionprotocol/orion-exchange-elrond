use elrond_codec::*;
use elrond_wasm::BigUintApi;

/**
 * Because the BigUint type isn't actually provided at compile time this is the only way we can
 * use it in structs. Using type params allows the actual implementation to be provided later.
 */
pub struct Trade<T: BigUintApi> {
    pub filled_price: T,
    pub filled_amount: T,
    pub fee_paid: T,
    pub timestamp: u64,
}

impl<T: elrond_wasm::BigUintApi> Trade<T> {
    pub fn new(filled_price: T, filled_amount: T, fee_paid: T, timestamp: u64) -> Self {
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

impl<T: BigUintApi> Encode for Trade<T> {
    fn dep_encode_to<O: Output>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.filled_price.dep_encode_to(dest)?;
        self.filled_amount.dep_encode_to(dest)?;
        self.fee_paid.dep_encode_to(dest)?;
        self.timestamp.dep_encode_to(dest)
    }
}

impl<T: BigUintApi> Decode for Trade<T> {
    fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        Ok(Trade {
            filled_price: T::dep_decode(input)?,
            filled_amount: T::dep_decode(input)?,
            fee_paid: T::dep_decode(input)?,
            timestamp: u64::dep_decode(input)?,
        })
    }
}
