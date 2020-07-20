(cd contracts/token; erdpy contract build)
(cd contracts/exchange; erdpy contract build)

(cd contracts/token; erdpy contract test)
(cd contracts/exchange; erdpy contract test)
