pub mod ordered;
pub mod unordered;

pub use ordered::*;
pub use unordered::*;

crate::new_ordered_identity!(UserId, "user_");
crate::new_ordered_identity!(CredentialId, "credential_");
crate::new_ordered_identity!(UserCredentialId, "user_credential_");
crate::new_ordered_identity!(UserCredentialPasswordId, "user_credential_password_");

crate::new_unordered_identity!(VerificationToken, "verification_token_");

impl CredentialId {
    pub const LOCAL: Self = Self::from_u128(0x019170b0_609a_79fe_9314_044d14863643);
    pub const OTP: Self = Self::from_u128(0x01918d67_5721_7c47_b66e_39be161e6322);
    pub const EMAIL_VERIFICATION: Self = Self::from_u128(0x0191adb4_e387_75c7_8082_d33779e6b8c3);
}
