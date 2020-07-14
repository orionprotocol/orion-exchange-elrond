use elrond_codec::*;

pub struct Trade {
    filled_price: u64,
    filled_amount: u64,
    fee_paid: u64,
    timestamp: u64,
}

// this serialization method taken from
// https://github.com/ElrondNetwork/sc-examples-rs/blob/master/features/src/ser_ex1.rs
// Surely this can be implemented as a derive macro

impl Encode for Trade {
    fn dep_encode_to<O: Output>(&self, dest: &mut O) {
        self.filled_price.dep_encode_to(dest);
        self.filled_amount.dep_encode_to(dest);
        self.fee_paid.dep_encode_to(dest);
        self.timestamp.dep_encode_to(dest);
    }
}

impl Decode for Trade {
    fn dep_decode<I: Input>(input: &mut I) -> Result<Self, DecodeError> {
        Ok(Trade {
            filled_price: u64::dep_decode(input)?,
            filled_amount: u64::dep_decode(input)?,
            fee_paid: u64::dep_decode(input)?,
            timestamp: u64::dep_decode(input)?,
        })
    }
}
