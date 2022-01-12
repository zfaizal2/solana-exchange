use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub enum ExchangeBoothInstruction {
    InititializeExchangeBooth {
        // TODO
        // check if token accounts are valid
        // assign tokenPairBooth owner to creator
        // create PDA for tokenPairBooth
     },
    Deposit {
        // TODO
        // validate token inputs
        // call token program 
        // withdraw amount from token account from users
        // add amount to vault account
    },
    Withdraw {
        // TODO
        // validate token inputs
        // withdraw amount from
    },
    Exchange {
        // TODO
    },
    CloseExchangeBooth {
        // TODO
    },
}
