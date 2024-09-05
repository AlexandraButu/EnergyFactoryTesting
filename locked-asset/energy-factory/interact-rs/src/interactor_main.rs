#![allow(non_snake_case)]

mod proxy;

use common_structs::EpochAmountPair;
use common_structs::UnlockEpochAmountPairs;
use multiversx_sc_snippets::imports::*;
use multiversx_sc_snippets::sdk;
use multiversx_sc_snippets::sdk::data::address;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};


const GATEWAY: &str = sdk::blockchain::DEVNET_GATEWAY;
const STATE_FILE: &str = "state.toml";

const DAN_ADDRESS: &str = "erd1kyaqzaprcdnv4luvanah0gfxzzsnpaygsy6pytrexll2urtd05ts9vegu7";
const FRANK_ADDRESS: &str = "erd1kdl46yctawygtwg2k462307dmz2v55c605737dp3zkxh04sct7asqylhyv";
const SC_ADDRESS_SIMPLE_LOCK: &str = "erd1qqqqqqqqqqqqqpgqlyyexsvg2kc2eqk9ellagd78ltm6lce9eccsytn0a9";


const OLD_LOCKED_ASSET_FACTORY_ADDRESS: &str =
    "erd1qqqqqqqqqqqqqpgqxk5hlvgkwzen6q0kxgaljtu6p524swwcv5ysa9hnht";
const BASE_ASSET_TOKEN_ID: &str = "TST-af9b21";
const LEGACY_TOKEN_ID: &str = "TSTT-d96162";


const ANOTHER_TOKEN_ID: &str = "SMTH-6fb124";
const ALICE_ADDRESS: &str = "erd1qyu5wthldzr8wx5c9ucg8kjagg0jfs53s8nr3zpz3hypefsdd8ssycr6th";
const BOB_ADDRESS: &str = "erd1spyavw0956vq68xj8y4tenjpq2wd5a9p2c6j8gsz7ztyrnpxrruqzu66jx";

const TOKEN_ISSUED: &str =  "CCC-869ee6";
const FEE: u128 = 50000000000000000; // 0.05 EGLD;


const TOKEN_NOU_CREAT: &str = "TEST-248836";
#[tokio::main]
async fn main() {
    env_logger::init();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect("at least one argument required");
    let mut interact = ContractInteract::new().await;
    match cmd.as_str() {
        // "deploy" => interact.deploy().await,
        // "lockTokens" => interact.lock_tokens_endpoint().await,
        // "unlockTokens" => interact.unlock_tokens_endpoint().await,
        // "extendLockPeriod" => interact.extend_lock_period().await,
        // "issueLockedToken" => interact.issue_locked_token().await,
        // "getLockedTokenId" => interact.locked_token().await,
        // "getBaseAssetTokenId" => interact.base_asset_token_id().await,
        // "getLegacyLockedTokenId" => interact.legacy_locked_token_id().await,
        // "getEnergyEntryForUser" => interact.get_updated_energy_entry_for_user().await,
        // "getEnergyAmountForUser" => interact.get_energy_amount_for_user().await,
        // "addLockOptions" => interact.add_lock_options().await,
        // "getLockOptions" => interact.get_lock_options_view().await,
        // "unlockEarly" => interact.unlock_early().await,
        // "reduceLockPeriod" => interact.reduce_lock_period().await,
        // "getPenaltyAmount" => interact.calculate_penalty_amount().await,
        // "setTokenUnstakeAddress" => interact.set_token_unstake_address().await,
        // "revertUnstake" => interact.revert_unstake().await,
        // "getTokenUnstakeScAddress" => interact.token_unstake_sc_address().await,
        // "setEnergyForOldTokens" => interact.set_energy_for_old_tokens().await,
        // "updateEnergyAfterOldTokenUnlock" => interact.update_energy_after_old_token_unlock().await,
        // "migrateOldTokens" => interact.migrate_old_tokens().await,
        // "pause" => interact.pause_endpoint().await,
        // "unpause" => interact.unpause_endpoint().await,
        // "isPaused" => interact.paused_status().await,
        // "setTransferRoleLockedToken" => interact.set_transfer_role().await,
        // "setBurnRoleLockedToken" => interact.set_burn_role().await,
        // "mergeTokens" => interact.merge_tokens_endpoint().await,
        // "lockVirtual" => interact.lock_virtual().await,
        // "addSCAddressToWhitelist" => interact.add_sc_address_to_whitelist().await,
        // "removeSCAddressFromWhitelist" => interact.remove_sc_address_from_whitelist().await,
        // "isSCAddressWhitelisted" => interact.is_sc_address_whitelisted().await,
        // "addToTokenTransferWhitelist" => interact.add_to_token_transfer_whitelist().await,
        // "removeFromTokenTransferWhitelist" => interact.remove_from_token_transfer_whitelist().await,
        // "setUserEnergyAfterLockedTokenTransfer" => interact.set_user_energy_after_locked_token_transfer().await,
        _ => panic!("unknown command: {}", &cmd),
    }
}


#[derive(Debug, Default, Serialize, Deserialize)]
struct State {
    contract_address: Option<Bech32Address>
}

impl State {
        // Deserializes state from file
        pub fn load_state() -> Self {
            if Path::new(STATE_FILE).exists() {
                let mut file = std::fs::File::open(STATE_FILE).unwrap();
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                toml::from_str(&content).unwrap()
            } else {
                Self::default()
            }
        }
    
        /// Sets the contract address
        pub fn set_address(&mut self, address: Bech32Address) {
            self.contract_address = Some(address);
        }
    
        /// Returns the contract address
        pub fn current_address(&self) -> &Bech32Address {
            self.contract_address
                .as_ref()
                .expect("no known contract, deploy first")
        }
    }
    
    impl Drop for State {
        // Serializes state to file
        fn drop(&mut self) {
            let mut file = std::fs::File::create(STATE_FILE).unwrap();
            file.write_all(toml::to_string(self).unwrap().as_bytes())
                .unwrap();
        }
    }

struct ContractInteract {
    interactor: Interactor,
    wallet_address: Address,
    user: Address,
    contract_code: BytesValue,
    state: State
}

impl ContractInteract {
    async fn new() -> Self {
        let mut interactor = Interactor::new(GATEWAY).await;
        let wallet_address = interactor.register_wallet(test_wallets::alice());
        let user = interactor.register_wallet(test_wallets::bob());

        let contract_code = BytesValue::interpret_from(
            "mxsc:../output/energy-factory.mxsc.json",
            &InterpreterContext::default(),
        );

        ContractInteract {
            interactor,
            user,
            wallet_address,
            contract_code,
            state: State::load_state()
        }
    }


      /// - base_asset_token_id: The only token that is accepted for the lockTokens endpoint. 
    ///     NOTE: The SC also needs the ESDTLocalMint and ESDTLocalBurn roles for this token. 
    /// - legacy_token_id: The token ID of the old locked asset. 
    ///     NOTE: The SC also needs the NFTBurn role for this token. 
    /// - old_locked_asset_factory_address 
    /// - min_migrated_token_locked_period - The minimum number of epochs that 
    ///     a migrated old LKMEX token will be locked for after the average is calculated 
    async fn deploy(&mut self) {
        let base_asset_token_id = TokenIdentifier::from_esdt_bytes(&b""[..]);
        let legacy_token_id = TokenIdentifier::from_esdt_bytes(&b""[..]);
        let old_locked_asset_factory_address = bech32::decode("");
        let min_migrated_token_locked_period = 0u64;
        let lock_options = MultiValueVec::from(vec![MultiValue2::from((0u64, 0u64))]);

        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .typed(proxy::SimpleLockEnergyProxy)
            .init(base_asset_token_id, legacy_token_id, old_locked_asset_factory_address, min_migrated_token_locked_period, lock_options)
            .code(&self.contract_code)
            .returns(ReturnsNewAddress)
            .prepare_async()
            .run()
            .await;
        let new_address_bech32 = bech32::encode(&new_address);
        self.state
            .set_address(Bech32Address::from_bech32_string(new_address_bech32.clone()));

        println!("new address: {new_address_bech32}");
    }


    async fn deployPetru(
        &mut self,
        base_asset_token_id: &str,   
        legacy_token_id: &str,
        old_locked_asset_factory_address: &str,
        min_migrated_token_locked_period: u64,
        lock_options: Vec<(u64, u64)>,
    ) {
        let base_asset_token_id = TokenIdentifier::from_esdt_bytes(base_asset_token_id.as_bytes());
        let legacy_token_id = TokenIdentifier::from_esdt_bytes(legacy_token_id.as_bytes());
        let old_locked_asset_factory_address = bech32::decode(old_locked_asset_factory_address);
        let lock_options = MultiValueVec::from(
            lock_options
                .iter()
                .map(|(a, b)| MultiValue2::from((*a, *b)))
                .collect::<Vec<_>>(),
        );

        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(200_000_000u64)
            .typed(proxy::SimpleLockEnergyProxy)
            .init(
                base_asset_token_id,
                legacy_token_id,
                old_locked_asset_factory_address,
                min_migrated_token_locked_period,
                lock_options,
            )
            .code(&self.contract_code)
            .returns(ReturnsNewAddress)
            .prepare_async()
            .run()
            .await;
        let new_address_bech32 = bech32::encode(&new_address);
        self.state.set_address(Bech32Address::from_bech32_string(
            new_address_bech32.clone(),
        ));

        println!("new address: {new_address_bech32}");
    }










    async fn lock_tokens_endpoint(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let lock_epochs = 0u64;
        let opt_destination = OptionalValue::Some(bech32::decode(""));

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .lock_tokens_endpoint(lock_epochs, opt_destination)
            .payment((TokenIdentifier::from(token_id.as_str()), token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }




    async fn lock_tokens_endpoint_parametri(
        &mut self,
        lock_epochs: u64,
        opt_destination: &str,
        token_id: &str,
        token_nonce: u64,
        token_amount: BigUint<StaticApi>,
    ) {
        let token_id = String::from(token_id);
       // let token_amount = BigUint::<StaticApi>::from(amount);
        let opt_destination = OptionalValue::Some(bech32::decode(opt_destination));

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(120_000_000u64)
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .lock_tokens_endpoint(lock_epochs, opt_destination)
            .payment((
                TokenIdentifier::from(token_id.as_str()), 
             //   TokenIdentifier::from("TST-af9b21"),
                token_nonce,
                token_amount,
            ))
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }


    async fn lock_tokens_endpoint_parametrii_fail(
        &mut self,
        lock_epochs: u64,
        opt_destination: &str,
        token_id: &str,
        token_nonce: u64,
        token_amount: BigUint<StaticApi>,
        expected_result : ExpectError<'_>
    ) {
        let token_id = String::from(token_id);
        let token_amount = BigUint::<StaticApi>::from(token_amount);
        let opt_destination = OptionalValue::Some(bech32::decode(opt_destination));

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(120_000_000u64)
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .lock_tokens_endpoint(lock_epochs, opt_destination)
            .payment((
                TokenIdentifier::from(token_id.as_str()),
                token_nonce,
                token_amount,
            ))
            .returns(expected_result)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }



    async fn unlock_tokens_endpoint(&mut self,    token_id: &str, token_nonce : u64, token_amount: BigUint<StaticApi> ) {
     //   let token_id = String::new();
  //      let token_nonce = 0u64;
      //  let token_amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(100_000_000u64)
            .typed(proxy::SimpleLockEnergyProxy)
            .unlock_tokens_endpoint()
            .payment((TokenIdentifier::from(token_id), token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn extend_lock_period(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let lock_epochs = 0u64;
        let user = bech32::decode("");

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .extend_lock_period(lock_epochs, user)
            .payment((TokenIdentifier::from(token_id.as_str()), token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn issue_locked_token(&mut self, name: &str, tt: &str, num_decimals: u32) {
        let egld_amount = BigUint::<StaticApi>::from(FEE);

        let token_display_name = ManagedBuffer::new_from_bytes(name.as_bytes());
        let token_ticker = ManagedBuffer::new_from_bytes(tt.as_bytes());

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(100_000_000u64)
            .typed(proxy::SimpleLockEnergyProxy)
            .issue_locked_token(token_display_name, token_ticker, num_decimals)
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }




    async fn locked_token(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .locked_token()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }


    async fn base_asset_token_id(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .base_asset_token_id()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn legacy_locked_token_id(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .legacy_locked_token_id()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn get_updated_energy_entry_for_user(&mut self) {
        let user = bech32::decode("");

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .get_updated_energy_entry_for_user(user)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn get_energy_amount_for_user(&mut self) {
        let user = bech32::decode("");

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .get_energy_amount_for_user(user)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    /*async fn add_lock_options(&mut self) {
        let new_lock_options = MultiValueVec::from(vec![MultiValue2::from((0u64, 0u64))]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .add_lock_options(new_lock_options)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }
*/
async fn add_lock_options(&mut self, lock_options: Vec<(u64, u64)>) {
    let new_lock_options = MultiValueVec::from(
        lock_options
            .iter()
            .map(|(a, b)| MultiValue2::from((*a, *b)))
            .collect::<Vec<_>>(),
    );
    let response = self
        .interactor
        .tx()
        .from(&self.wallet_address)
        .to(self.state.current_address())
        .typed(proxy::SimpleLockEnergyProxy)
        .add_lock_options(new_lock_options)
        .returns(ReturnsResultUnmanaged)
        .prepare_async()
        .run()
        .await;

    println!("Result: {response:?}");
}


    async fn get_lock_options_view(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .get_lock_options_view()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn get_lock_options_view_fail(&mut self,expected_result : ExpectError<'_>) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .get_lock_options_view()
            .returns(expected_result)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn unlock_early(&mut self,   token_id: &str, token_nonce : u64, token_amount: BigUint<StaticApi>) {
     

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(100_000_000u64)
            .typed(proxy::SimpleLockEnergyProxy)
            .unlock_early()
            .payment((TokenIdentifier::from(token_id), token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }



    async fn unlock_early_fail(&mut self,   token_id: &str, token_nonce : u64, token_amount: BigUint<StaticApi>, expected_result : ExpectError<'_>) {
     

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .unlock_early()
            .payment((TokenIdentifier::from(token_id), token_nonce, token_amount))
            .returns(expected_result)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }
    async fn reduce_lock_period(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let new_lock_period = 0u64;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .reduce_lock_period(new_lock_period)
            .payment((TokenIdentifier::from(token_id.as_str()), token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn calculate_penalty_amount(&mut self) {
        let token_amount = BigUint::<StaticApi>::from(0u128);
        let prev_lock_epochs = 0u64;
        let new_lock_epochs = 0u64;

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .calculate_penalty_amount(token_amount, prev_lock_epochs, new_lock_epochs)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn set_token_unstake_address(&mut self) {
        let sc_address = bech32::decode("");

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .set_token_unstake_address(sc_address)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn revert_unstake(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let user = bech32::decode("");
        let new_energy = proxy::Energy{
            amount: BigInt::from(0i32),
            last_update_epoch: 0u64,
            total_locked_tokens: BigUint::<StaticApi>::from(0u128)
        };

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .revert_unstake(user, new_energy)
            .payment((TokenIdentifier::from(token_id.as_str()), token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn token_unstake_sc_address(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .token_unstake_sc_address()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    /* 
    async fn set_energy_for_old_tokens(&mut self) {


        // MultiValueEncoded<MultiValue3<ManagedAddress, BigUint, BigInt>>

        let mut users_energy = MultiValueEncoded::new();
        users_energy.push(MultiValue3((ManagedAddress::zero(), BigUint::from(0u128), BigInt::from(0i64))));
       
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .set_energy_for_old_tokens(users_energy)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }
*/




async fn set_energy_for_old_tokens(&mut self,  user: &str) {
    let users_energy = MultiValueVec::from(vec![MultiValue3::from((
        bech32::decode(user),
        BigUint::<StaticApi>::from(0u128),
        BigInt::<StaticApi>::from(0i64),
    ))]);

    let response = self
        .interactor
        .tx()
        .from(&self.wallet_address)
        .to(self.state.current_address())
        .typed(proxy::SimpleLockEnergyProxy)
        .set_energy_for_old_tokens(users_energy)
        .returns(ReturnsResultUnmanaged)
        .prepare_async()
        .run()
        .await;

    println!("Result: {response:?}");
}




async fn set_energy_for_old_tokens_not_paused_fail(&mut self,  address: &str, expected_result: ExpectError<'_>) {
    let users_energy = MultiValueVec::from(vec![MultiValue3::from((
        bech32::decode(address),
        BigUint::<StaticApi>::from(0u128),
        BigInt::<StaticApi>::from(0i64),
    ))]);

    let response = self
        .interactor
        .tx()
        .from(&self.wallet_address)
        .to(self.state.current_address())
        .typed(proxy::SimpleLockEnergyProxy)
        .set_energy_for_old_tokens(users_energy)
        .returns(expected_result)
        .prepare_async()
        .run()
        .await;

    println!("Result: {response:?}");
}


async fn set_energy_for_old_tokens_by_user_fail(&mut self,  address: &str, expected_result: ExpectError<'_>) {
    let users_energy = MultiValueVec::from(vec![MultiValue3::from((
        bech32::decode(address),
        BigUint::<StaticApi>::from(0u128),
        BigInt::<StaticApi>::from(0i64),
    ))]);

    let response = self
        .interactor
        .tx()
        .from(&self.user)
        .to(self.state.current_address())
        .typed(proxy::SimpleLockEnergyProxy)
        .set_energy_for_old_tokens(users_energy)
        .returns(expected_result)
        .prepare_async()
        .run()
        .await;

    println!("Result: {response:?}");
}





/////////////////////////////         BIANCA                    ////// 


    async fn update_energy_after_old_token_unlock(&mut self) {
        let original_caller = bech32::decode("");
        let mut pereche = ArrayVec::new();
        pereche.push(EpochAmountPair{
             epoch: 0u64,
             amount: BigUint::from(0u128),
        });


        let initial_epoch_amount_pairs = UnlockEpochAmountPairs {
       
            pairs:pereche
        };


        let mut pereche2 = ArrayVec::new();
        pereche2.push(EpochAmountPair{
             epoch: 0u64,
             amount: BigUint::from(0u128),
        });


        let final_epoch_amount_pairs = UnlockEpochAmountPairs {
       
            pairs:pereche2
        };


        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .update_energy_after_old_token_unlock(original_caller, initial_epoch_amount_pairs, final_epoch_amount_pairs)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn migrate_old_tokens(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .migrate_old_tokens()
            .payment((TokenIdentifier::from(token_id.as_str()), token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn pause_endpoint(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .pause_endpoint()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn unpause_endpoint(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .unpause_endpoint()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn paused_status(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .paused_status()
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    async fn set_transfer_role(&mut self) {
        let opt_address = OptionalValue::Some(bech32::decode(""));

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .set_transfer_role(opt_address)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn set_burn_role(&mut self) {
        let address = bech32::decode("");

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .set_burn_role(address)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn merge_tokens_endpoint(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let opt_original_caller = OptionalValue::Some(bech32::decode(""));

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .merge_tokens_endpoint(opt_original_caller)
            .payment((TokenIdentifier::from(token_id.as_str()), token_nonce, token_amount))
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    /* 
    async fn lock_virtual(&mut self) {
        let token_id = TokenIdentifier::from_esdt_bytes(&b""[..]);
        let amount = BigUint::<StaticApi>::from(0u128);
        let lock_epochs = 0u64;
        let dest_address = bech32::decode("");
        let energy_address = bech32::decode("");

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .lock_virtual(token_id, amount, lock_epochs, dest_address, energy_address)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }
*/

    async fn lock_virtual_parametrii(&mut self, lock_epochs: u64,  token_id: &str,  token_amount: u128, dest_address: &str, energy_address : &str  ) 
     {
      
        let token_id = String::from(token_id);
        let token_amount = BigUint::<StaticApi>::from(token_amount);   
        
        let dest_address = bech32::decode(dest_address);
        let energy_address = bech32::decode(energy_address);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(100_000_000u64)
            .typed(proxy::SimpleLockEnergyProxy)
            .lock_virtual(  TokenIdentifier::from(token_id.as_str()), token_amount, lock_epochs, dest_address, energy_address)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }
    


    async fn lock_virtual_fail(&mut self, lock_epochs: u64,  token_id: &str,  token_amount: u128, dest_address: &str, energy_address : &str, expected_result : ExpectError<'_>  ) 
     {
      
        let token_id = String::from(token_id);
        let token_amount = BigUint::<StaticApi>::from(token_amount);   
        
        let dest_address = bech32::decode(dest_address);
        let energy_address = bech32::decode(energy_address);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .lock_virtual(  TokenIdentifier::from(token_id.as_str()), token_amount, lock_epochs, dest_address, energy_address)
            .returns(expected_result)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }
    

    async fn add_sc_address_to_whitelist(&mut self, addresa : &str) {
        let address = bech32::decode(addresa);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .add_sc_address_to_whitelist(address)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }
    async fn add_sc_address_to_whitelist_twice_fail(&mut self, adresa : &str, expected_result : ExpectError<'_>) {
        let address = bech32::decode(adresa);
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .add_sc_address_to_whitelist(address)
            .returns(expected_result)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }



    async fn remove_sc_address_from_whitelist(&mut self,adresa : &str,) {
        let address = bech32::decode(adresa);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .remove_sc_address_from_whitelist(address)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn remove_sc_address_from_whitelist_fail(&mut self,adresa : &str,expected_result : ExpectError<'_>) {
        let address = bech32::decode(adresa);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .remove_sc_address_from_whitelist(address)
            .returns(expected_result)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn is_sc_address_whitelisted(&mut self, address : &str) -> bool {
            
        let address = bech32::decode(address);

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .is_sc_address_whitelisted(address)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {result_value:?}");
        result_value
    }

    async fn add_to_token_transfer_whitelist(&mut self) {
        let sc_addresses = MultiValueVec::from(vec![bech32::decode("erd1qqqqqqqqqqqqqpgqlyyexsvg2kc2eqk9ellagd78ltm6lce9eccsytn0a9")]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .add_to_token_transfer_whitelist(sc_addresses)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }


 /* 
    async fn add_to_token_transfer_whitelist(
        &mut self,
   
        sc_addresses: Vec<Bech32Address>,
    ) {
        let sc_addresses = MultiValueVec::from(sc_addresses);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .add_to_token_transfer_whitelist(sc_addresses)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }
*/
    async fn remove_from_token_transfer_whitelist(&mut self) {
        let sc_addresses = MultiValueVec::from(vec![bech32::decode("")]);

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .remove_from_token_transfer_whitelist(sc_addresses)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }

    async fn set_user_energy_after_locked_token_transfer(&mut self) {
        let user = bech32::decode("");

        
        let energy = proxy::Energy{
            amount: BigInt::from(0i32),
            last_update_epoch: 0u64,
            total_locked_tokens: BigUint::<StaticApi>::from(0u128)
        };

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .typed(proxy::SimpleLockEnergyProxy)
            .set_user_energy_after_locked_token_transfer(user, energy)
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;

        println!("Result: {response:?}");
    }



    async fn set_role(&mut self, token_id: &str) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(70_000_000u64)
            .typed(proxy::SimpleLockEnergyProxy)
            .set_role(TokenIdentifier::from(token_id))
            .returns(ReturnsResultUnmanaged)
            .prepare_async()
            .run()
            .await;


        println!("Set roles: {response:?}");
    }



 


}



#[tokio::test]
async fn test_deploy() {
    let mut interact = ContractInteract::new().await;
    interact
        .deployPetru(
            BASE_ASSET_TOKEN_ID,
            LEGACY_TOKEN_ID,
            OLD_LOCKED_ASSET_FACTORY_ADDRESS,
            180,
            vec![(360, 0), (361, 2_000), (362, 5_000)],
        )
        .await;
}




////////////////////////////////// EROARE  Invalid option
#[tokio::test]
async fn test_get_lock_options_null_fail() {
    let mut interact = ContractInteract::new().await;

    interact
        .deployPetru(
            BASE_ASSET_TOKEN_ID,
            LEGACY_TOKEN_ID,
            OLD_LOCKED_ASSET_FACTORY_ADDRESS,
            180,
            vec![(0,0)],  // 
        )
        .await;

      //  interact.get_lock_options_view().await; 
interact.get_lock_options_view_fail( ExpectError(4, "no lock options available")).await;

}



//VERIFICARE STARE CONTRACT
#[tokio::test]
async fn test_paused_status() {
    let mut interact = ContractInteract::new().await;
    
    interact.paused_status().await; 
   
}

//AFISARE LOCK OPTIONS 
#[tokio::test]
async fn test_get_lock_options_view() {
    let mut interact = ContractInteract::new().await;
    
    interact.get_lock_options_view().await; 
   
}

//?
#[tokio::test]
async fn test_add_lock_options() {
    let mut interact = ContractInteract::new().await;
interact.add_lock_options(  vec![]).await


}


#[tokio::test]
async fn test_extend_new_token_period_fAIL() {
    let mut interact = ContractInteract::new().await;
    interact.unpause_endpoint().await;

    let token_amount = BigUint::<StaticApi>::from(10000000000000000000u128);


    //extend_new_token_period CAND FACEM DE 2 ORI
 //   interact
 //   .lock_tokens_endpoint_parametrii(360, ALICE_ADDRESS, TOKEN_ISSUED, 1, token_amount )
  //  .await;


    interact
        .lock_tokens_endpoint_parametrii_fail(360, ALICE_ADDRESS, TOKEN_ISSUED, 1, token_amount, ExpectError(4,"New lock period must be longer than the current one") )
        .await;
}








/*
// Error " Unlock epoch must be greater than the current epoch" din , dar daca pun orice valoare mai mica de 360 la lock epochs e prinsa de eroarea  Invalid lock choice
#[tokio::test]
async fn test_lock_tokens_fail() {
    let mut interact = ContractInteract::new().await;
    interact.unpause_endpoint().await;
    interact
        .lock_tokens_endpoint_parametrii_fail(360, ALICE_ADDRESS, BASE_ASSET_TOKEN_ID, 0, 1, ExpectError(4,"Unlock epoch must be greater than the current epoch") )
        .await;
}

 */

// Negative test: set_energy_for_old_tokens contract is not paused 
#[tokio::test]
async fn test_set_energy_for_old_tokens_fail_contract_not_paused() {
    let mut interact = ContractInteract::new().await;
    
  //  interact.paused_status().await; 
    interact.unpause_endpoint().await; 
    interact.set_energy_for_old_tokens_not_paused_fail(ALICE_ADDRESS,  ExpectError(4,"Contract is not paused") ).await;

    interact.pause_endpoint().await;
}



// Negative test: set_energy_for_old_tokens  by user not owner
#[tokio::test]
async fn test_set_energy_for_old_tokens_fail_by_user() {
    let mut interact = ContractInteract::new().await;
    
 //   interact.paused_status().await; 
    interact.pause_endpoint().await; 
    interact.set_energy_for_old_tokens_by_user_fail(BOB_ADDRESS,  ExpectError(4, "Endpoint can only be called by owner") ).await;

    interact.unpause_endpoint().await;
}


#[tokio::test]
async fn test_lock_virtual_wrong_token_fail() {
    let mut interact = ContractInteract::new().await;
    
    interact.unpause_endpoint().await; 
    interact.lock_virtual_fail(360,TOKEN_ISSUED , 1, BOB_ADDRESS, ALICE_ADDRESS ,  ExpectError(4, "May only lock the base asset token")).await;
}


#[tokio::test]
async fn test_lock_virtual_contract_paused_fail() {
    let mut interact = ContractInteract::new().await;
    
    interact.pause_endpoint().await; 
    interact.lock_virtual_fail(360,BASE_ASSET_TOKEN_ID , 1, BOB_ADDRESS, ALICE_ADDRESS ,  ExpectError(4, "Contract is paused")).await;
}


#[tokio::test]
async fn test_lock_virtual_null_amount_fail() {
    let mut interact = ContractInteract::new().await;
    
    interact.unpause_endpoint().await; 
    interact.lock_virtual_fail(360, BASE_ASSET_TOKEN_ID , 0, BOB_ADDRESS, ALICE_ADDRESS ,  ExpectError(4, "Amount cannot be 0")).await;
}

#[tokio::test]
async fn test_lock_virtual_wrong_lock_epochs_fail() {
    let mut interact = ContractInteract::new().await;
    
    interact.unpause_endpoint().await; 
    interact.lock_virtual_fail(0, BASE_ASSET_TOKEN_ID , 1, BOB_ADDRESS, ALICE_ADDRESS ,  ExpectError(4, "Invalid lock choice")).await;
}



//Pica cu Invalid lock choice
#[tokio::test]
async fn test_lock_virtual_wrong_unlock_epoch_fail() {
    let mut interact = ContractInteract::new().await;
    
    interact.unpause_endpoint().await; 
    interact.lock_virtual_fail(300, BASE_ASSET_TOKEN_ID , 1, BOB_ADDRESS, ALICE_ADDRESS ,  ExpectError(4, "Unlock epoch must be greater than the current epoch")).await;
}


#[tokio::test]
async fn test_is_sc_address_whitelisted() {
    let mut interact = ContractInteract::new().await;
  //  interact.is_sc_address_whitelisted(ALICE_ADDRESS).await;
    interact.is_sc_address_whitelisted(BOB_ADDRESS).await;

}


//OK
#[tokio::test]
async fn test_lock_virtual_sc_not_whitelisted_fail() {
    let mut interact = ContractInteract::new().await;
    interact.is_sc_address_whitelisted(ALICE_ADDRESS).await;
    interact.is_sc_address_whitelisted(BOB_ADDRESS).await;
    interact.unpause_endpoint().await; 
    interact.lock_virtual_fail(360, BASE_ASSET_TOKEN_ID ,1, BOB_ADDRESS, ALICE_ADDRESS ,  ExpectError(4, "Item not whitelisted")).await;
}




#[tokio::test]
async fn test_add_sc_address_to_whitelist() {
    let mut interact = ContractInteract::new().await;

    interact
        .add_sc_address_to_whitelist(ALICE_ADDRESS).await;
}



#[tokio::test]
async fn test_add_sc_address_to_whitelist_twice_fail() {
    let mut interact = ContractInteract::new().await;
    let result = interact.is_sc_address_whitelisted(SC_ADDRESS_SIMPLE_LOCK).await;

    if result {
        
        interact.add_sc_address_to_whitelist_twice_fail(SC_ADDRESS_SIMPLE_LOCK, ExpectError(4, "Bad parameters")).await;
    } else {
        
        interact.add_sc_address_to_whitelist(SC_ADDRESS_SIMPLE_LOCK).await;
        interact.add_sc_address_to_whitelist_twice_fail(SC_ADDRESS_SIMPLE_LOCK, ExpectError(4, "Bad parameters")).await;
    }
}


#[tokio::test]
async fn test_remove_sc_address_to_whitelist() {
    let mut interact = ContractInteract::new().await;
   
    interact
        .remove_sc_address_from_whitelist(ALICE_ADDRESS).await;
}


#[tokio::test]
async fn test_remove_an_inexistent_sc_address_to_whitelist() {
    let mut interact = ContractInteract::new().await;
   
    let result = interact.is_sc_address_whitelisted(SC_ADDRESS_SIMPLE_LOCK).await;

    if result { 
      interact.remove_sc_address_from_whitelist(SC_ADDRESS_SIMPLE_LOCK).await;
      interact.remove_sc_address_from_whitelist_fail(SC_ADDRESS_SIMPLE_LOCK, ExpectError(4, "Bad parameters")).await; }
    else {
    interact
        .remove_sc_address_from_whitelist_fail(SC_ADDRESS_SIMPLE_LOCK, ExpectError(4, "Bad parameters")).await;}
}





#[tokio::test]
async fn test_virtual_lock() {
    let mut interact = ContractInteract::new().await;

    interact.unpause_endpoint().await; 

    let result = interact.is_sc_address_whitelisted(ALICE_ADDRESS).await;

    if !result {
        
        interact.add_sc_address_to_whitelist(ALICE_ADDRESS).await;
    }

    interact.lock_virtual_parametrii(360, BASE_ASSET_TOKEN_ID , 2, BOB_ADDRESS, ALICE_ADDRESS ).await;

}




#[tokio::test]
async fn test_unlock_early_contract_paused_fail() {
    let mut interact = ContractInteract::new().await;

    let token_amount = BigUint::<StaticApi>::from(1000000000000000u128);
    interact.pause_endpoint().await; 
    interact.unlock_early_fail(BASE_ASSET_TOKEN_ID, 0, token_amount, ExpectError(4, "Contract is paused")).await
}


#[tokio::test]
async fn test_unlock_early_wrong_token_fail() {
    let mut interact = ContractInteract::new().await;

    let token_amount = BigUint::<StaticApi>::from(1000000000000000u128);
    interact.unpause_endpoint().await; 
    interact.unlock_early_fail(ANOTHER_TOKEN_ID, 0, token_amount, ExpectError(4, "Invalid payment token")).await
}






/*
#[tokio::test]
async fn test_unlock() {
    let mut interact = ContractInteract::new().await;

    let token_amount = BigUint::<StaticApi>::from(10000000000000000000u128);
    interact.unpause_endpoint().await; 
    interact.unlock_tokens_endpoint(TOKEN, 2, token_amount).await
}

 */





#[tokio::test]
async fn test_add_to_token_transfer_whitelist() {
    let mut interact = ContractInteract::new().await;

    interact
        .add_to_token_transfer_whitelist()
        .await;
}








#[tokio::test]
async fn test_deploy_issue_locked_token() {
    let mut interact = ContractInteract::new().await;

    interact
        .deployPetru(
            BASE_ASSET_TOKEN_ID,
            LEGACY_TOKEN_ID,
            OLD_LOCKED_ASSET_FACTORY_ADDRESS,
            180,
            vec![(360, 0), (361, 2_000), (362, 5_000)],
        )
        .await;

    interact.issue_locked_token( "EnergyF", "CCC", 18).await;
}


///BBB-234252
#[tokio::test]
async fn test_get_locked_token() {
    let mut interact = ContractInteract::new().await;

    interact.locked_token().await;

}



#[tokio::test]
async fn test_lock_tokens() {
    let mut interact = ContractInteract::new().await;
    interact.unpause_endpoint().await;

    let token_amount = BigUint::<StaticApi>::from(10000000000000000000u128);

    interact.lock_tokens_endpoint_parametri(360, ALICE_ADDRESS, BASE_ASSET_TOKEN_ID, 0, token_amount ).await;
}



#[tokio::test]
async fn test_unlock_early() {
    let mut interact = ContractInteract::new().await;

    let token_amount = BigUint::<StaticApi>::from(10000000000000000000u128);
    interact.unpause_endpoint().await; 
    interact.unlock_early(TOKEN_ISSUED, 1, token_amount).await
}



#[tokio::test]
async fn test_set_role_mint_base_asset_token() {
    let mut interact = ContractInteract::new().await;
   
    interact.set_role(TOKEN_ISSUED).await;
}