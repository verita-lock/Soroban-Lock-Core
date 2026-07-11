//! Vulnerability detectors for Soroban smart contracts.

pub mod addr_param_no_auth;
pub mod address_cmp_instead_of_auth;
pub mod address_from_str;
pub mod address_str_eq;
pub mod admin;
pub mod admin_eq_instead_of_auth;
pub mod admin_in_temp;
pub mod admin_key_removal;
pub mod admin_no_event;
pub mod admin_no_group_auth;
pub mod admin_no_remove;
pub mod admin_overwrite;
pub mod admin_stored_unused;
pub mod admin_zero_address;
pub mod alloc_in_loop;
pub mod allowance_clear;
pub mod amount_mul_overflow;
pub mod amount_type;
pub mod assert_for_auth;
pub mod auth;
pub mod auth_after_write;
pub mod auth_in_branch;
pub mod auth_loop_dos;
pub mod auth_on_literal_addr;
pub mod auth_shadow;
pub mod auth_temp_storage;
pub mod auth_untrusted_storage;
pub mod authorize_as_contract;
pub mod authorize_empty;
pub mod balance_mul_overflow;
pub mod balance_negative_check;
pub mod balance_overflow;
pub mod broken_pause;
pub mod bump_after_read;
pub mod bump_to_ttl;
pub mod burn_auth;
pub mod burn_no_event;
pub mod bytes_not_bytesn;
pub mod bytes_oversized;
pub mod catch_unwind;
pub mod commitment_not_cleared;
pub mod contract_addr_in_loop;
pub mod contracterror_attr;
pub mod contracttype;
pub mod crypto_no_cache;
pub mod current_contract_unwrap;
pub mod dead_storage_code;
pub mod debug_entrypoint;
pub mod decimals_mismatch;
pub mod deploy_address_lost;
pub mod deploy_arbitrary_wasm;
pub mod deploy_arg_auth;
pub mod deploy_no_event;
pub mod deploy_salt_predictable;
pub mod deploy_unverified;
pub mod deployer_no_auth;
pub mod deployer_reuse;
pub mod div_before_mul;
pub mod dynamic_symbol_key;
pub mod ed25519_key_in_temp;
pub mod ed25519_unchecked;
pub mod env_in_struct;
pub mod event_before_auth;
pub mod event_before_storage;
pub mod event_data_leak;
pub mod event_duplicate;
pub mod event_full_state;
pub mod event_no_topics;
pub mod event_topic_duplicates;
pub mod event_topic_runtime_string;
pub mod event_topic_user_input;
pub mod event_val_type;
pub mod events;
pub mod events_no_cache;
pub mod expect_leaks;
pub mod expired_deadline;
pub mod extend_ttl_in_loop;
pub mod float_arithmetic;
pub mod hardcoded_address;
pub mod hardcoded_bytes;
pub mod hardcoded_passphrase;
pub mod hash_as_storage_key;
pub mod hash_in_loop;
pub mod host_result_ignored;
pub mod i128_signed_abuse;
pub mod i128_to_u64;
pub mod init_admin_no_auth;
pub mod init_no_event;
pub mod instance_domain_mixing;
pub mod instance_get_guard;
pub mod instance_per_user;
pub mod instance_remove_critical;
pub mod instance_set_no_has;
pub mod instance_ttl;
pub mod instance_vec_growth;
pub mod invalid_address_literal;
pub mod invoke_func_from_input;
pub mod invoke_in_loop;
pub mod invoke_nonexistent_func;
pub mod invoke_result_stored;
pub mod invoke_result_untrusted;
pub mod invoke_return;
pub mod invoke_store_no_event;
pub mod invoke_unchecked_cast;
pub mod keccak_misuse;
pub mod key_length_exceeded;
pub mod key_prefix_collision;
pub mod key_type_mismatch;
pub mod linear_whitelist_scan;
pub mod lock_period_truncation;
pub mod loop_bound_no_cap;
pub mod loop_host_call;
pub mod map_get_unwrap;
pub mod map_key_explosion;
pub mod map_linear_scan;
pub mod map_user_key_bloat;
pub mod migration_guard;
pub mod mint_auth;
pub mod mint_no_cap;
pub mod missing_contract_attr;
pub mod missing_contractimpl;
pub mod missing_ttl;
pub mod mixed_storage_tiers;
pub mod mul_before_div;
pub mod negative_balance;
pub mod negative_deposit;
pub mod nested_loop_storage;
pub mod no_admin;
pub mod no_events_at_all;
pub mod no_param_no_auth;
pub mod no_std;
pub mod nonce_in_temp;
pub mod nonce_increment_order;
pub mod overflow;
pub mod ownership_immediate;
pub mod ownership_no_approval_invalidation;
pub mod ownership_no_event;
pub mod ownership_pending_not_cleared;
pub mod ownership_transfer;
pub mod panic_raw_int;
pub mod panic_usage;
pub mod partial_write_on_error;
pub mod persistent_for_temp;
pub mod persistent_no_extend;
pub mod persistent_overwrite;
pub mod persistent_set_in_loop;
pub mod recursion_no_depth;
pub mod redundant_auth_args;
pub mod reentrancy;
pub mod remove_without_has;
pub mod renounce_no_backup;
pub mod reserved_fn_name;
pub mod result_err_ignored;
pub mod result_non_exhaustive;
pub mod runtime_symbol;
pub mod secp256k1_unchecked;
pub mod self_invoke;
pub mod self_transfer;
pub mod sequence_as_key;
pub mod sequence_nonce;
pub mod sha256_empty;
pub mod sig_verify_inverted;
pub mod storage;
pub mod storage_has_get_mismatch;
pub mod storage_has_get_race;
pub mod storage_key_collision;
pub mod storage_no_cache;
pub mod storage_type_confusion;
pub mod storage_type_version;
pub mod storage_unwrap;
pub mod supply_cap;
pub mod symbol_as_user_key;
pub mod symbol_short_len;
pub mod temp_for_persistent_data;
pub mod temp_get_no_has;
pub mod temp_read_in_view;
pub mod temp_set_no_ttl;
pub mod tier_key_collision;
pub mod timestamp_expiry_no_min;
pub mod timestamp_truncation;
pub mod token_burn_auth;
pub mod token_shared_storage;
pub mod token_transfer_unchecked;
pub mod transfer_to_self;
pub mod transfer_wrong_auth;
pub mod try_into_unwrap;
pub mod ttl_arg_order;
pub mod ttl_before_write;
pub mod ttl_every_call;
pub mod ttl_hardcoded_low;
pub mod ttl_min_zero;
pub mod ttl_no_set;
pub mod ttl_uniform;
pub mod u32_timestamp;
pub mod unauth_address_in_struct;
pub mod unauth_address_tuple;
pub mod unauth_fee_setter;
pub mod unauth_sensitive_read;
pub mod unauth_storage_remove;
pub mod unauthorized_storage_read;
pub mod unbounded_batch;
pub mod unbounded_input_storage;
pub mod unbounded_storage;
pub mod uncapped_fee;
pub mod uncapped_slippage;
pub mod unintended_public_method;
pub mod unlimited_allowance;
pub mod unvalidated_invoke_target;
pub mod unvalidated_price;
pub mod unwrap_or_default_addr;
pub mod upgrade_auth;
pub mod upgrade_no_event;
pub mod upload_wasm_auth;
mod util;
pub mod val_unchecked_convert;
pub mod vec_get_unwrap;
pub mod vec_iter_collect;
pub mod vec_map_tuple_convert;
pub mod vec_mutate_in_loop;
pub mod vec_pop_unwrap;
pub mod vec_push_in_loop;
pub mod vec_slice_unchecked;
pub mod vesting_cliff;
pub mod weak_commitment;
pub mod weak_commitment_known;
pub mod weak_randomness;
pub mod while_host_condition;
pub mod withdraw_auth;
pub mod wrapping_balance_op;
pub mod zero_amount;
pub mod zero_divisor;
pub mod zero_transfer_event;

pub use addr_param_no_auth::AddrParamNoAuthCheck;
pub use address_cmp_instead_of_auth::AddressCmpInsteadOfAuthCheck;
pub use address_from_str::AddressFromStrCheck;
pub use address_str_eq::AddressStrEqCheck;
pub use admin::UnprotectedAdminCheck;
pub use admin_eq_instead_of_auth::AdminEqInsteadOfAuthCheck;
pub use admin_in_temp::AdminInTempCheck;
pub use admin_key_removal::AdminKeyRemovalCheck;
pub use admin_no_event::AdminNoEventCheck;
pub use admin_no_group_auth::AdminNoGroupAuthCheck;
pub use admin_no_remove::AdminNoRemoveCheck;
pub use admin_overwrite::AdminOverwriteCheck;
pub use admin_stored_unused::AdminStoredUnusedCheck;
pub use admin_zero_address::AdminZeroAddressCheck;
pub use alloc_in_loop::AllocInLoopCheck;
pub use allowance_clear::AllowanceClearCheck;
pub use amount_mul_overflow::AmountMulOverflowCheck;
pub use amount_type::AmountTypeCheck;
pub use assert_for_auth::AssertForAuthCheck;
pub use auth::MissingRequireAuthCheck;
pub use auth_after_write::AuthAfterWriteCheck;
pub use auth_in_branch::AuthInBranchCheck;
pub use auth_loop_dos::AuthLoopDosCheck;
pub use auth_on_literal_addr::AuthOnLiteralAddrCheck;
pub use auth_shadow::AuthShadowCheck;
pub use auth_temp_storage::AuthTempStorageCheck;
pub use auth_untrusted_storage::AuthUntrustedStorageCheck;
pub use authorize_as_contract::AuthorizeAsContractCheck;
pub use authorize_empty::AuthorizeEmptyCheck;
pub use balance_mul_overflow::BalanceMulOverflowCheck;
pub use balance_negative_check::BalanceNegativeCheck;
pub use balance_overflow::BalanceOverflowCheck;
pub use broken_pause::BrokenPauseCheck;
pub use bump_after_read::BumpAfterReadCheck;
pub use bump_to_ttl::BumpToTtlCheck;
pub use burn_auth::BurnAuthCheck;
pub use burn_no_event::BurnNoEventCheck;
pub use bytes_not_bytesn::BytesNotBytesNCheck;
pub use bytes_oversized::BytesOversizedCheck;
pub use catch_unwind::CatchUnwindCheck;
pub use commitment_not_cleared::CommitmentNotClearedCheck;
pub use contract_addr_in_loop::ContractAddrInLoopCheck;
pub use contracterror_attr::ContracterrorAttrCheck;
pub use contracttype::MissingContracttypeCheck;
pub use crypto_no_cache::CryptoNoCacheCheck;
pub use current_contract_unwrap::CurrentContractUnwrapCheck;
pub use dead_storage_code::DeadStorageCodeCheck;
pub use debug_entrypoint::DebugEntrypointCheck;
pub use decimals_mismatch::DecimalsMismatchCheck;
pub use deploy_address_lost::DeployAddressLostCheck;
pub use deploy_arbitrary_wasm::DeployArbitraryWasmCheck;
pub use deploy_arg_auth::DeployArgAuthCheck;
pub use deploy_no_event::DeployNoEventCheck;
pub use deploy_salt_predictable::DeploySaltPredictableCheck;
pub use deploy_unverified::DeployUnverifiedCheck;
pub use deployer_no_auth::DeployerNoAuthCheck;
pub use deployer_reuse::DeployerReuseCheck;
pub use div_before_mul::DivBeforeMulCheck;
pub use dynamic_symbol_key::DynamicSymbolKeyCheck;
pub use ed25519_key_in_temp::Ed25519KeyInTempCheck;
pub use ed25519_unchecked::Ed25519UncheckedCheck;
pub use env_in_struct::EnvInStructCheck;
pub use event_before_auth::EventBeforeAuthCheck;
pub use event_before_storage::EventBeforeStorageCheck;
pub use event_data_leak::EventDataLeakCheck;
pub use event_duplicate::EventDuplicateCheck;
pub use event_full_state::EventFullStateCheck;
pub use event_no_topics::EventNoTopicsCheck;
pub use event_topic_duplicates::EventTopicDuplicatesCheck;
pub use event_topic_runtime_string::EventTopicRuntimeStringCheck;
pub use event_topic_user_input::EventTopicUserInputCheck;
pub use event_val_type::EventValTypeCheck;
pub use events::MissingEventsCheck;
pub use events_no_cache::EventsNoCacheCheck;
pub use expect_leaks::ExpectLeaksCheck;
pub use expired_deadline::ExpiredDeadlineCheck;
pub use extend_ttl_in_loop::ExtendTtlInLoopCheck;
pub use float_arithmetic::FloatArithmeticCheck;
pub use hardcoded_address::HardcodedAddressCheck;
pub use hardcoded_bytes::HardcodedBytesCheck;
pub use hardcoded_passphrase::HardcodedPassphraseCheck;
pub use hash_as_storage_key::HashAsStorageKeyCheck;
pub use hash_in_loop::HashInLoopCheck;
pub use host_result_ignored::HostResultIgnoredCheck;
pub use i128_signed_abuse::I128SignedAbuseCheck;
pub use i128_to_u64::I128ToU64Check;
pub use init_admin_no_auth::InitAdminNoAuthCheck;
pub use init_no_event::InitNoEventCheck;
pub use instance_domain_mixing::InstanceDomainMixingCheck;
pub use instance_get_guard::InstanceGetGuardCheck;
pub use instance_per_user::InstancePerUserCheck;
pub use instance_remove_critical::InstanceRemoveCriticalCheck;
pub use instance_set_no_has::InstanceSetNoHasCheck;
pub use instance_ttl::InstanceTtlCheck;
pub use instance_vec_growth::InstanceVecGrowthCheck;
pub use invalid_address_literal::InvalidAddressLiteralCheck;
pub use invoke_func_from_input::InvokeFuncFromInputCheck;
pub use invoke_in_loop::InvokeInLoopCheck;
pub use invoke_nonexistent_func::InvokeNonexistentFuncCheck;
pub use invoke_result_stored::InvokeResultStoredCheck;
pub use invoke_result_untrusted::InvokeResultUntrustedCheck;
pub use invoke_return::InvokeReturnCheck;
pub use invoke_store_no_event::InvokeStoreNoEventCheck;
pub use invoke_unchecked_cast::InvokeUncheckedCastCheck;
pub use keccak_misuse::KeccakMisuseCheck;
pub use key_length_exceeded::KeyLengthExceededCheck;
pub use key_prefix_collision::KeyPrefixCollisionCheck;
pub use key_type_mismatch::KeyTypeMismatchCheck;
pub use linear_whitelist_scan::LinearWhitelistScanCheck;
pub use lock_period_truncation::LockPeriodTruncationCheck;
pub use loop_bound_no_cap::LoopBoundNoCapCheck;
pub use loop_host_call::LoopHostCallCheck;
pub use map_get_unwrap::MapGetUnwrapCheck;
pub use map_key_explosion::MapKeyExplosionCheck;
pub use map_linear_scan::MapLinearScanCheck;
pub use map_user_key_bloat::MapUserKeyBloatCheck;
pub use migration_guard::MigrationGuardCheck;
pub use mint_auth::MintAuthCheck;
pub use mint_no_cap::MintNoCapCheck;
pub use missing_contract_attr::MissingContractAttrCheck;
pub use missing_contractimpl::MissingContractimplCheck;
pub use missing_ttl::MissingTtlExtensionCheck;
pub use mixed_storage_tiers::MixedStorageTiersCheck;
pub use mul_before_div::MulBeforeDivCheck;
pub use negative_balance::NegativeBalanceCheck;
pub use negative_deposit::NegativeDepositCheck;
pub use nested_loop_storage::NestedLoopStorageCheck;
pub use no_admin::NoAdminCheck;
pub use no_events_at_all::NoEventsAtAllCheck;
pub use no_param_no_auth::NoParamNoAuthCheck;
pub use no_std::NoStdCheck;
pub use nonce_in_temp::NonceInTempCheck;
pub use nonce_increment_order::NonceIncrementOrderCheck;
pub use overflow::UncheckedArithmeticCheck;
pub use ownership_immediate::OwnershipImmediateCheck;
pub use ownership_no_approval_invalidation::OwnershipNoApprovalInvalidationCheck;
pub use ownership_no_event::OwnershipNoEventCheck;
pub use ownership_pending_not_cleared::OwnershipPendingNotClearedCheck;
pub use ownership_transfer::OwnershipTransferCheck;
pub use panic_raw_int::PanicRawIntCheck;
pub use panic_usage::PanicUsageCheck;
pub use partial_write_on_error::PartialWriteOnErrorCheck;
pub use persistent_for_temp::PersistentForTempCheck;
pub use persistent_no_extend::PersistentNoExtendCheck;
pub use persistent_overwrite::PersistentOverwriteCheck;
pub use persistent_set_in_loop::PersistentSetInLoopCheck;
pub use recursion_no_depth::RecursionNoDepthCheck;
pub use redundant_auth_args::RedundantAuthArgsCheck;
pub use reentrancy::ReentrancyCheck;
pub use remove_without_has::RemoveWithoutHasCheck;
pub use renounce_no_backup::RenounceNoBackupCheck;
pub use reserved_fn_name::ReservedFnNameCheck;
pub use result_err_ignored::ResultErrIgnoredCheck;
pub use result_non_exhaustive::ResultNonExhaustiveCheck;
pub use runtime_symbol::RuntimeSymbolCheck;
pub use secp256k1_unchecked::Secp256k1UncheckedCheck;
pub use self_invoke::SelfInvokeCheck;
pub use self_transfer::SelfTransferCheck;
pub use sequence_as_key::SequenceAsKeyCheck;
pub use sequence_nonce::SequenceNonceCheck;
pub use sha256_empty::Sha256EmptyCheck;
pub use sig_verify_inverted::SigVerifyInvertedCheck;
pub use storage::UnsafeStoragePatternsCheck;
pub use storage_has_get_mismatch::StorageHasGetMismatchCheck;
pub use storage_has_get_race::StorageHasGetRaceCheck;
pub use storage_key_collision::StorageKeyCollisionCheck;
pub use storage_no_cache::StorageNoCacheCheck;
pub use storage_type_confusion::StorageTypeConfusionCheck;
pub use storage_type_version::StorageTypeVersionCheck;
pub use storage_unwrap::StorageUnwrapCheck;
pub use supply_cap::SupplyCapCheck;
pub use symbol_as_user_key::SymbolAsUserKeyCheck;
pub use symbol_short_len::SymbolShortLenCheck;
pub use temp_for_persistent_data::TempForPersistentDataCheck;
pub use temp_get_no_has::TempGetNoHasCheck;
pub use temp_read_in_view::TempReadInViewCheck;
pub use temp_set_no_ttl::TempSetNoTtlCheck;
pub use tier_key_collision::TierKeyCollisionCheck;
pub use timestamp_expiry_no_min::TimestampExpiryNoMinCheck;
pub use timestamp_truncation::TimestampTruncationCheck;
pub use token_burn_auth::TokenBurnAuthCheck;
pub use token_shared_storage::TokenSharedStorageCheck;
pub use token_transfer_unchecked::TokenTransferUncheckedCheck;
pub use transfer_to_self::TransferToSelfCheck;
pub use transfer_wrong_auth::TransferWrongAuthCheck;
pub use try_into_unwrap::TryIntoUnwrapCheck;
pub use ttl_arg_order::TtlArgOrderCheck;
pub use ttl_before_write::TtlBeforeWriteCheck;
pub use ttl_every_call::TtlEveryCallCheck;
pub use ttl_hardcoded_low::TtlHardcodedLowCheck;
pub use ttl_min_zero::TtlMinZeroCheck;
pub use ttl_no_set::TtlNoSetCheck;
pub use ttl_uniform::TtlUniformCheck;
pub use u32_timestamp::U32TimestampCheck;
pub use unauth_address_in_struct::UnauthAddressInStructCheck;
pub use unauth_address_tuple::UnauthAddressTupleCheck;
pub use unauth_fee_setter::UnauthFeeSetterCheck;
pub use unauth_sensitive_read::UnauthSensitiveReadCheck;
pub use unauth_storage_remove::UnauthStorageRemoveCheck;
pub use unauthorized_storage_read::UnauthorizedStorageReadCheck;
pub use unbounded_batch::UnboundedBatchCheck;
pub use unbounded_input_storage::UnboundedInputStorageCheck;
pub use unbounded_storage::UnboundedStorageCheck;
pub use uncapped_fee::UncappedFeeCheck;
pub use uncapped_slippage::UncappedSlippageCheck;
pub use unintended_public_method::UnintendedPublicMethodCheck;
pub use unlimited_allowance::UnlimitedAllowanceCheck;
pub use unvalidated_invoke_target::UnvalidatedInvokeTargetCheck;
pub use unvalidated_price::UnvalidatedPriceCheck;
pub use unwrap_or_default_addr::UnwrapOrDefaultAddrCheck;
pub use upgrade_auth::UpgradeAuthCheck;
pub use upgrade_no_event::UpgradeNoEventCheck;
pub use upload_wasm_auth::UploadWasmAuthCheck;
pub use val_unchecked_convert::ValUncheckedConvertCheck;
pub use vec_get_unwrap::VecGetUnwrapCheck;
pub use vec_iter_collect::VecIterCollectCheck;
pub use vec_map_tuple_convert::VecMapTupleConvertCheck;
pub use vec_mutate_in_loop::VecMutateInLoopCheck;
pub use vec_pop_unwrap::VecPopUnwrapCheck;
pub use vec_push_in_loop::VecPushInLoopCheck;
pub use vec_slice_unchecked::VecSliceUncheckedCheck;
pub use vesting_cliff::VestingCliffCheck;
pub use weak_commitment::WeakCommitmentCheck;
pub use weak_commitment_known::WeakCommitmentKnownCheck;
pub use weak_randomness::WeakRandomnessCheck;
pub use while_host_condition::WhileHostConditionCheck;
pub use withdraw_auth::WithdrawAuthCheck;
pub use wrapping_balance_op::WrappingBalanceOpCheck;
pub use zero_amount::ZeroAmountCheck;
pub use zero_divisor::ZeroDivisorCheck;
pub use zero_transfer_event::ZeroTransferEventCheck;

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
        Box::new(NoAdminCheck),
        Box::new(UnsafeStoragePatternsCheck),
        Box::new(InstanceDomainMixingCheck),
        Box::new(PanicUsageCheck),
        Box::new(CatchUnwindCheck),
        Box::new(PartialWriteOnErrorCheck),
        Box::new(MissingContracttypeCheck),
        Box::new(UnboundedStorageCheck),
        Box::new(UnboundedInputStorageCheck),
        Box::new(ZeroAmountCheck),
        Box::new(ZeroDivisorCheck),
        Box::new(SelfTransferCheck),
        Box::new(SequenceAsKeyCheck),
        Box::new(NoStdCheck),
        Box::new(EnvInStructCheck),
        Box::new(TempGetNoHasCheck),
        Box::new(AmountMulOverflowCheck),
        Box::new(FloatArithmeticCheck),
        Box::new(Sha256EmptyCheck),
        Box::new(Ed25519KeyInTempCheck),
        Box::new(WeakRandomnessCheck),
        Box::new(WeakCommitmentKnownCheck),
        Box::new(ReentrancyCheck),
        Box::new(ResultErrIgnoredCheck),
        Box::new(TokenTransferUncheckedCheck),
        Box::new(ContracterrorAttrCheck),
        Box::new(TokenBurnAuthCheck),
        Box::new(MintAuthCheck),
        Box::new(MintNoCapCheck),
        Box::new(BurnNoEventCheck),
        Box::new(SequenceNonceCheck),
        Box::new(AssertForAuthCheck),
        Box::new(AuthorizeAsContractCheck),
        Box::new(AuthorizeEmptyCheck),
        Box::new(AddressCmpInsteadOfAuthCheck),
        Box::new(DeployArgAuthCheck),
        Box::new(DeployerReuseCheck),
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
        Box::new(StorageKeyCollisionCheck),
        Box::new(UnauthAddressInStructCheck),
        Box::new(InvokeUncheckedCastCheck),
        Box::new(InvokeFuncFromInputCheck),
        Box::new(InvokeResultUntrustedCheck),
        Box::new(DeploySaltPredictableCheck),
        Box::new(DeployUnverifiedCheck),
        Box::new(NegativeDepositCheck),
        Box::new(NoParamNoAuthCheck),
        Box::new(StorageTypeConfusionCheck),
        Box::new(ResultNonExhaustiveCheck),
        Box::new(AuthLoopDosCheck),
        Box::new(OwnershipTransferCheck),
        Box::new(Secp256k1UncheckedCheck),
        Box::new(TempSetNoTtlCheck),
        Box::new(BalanceOverflowCheck),
        Box::new(WrappingBalanceOpCheck),
        Box::new(UnauthSensitiveReadCheck),
        Box::new(InstanceRemoveCriticalCheck),
        Box::new(MapGetUnwrapCheck),
        Box::new(MapUserKeyBloatCheck),
        Box::new(TimestampTruncationCheck),
        Box::new(UnlimitedAllowanceCheck),
        Box::new(AllowanceClearCheck),
        Box::new(LockPeriodTruncationCheck),
        Box::new(LinearWhitelistScanCheck),
        Box::new(UncappedSlippageCheck),
        Box::new(NonceIncrementOrderCheck),
        Box::new(NonceInTempCheck),
        Box::new(BalanceNegativeCheck),
        Box::new(MulBeforeDivCheck),
        Box::new(TokenSharedStorageCheck),
        Box::new(AdminNoEventCheck),
        Box::new(UnauthorizedStorageReadCheck),
        Box::new(TierKeyCollisionCheck),
        Box::new(StorageTypeVersionCheck),
        Box::new(AdminZeroAddressCheck),
        Box::new(AdminNoGroupAuthCheck),
        Box::new(AdminNoRemoveCheck),
        Box::new(AdminStoredUnusedCheck),
        Box::new(AdminEqInsteadOfAuthCheck),
        Box::new(VecMapTupleConvertCheck),
        Box::new(OwnershipPendingNotClearedCheck),
        Box::new(OwnershipNoApprovalInvalidationCheck),
        Box::new(TryIntoUnwrapCheck),
        Box::new(AddrParamNoAuthCheck),
        Box::new(AddressFromStrCheck),
        Box::new(AddressStrEqCheck),
        Box::new(AdminInTempCheck),
        Box::new(AllocInLoopCheck),
        Box::new(AmountTypeCheck),
        Box::new(AuthAfterWriteCheck),
        Box::new(AuthInBranchCheck),
        Box::new(AuthOnLiteralAddrCheck),
        Box::new(AuthShadowCheck),
        Box::new(AuthUntrustedStorageCheck),
        Box::new(BalanceMulOverflowCheck),
        Box::new(BumpAfterReadCheck),
        Box::new(BumpToTtlCheck),
        Box::new(BurnAuthCheck),
        Box::new(BytesOversizedCheck),
        Box::new(CommitmentNotClearedCheck),
        Box::new(ContractAddrInLoopCheck),
        Box::new(CryptoNoCacheCheck),
        Box::new(CurrentContractUnwrapCheck),
        Box::new(DeadStorageCodeCheck),
        Box::new(DecimalsMismatchCheck),
        Box::new(DeployAddressLostCheck),
        Box::new(DeployArbitraryWasmCheck),
        Box::new(DeployNoEventCheck),
        Box::new(DeployerNoAuthCheck),
        Box::new(DivBeforeMulCheck),
        Box::new(Ed25519UncheckedCheck),
        Box::new(EventBeforeAuthCheck),
        Box::new(EventBeforeStorageCheck),
        Box::new(EventDataLeakCheck),
        Box::new(EventDuplicateCheck),
        Box::new(EventFullStateCheck),
        Box::new(EventNoTopicsCheck),
        Box::new(EventTopicDuplicatesCheck),
        Box::new(EventTopicRuntimeStringCheck),
        Box::new(EventTopicUserInputCheck),
        Box::new(EventValTypeCheck),
        Box::new(MissingEventsCheck),
        Box::new(EventsNoCacheCheck),
        Box::new(ExpectLeaksCheck),
        Box::new(ExpiredDeadlineCheck),
        Box::new(HardcodedAddressCheck),
        Box::new(HardcodedBytesCheck),
        Box::new(HardcodedPassphraseCheck),
        Box::new(HashInLoopCheck),
        Box::new(HostResultIgnoredCheck),
        Box::new(I128SignedAbuseCheck),
        Box::new(I128ToU64Check),
        Box::new(InitAdminNoAuthCheck),
        Box::new(InitNoEventCheck),
        Box::new(InstanceGetGuardCheck),
        Box::new(InstancePerUserCheck),
        Box::new(InstanceSetNoHasCheck),
        Box::new(InstanceTtlCheck),
        Box::new(InvalidAddressLiteralCheck),
        Box::new(InvokeInLoopCheck),
        Box::new(InvokeNonexistentFuncCheck),
        Box::new(InvokeResultStoredCheck),
        Box::new(InvokeReturnCheck),
        Box::new(InvokeStoreNoEventCheck),
        Box::new(KeccakMisuseCheck),
        Box::new(KeyLengthExceededCheck),
        Box::new(KeyPrefixCollisionCheck),
        Box::new(KeyTypeMismatchCheck),
        Box::new(LoopBoundNoCapCheck),
        Box::new(LoopHostCallCheck),
        Box::new(MapLinearScanCheck),
        Box::new(MissingContractAttrCheck),
        Box::new(MissingContractimplCheck),
        Box::new(MissingTtlExtensionCheck),
        Box::new(MixedStorageTiersCheck),
        Box::new(NegativeBalanceCheck),
        Box::new(NestedLoopStorageCheck),
        Box::new(NoEventsAtAllCheck),
        Box::new(OwnershipImmediateCheck),
        Box::new(OwnershipNoEventCheck),
        Box::new(PanicRawIntCheck),
        Box::new(PersistentForTempCheck),
        Box::new(PersistentNoExtendCheck),
        Box::new(PersistentOverwriteCheck),
        Box::new(PersistentSetInLoopCheck),
        Box::new(RecursionNoDepthCheck),
        Box::new(RedundantAuthArgsCheck),
        Box::new(RemoveWithoutHasCheck),
        Box::new(RenounceNoBackupCheck),
        Box::new(ReservedFnNameCheck),
        Box::new(RuntimeSymbolCheck),
        Box::new(SelfInvokeCheck),
        Box::new(SigVerifyInvertedCheck),
        Box::new(StorageHasGetMismatchCheck),
        Box::new(StorageHasGetRaceCheck),
        Box::new(StorageNoCacheCheck),
        Box::new(StorageUnwrapCheck),
        Box::new(SupplyCapCheck),
        Box::new(SymbolAsUserKeyCheck),
        Box::new(SymbolShortLenCheck),
        Box::new(TempForPersistentDataCheck),
        Box::new(TempReadInViewCheck),
        Box::new(TimestampExpiryNoMinCheck),
        Box::new(TransferToSelfCheck),
        Box::new(TransferWrongAuthCheck),
        Box::new(TtlArgOrderCheck),
        Box::new(TtlBeforeWriteCheck),
        Box::new(TtlEveryCallCheck),
        Box::new(TtlHardcodedLowCheck),
        Box::new(TtlMinZeroCheck),
        Box::new(TtlNoSetCheck),
        Box::new(TtlUniformCheck),
        Box::new(U32TimestampCheck),
        Box::new(UnauthAddressTupleCheck),
        Box::new(UnauthFeeSetterCheck),
        Box::new(UnauthStorageRemoveCheck),
        Box::new(UnboundedBatchCheck),
        Box::new(UncappedFeeCheck),
        Box::new(UnvalidatedInvokeTargetCheck),
        Box::new(UnintendedPublicMethodCheck),
        Box::new(UnvalidatedPriceCheck),
        Box::new(UnwrapOrDefaultAddrCheck),
        Box::new(UpgradeAuthCheck),
        Box::new(UpgradeNoEventCheck),
        Box::new(UploadWasmAuthCheck),
        Box::new(ValUncheckedConvertCheck),
        Box::new(VecGetUnwrapCheck),
        Box::new(VecIterCollectCheck),
        Box::new(VecMutateInLoopCheck),
        Box::new(VecPopUnwrapCheck),
        Box::new(VecPushInLoopCheck),
        Box::new(VecSliceUncheckedCheck),
        Box::new(VestingCliffCheck),
        Box::new(WeakCommitmentCheck),
        Box::new(WhileHostConditionCheck),
        Box::new(ZeroTransferEventCheck),
    ]
}

#[cfg(test)]
mod registration_tests {
    use std::collections::HashSet;

    /// Every check re-exported via `pub use x::Y;` at the top of this file must also
    /// appear in `default_checks()`. This guards against the exact regression that let
    /// dozens of implemented checks silently never run: a check module gets written,
    /// tested, and `pub use`d, but the `Box::new(...)` entry in `default_checks()` is
    /// forgotten.
    #[test]
    fn all_exported_checks_are_registered() {
        let src = include_str!("lib.rs");

        let mut exported = HashSet::new();
        for line in src.lines() {
            let line = line.trim();
            if let Some(rest) = line.strip_prefix("pub use ") {
                if let Some(idx) = rest.rfind("::") {
                    exported.insert(rest[idx + 2..].trim_end_matches(';').to_string());
                }
            }
        }

        let default_checks_src = src
            .split_once("pub fn default_checks()")
            .expect("default_checks() not found in lib.rs")
            .1;
        let mut registered = HashSet::new();
        for chunk in default_checks_src.split("Box::new(").skip(1) {
            if let Some(end) = chunk.find(')') {
                registered.insert(chunk[..end].to_string());
            }
        }

        let missing: Vec<&String> = exported.difference(&registered).collect();
        assert!(
            missing.is_empty(),
            "checks are exported via `pub use` but missing from default_checks(): {missing:?}"
        );
    }
}
