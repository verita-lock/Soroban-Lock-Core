//! Vulnerability detectors for Soroban smart contracts.

pub mod admin;
pub mod admin_in_temp;
pub mod admin_key_removal;
pub mod admin_overwrite;
pub mod amount_mul_overflow;
pub mod assert_for_auth;
pub mod auth;
pub mod auth_loop_dos;
pub mod auth_shadow;
pub mod authorize_as_contract;
pub mod authorize_empty;
pub mod deploy_arg_auth;
pub mod balance_overflow;
pub mod broken_pause;
pub mod bump_to_ttl;
pub mod burn_auth;
pub mod bytes_not_bytesn;
pub mod contracterror_attr;
pub mod contracttype;
pub mod current_contract_unwrap;
pub mod debug_entrypoint;
pub mod dynamic_symbol_key;
pub mod env_in_struct;
pub mod extend_ttl_in_loop;
pub mod float_arithmetic;
pub mod hash_as_storage_key;
pub mod instance_domain_mixing;
pub mod instance_remove_critical;
pub mod instance_ttl;
pub mod instance_vec_growth;
pub mod key_prefix_collision;
pub mod linear_whitelist_scan;
pub mod lock_period_truncation;
pub mod invoke_unchecked_cast;
pub mod key_length_exceeded;
pub mod map_key_explosion;
pub mod map_user_key_bloat;
pub mod migration_guard;
pub mod mint_auth;
pub mod missing_ttl;
pub mod negative_deposit;
pub mod no_param_no_auth;
pub mod no_std;
pub mod nonce_increment_order;
pub mod overflow;
pub mod ownership_transfer;
pub mod persistent_overwrite;
pub mod instance_set_no_has;
pub mod storage_type_version;
pub mod ttl_before_write;
pub mod uncapped_fee;
pub mod unlimited_allowance;
pub mod panic_usage;
pub mod partial_write_on_error;
pub mod persistent_for_temp;
pub mod reentrancy;
pub mod secp256k1_unchecked;
pub mod self_transfer;
pub mod sequence_as_key;
pub mod sequence_nonce;
pub mod storage;
pub mod storage_has_get_mismatch;
pub mod storage_no_cache;
pub mod storage_type_confusion;
pub mod temp_get_no_has;
pub mod temp_read_in_view;
pub mod temp_set_no_ttl;
pub mod tier_key_collision;
pub mod timestamp_expiry_no_min;
pub mod timestamp_truncation;
pub mod token_burn_auth;
pub mod token_transfer_unchecked;
pub mod transfer_to_self;
pub mod ttl_arg_order;
pub mod unauth_address_in_struct;
pub mod uncapped_slippage;
pub mod unauth_fee_setter;
pub mod unauth_sensitive_read;
pub mod unbounded_batch;
pub mod unbounded_input_storage;
pub mod unbounded_storage;
pub mod unvalidated_invoke_target;
pub mod unvalidated_price;
pub mod redundant_auth_args;
mod util;
pub mod vec_push_in_loop;
pub mod vesting_cliff;
pub mod weak_randomness;
pub mod withdraw_auth;
pub mod wrapping_balance_op;
pub mod zero_amount;

pub use admin::UnprotectedAdminCheck;
pub use admin_in_temp::AdminInTempCheck;
pub use admin_key_removal::AdminKeyRemovalCheck;
pub use admin_overwrite::AdminOverwriteCheck;
pub use amount_mul_overflow::AmountMulOverflowCheck;
pub use assert_for_auth::AssertForAuthCheck;
pub use auth::MissingRequireAuthCheck;
pub use auth_loop_dos::AuthLoopDosCheck;
pub use auth_shadow::AuthShadowCheck;
pub use authorize_as_contract::AuthorizeAsContractCheck;
pub use authorize_empty::AuthorizeEmptyCheck;
pub use deploy_arg_auth::DeployArgAuthCheck;
pub use balance_overflow::BalanceOverflowCheck;
pub use broken_pause::BrokenPauseCheck;
pub use bump_to_ttl::BumpToTtlCheck;
pub use burn_auth::BurnAuthCheck;
pub use bytes_not_bytesn::BytesNotBytesNCheck;
pub use contracterror_attr::ContracterrorAttrCheck;
pub use contracttype::MissingContracttypeCheck;
pub use current_contract_unwrap::CurrentContractUnwrapCheck;
pub use debug_entrypoint::DebugEntrypointCheck;
pub use dynamic_symbol_key::DynamicSymbolKeyCheck;
pub use env_in_struct::EnvInStructCheck;
pub use extend_ttl_in_loop::ExtendTtlInLoopCheck;
pub use float_arithmetic::FloatArithmeticCheck;
pub use hash_as_storage_key::HashAsStorageKeyCheck;
pub use instance_domain_mixing::InstanceDomainMixingCheck;
pub use instance_remove_critical::InstanceRemoveCriticalCheck;
pub use instance_ttl::InstanceTtlCheck;
pub use instance_vec_growth::InstanceVecGrowthCheck;
pub use invoke_unchecked_cast::InvokeUncheckedCastCheck;
pub use key_prefix_collision::KeyPrefixCollisionCheck;
pub use linear_whitelist_scan::LinearWhitelistScanCheck;
pub use lock_period_truncation::LockPeriodTruncationCheck;
pub use map_key_explosion::MapKeyExplosionCheck;
pub use map_user_key_bloat::MapUserKeyBloatCheck;
pub use migration_guard::MigrationGuardCheck;
pub use mint_auth::MintAuthCheck;
pub use missing_ttl::MissingTtlExtensionCheck;
pub use negative_deposit::NegativeDepositCheck;
pub use no_param_no_auth::NoParamNoAuthCheck;
pub use no_std::NoStdCheck;
pub use nonce_increment_order::NonceIncrementOrderCheck;
pub use overflow::UncheckedArithmeticCheck;
pub use ownership_transfer::OwnershipTransferCheck;
pub use persistent_overwrite::PersistentOverwriteCheck;
pub use instance_set_no_has::InstanceSetNoHasCheck;
pub use storage_type_version::StorageTypeVersionCheck;
pub use ttl_before_write::TtlBeforeWriteCheck;
pub use uncapped_fee::UncappedFeeCheck;
pub use unlimited_allowance::UnlimitedAllowanceCheck;
pub use panic_usage::PanicUsageCheck;
pub use partial_write_on_error::PartialWriteOnErrorCheck;
pub use persistent_for_temp::PersistentForTempCheck;
pub use reentrancy::ReentrancyCheck;
pub use secp256k1_unchecked::Secp256k1UncheckedCheck;
pub use self_transfer::SelfTransferCheck;
pub use sequence_as_key::SequenceAsKeyCheck;
pub use sequence_nonce::SequenceNonceCheck;
pub use storage::UnsafeStoragePatternsCheck;
pub use storage_has_get_mismatch::StorageHasGetMismatchCheck;
pub use storage_no_cache::StorageNoCacheCheck;
pub use storage_type_confusion::StorageTypeConfusionCheck;
pub use temp_get_no_has::TempGetNoHasCheck;
pub use temp_set_no_ttl::TempSetNoTtlCheck;
pub use tier_key_collision::TierKeyCollisionCheck;
pub use timestamp_expiry_no_min::TimestampExpiryNoMinCheck;
pub use timestamp_truncation::TimestampTruncationCheck;
pub use token_burn_auth::TokenBurnAuthCheck;
pub use token_transfer_unchecked::TokenTransferUncheckedCheck;
pub use transfer_to_self::TransferToSelfCheck;
pub use ttl_arg_order::TtlArgOrderCheck;
pub use unauth_address_in_struct::UnauthAddressInStructCheck;
pub use uncapped_slippage::UncappedSlippageCheck;
pub use unauth_fee_setter::UnauthFeeSetterCheck;
pub use unauth_sensitive_read::UnauthSensitiveReadCheck;
pub use unbounded_batch::UnboundedBatchCheck;
pub use unbounded_input_storage::UnboundedInputStorageCheck;
pub use unbounded_storage::UnboundedStorageCheck;
pub use unvalidated_price::UnvalidatedPriceCheck;
pub use redundant_auth_args::RedundantAuthArgsCheck;
pub use vec_push_in_loop::VecPushInLoopCheck;
pub use vesting_cliff::VestingCliffCheck;
pub use weak_randomness::WeakRandomnessCheck;
pub use withdraw_auth::WithdrawAuthCheck;
pub use wrapping_balance_op::WrappingBalanceOpCheck;
pub use zero_amount::ZeroAmountCheck;

use serde::Serialize;
use syn::File;

/// Severity of a finding.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum Severity {
    High,
    Medium,
    Low,
}

/// One issue reported by a check.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Finding {
    pub check_name: String,
    pub severity: Severity,
    pub file_path: String,
    pub line: usize,
    pub function_name: String,
    pub description: String,
}

/// A static analyzer check implemented against a parsed `syn::File`.
pub trait Check {
    fn name(&self) -> &str;
    fn run(&self, file: &File, source: &str) -> Vec<Finding>;
}

/// All checks executed by the analyzer (extend here as you add detectors).
///
/// Checks are **stateless and isolated**: implementations must not use shared mutable
/// static state or assume a particular invocation order. The analyzer runs each check
/// against the same parsed `syn::File` independently and concatenates `Finding`s.
pub fn default_checks() -> Vec<Box<dyn Check + Send + Sync>> {
    vec![
        Box::new(MissingRequireAuthCheck),
        Box::new(UncheckedArithmeticCheck),
        Box::new(UnprotectedAdminCheck),
        Box::new(AdminOverwriteCheck),
        Box::new(AdminKeyRemovalCheck),
        Box::new(UnsafeStoragePatternsCheck),
        Box::new(InstanceDomainMixingCheck),
        Box::new(PanicUsageCheck),
        Box::new(PartialWriteOnErrorCheck),
        Box::new(MissingContracttypeCheck),
        Box::new(UnboundedStorageCheck),
        Box::new(UnboundedInputStorageCheck),
        Box::new(ZeroAmountCheck),
        Box::new(SelfTransferCheck),
        Box::new(SequenceAsKeyCheck),
        Box::new(NoStdCheck),
        Box::new(EnvInStructCheck),
        Box::new(TempGetNoHasCheck),
        Box::new(AmountMulOverflowCheck),
        Box::new(FloatArithmeticCheck),
        Box::new(WeakRandomnessCheck),
        Box::new(ReentrancyCheck),
        Box::new(TokenTransferUncheckedCheck),
        Box::new(ContracterrorAttrCheck),
        Box::new(TokenBurnAuthCheck),
        Box::new(MintAuthCheck),
        Box::new(SequenceNonceCheck),
        Box::new(AssertForAuthCheck),
        Box::new(AuthorizeAsContractCheck),
        Box::new(AuthorizeEmptyCheck),
        Box::new(AddressCmpInsteadOfAuthCheck),
        Box::new(DeployArgAuthCheck),
        Box::new(AuthTempStorageCheck),
        Box::new(MapKeyExplosionCheck),
        Box::new(DynamicSymbolKeyCheck),
        Box::new(InstanceVecGrowthCheck),
        Box::new(MigrationGuardCheck),
        Box::new(WithdrawAuthCheck),
        Box::new(BrokenPauseCheck),
        Box::new(BytesNotBytesNCheck),
        Box::new(DebugEntrypointCheck),
        Box::new(ExtendTtlInLoopCheck),
        Box::new(HashAsStorageKeyCheck),
        Box::new(UnauthAddressInStructCheck),
        Box::new(InvokeUncheckedCastCheck),
        Box::new(NegativeDepositCheck),
        Box::new(NoParamNoAuthCheck),
        Box::new(StorageTypeConfusionCheck),
        Box::new(AuthLoopDosCheck),
        Box::new(OwnershipTransferCheck),
        Box::new(Secp256k1UncheckedCheck),
        Box::new(TempSetNoTtlCheck),
        Box::new(BalanceOverflowCheck),
        Box::new(WrappingBalanceOpCheck),
        Box::new(UnauthSensitiveReadCheck),
        Box::new(InstanceRemoveCriticalCheck),
        Box::new(MapUserKeyBloatCheck),
        Box::new(TimestampTruncationCheck),
        Box::new(UnlimitedAllowanceCheck),
        Box::new(LockPeriodTruncationCheck),
        Box::new(LinearWhitelistScanCheck),
        Box::new(UncappedSlippageCheck),
        Box::new(NonceIncrementOrderCheck),
        Box::new(TierKeyCollisionCheck),
    ]
}
