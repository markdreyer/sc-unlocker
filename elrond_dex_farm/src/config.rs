elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, PartialEq, TypeAbi)]
pub enum State {
    Inactive,
    Active,
}

#[elrond_wasm_derive::module]
pub trait ConfigModule {
    #[inline]
    fn is_active(&self) -> bool {
        let state = self.state().get();
        state == State::Active
    }

    fn require_permissions(&self) -> SCResult<()> {
        let caller = self.blockchain().get_caller();
        let owner = self.owner().get();
        let router = self.router_address().get();
        require!(caller == owner || caller == router, "Permission denied");
        Ok(())
    }

    #[endpoint]
    fn set_penalty_percent(&self, percent: u8) -> SCResult<()> {
        self.require_permissions()?;
        self.penalty_percent().set(&percent);
        Ok(())
    }

    #[endpoint]
    fn set_locked_rewards_liquidity_multiplier(&self, muliplier: u8) -> SCResult<()> {
        self.require_permissions()?;
        self.locked_rewards_liquidity_multiplier().set(&muliplier);
        Ok(())
    }

    #[endpoint]
    fn set_burn_tokens_gas_limit(&self, limit: u64) -> SCResult<()> {
        self.require_permissions()?;
        self.burn_tokens_gas_limit().set(&limit);
        Ok(())
    }

    #[endpoint]
    fn set_minimum_farming_epochs(&self, epochs: u8) -> SCResult<()> {
        self.require_permissions()?;
        self.minimum_farming_epochs().set(&epochs);
        Ok(())
    }

    #[view(getLastErrorMessage)]
    #[storage_mapper("last_error_message")]
    fn last_error_message(&self) -> SingleValueMapper<Self::Storage, BoxedBytes>;

    #[view(getRouterAddress)]
    #[storage_mapper("router_address")]
    fn router_address(&self) -> SingleValueMapper<Self::Storage, Address>;

    #[view(getState)]
    #[storage_mapper("state")]
    fn state(&self) -> SingleValueMapper<Self::Storage, State>;

    #[view(getOwner)]
    #[storage_mapper("owner")]
    fn owner(&self) -> SingleValueMapper<Self::Storage, Address>;

    #[view(getFarmingTokenId)]
    #[storage_mapper("farming_token_id")]
    fn farming_token_id(&self) -> SingleValueMapper<Self::Storage, TokenIdentifier>;

    #[view(getRewardTokenId)]
    #[storage_mapper("reward_token_id")]
    fn reward_token_id(&self) -> SingleValueMapper<Self::Storage, TokenIdentifier>;

    #[view(getLockedAssetFactoryAddress)]
    #[storage_mapper("locked_asset_factory_address")]
    fn locked_asset_factory_address(&self) -> SingleValueMapper<Self::Storage, Address>;

    #[view(getPenaltyPercent)]
    #[storage_mapper("penalty_percent")]
    fn penalty_percent(&self) -> SingleValueMapper<Self::Storage, u8>;

    #[view(getLockedRewardLiquidityMuliplier)]
    #[storage_mapper("locked_rewards_liquidity_mulitplier")]
    fn locked_rewards_liquidity_multiplier(&self) -> SingleValueMapper<Self::Storage, u8>;

    #[view(getBurnTokensGasLimit)]
    #[storage_mapper("burn_tokens_gas_limit")]
    fn burn_tokens_gas_limit(&self) -> SingleValueMapper<Self::Storage, u64>;

    #[view(getMinimumFarmingEpoch)]
    #[storage_mapper("minimum_farming_epochs")]
    fn minimum_farming_epochs(&self) -> SingleValueMapper<Self::Storage, u8>;
}