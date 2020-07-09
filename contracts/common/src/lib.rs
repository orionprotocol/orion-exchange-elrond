#![no_std]
/**
 * Super hand macro, allow us to write Solidity style require!(<condition>, <error_msg>) and avoid if statements
 * 
 * It can only be used in a function that returns `Result<_, SCError>` where _ can be any type
 * 
 * example:
 * 
 * ```
 * fn only_callable_by_owner(&self) -> Result<(), SCError> {
 *     require!(self.get_caller() == self.get_owner(), "Caller must be owner");
 *     Ok(())
 * }
 * ```
 */

#[macro_export]
macro_rules! require {
	($expression:expr, $error_msg:literal) => {
		if (($expression) == false) {
			return sc_error!($error_msg)
		}
	};
}
