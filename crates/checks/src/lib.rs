//! Vulnerability detectors for Soroban smart contracts.

pub mod admin;
pub mod admin_in_temp;
pub mod admin_overwrite;
pub mod auth;
pub mod burn_auth;
pub mod mint_auth;
pub mod contracttype;
pub mod float_arithmetic;
pub mod init_no_event;
pub mod missing_ttl;
pub mod no_events_at_all;
pub mod no_std;
pub mod overflow;
pub mod panic_usage;
pub mod reentrancy;
pub mod self_transfer;
pub mod storage;
pub mod ttl_arg_order;
pub mod unbounded_storage;
pub mod weak_randomness;
pub mod token_transfer_unchecked;
pub mod zero_amount;
mod util;

pub use admin::UnprotectedAdminCheck;
pub use admin_in_temp::AdminInTempCheck;
pub use admin_overwrite::AdminOverwriteCheck;
pub use auth::MissingRequireAuthCheck;
pub use burn_auth::BurnAuthCheck;
pub use mint_auth::MintAuthCheck;
pub use contracttype::MissingContracttypeCheck;
pub use float_arithmetic::FloatArithmeticCheck;
pub use init_no_event::InitNoEventCheck;
pub use missing_ttl::MissingTtlExtensionCheck;
pub use no_events_at_all::NoEventsAtAllCheck;
pub use no_std::NoStdCheck;
pub use overflow::UncheckedArithmeticCheck;
pub use panic_usage::PanicUsageCheck;
pub use reentrancy::ReentrancyCheck;
pub use self_transfer::SelfTransferCheck;
pub use storage::UnsafeStoragePatternsCheck;
pub use token_transfer_unchecked::TokenTransferUncheckedCheck;
pub use ttl_arg_order::TtlArgOrderCheck;
pub use unbounded_storage::UnboundedStorageCheck;
pub use weak_randomness::WeakRandomnessCheck;
pub use zero_amount::ZeroAmountCheck;

use serde::Serialize;
use syn::File;

/// Severity of a finding.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
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
        Box::new(UnsafeStoragePatternsCheck),
        Box::new(PanicUsageCheck),
        Box::new(MissingContracttypeCheck),
        Box::new(UnboundedStorageCheck),
        Box::new(ZeroAmountCheck),
        Box::new(SelfTransferCheck),
        Box::new(NoStdCheck),
        Box::new(FloatArithmeticCheck),
        Box::new(WeakRandomnessCheck),
        Box::new(ReentrancyCheck),
        Box::new(TokenTransferUncheckedCheck),
        Box::new(MintAuthCheck),
        Box::new(InitNoEventCheck),
        Box::new(NoEventsAtAllCheck),
    ]
}
