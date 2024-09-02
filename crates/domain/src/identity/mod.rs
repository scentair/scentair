pub mod ordered;
pub mod unordered;

pub use ordered::*;
pub use unordered::*;

crate::new_ordered_identity!(UserId, "user_");
crate::new_ordered_identity!(CredentialId, "credential_");
crate::new_ordered_identity!(UserCredentialId, "user_credential_");
crate::new_ordered_identity!(UserCredentialPasswordId, "user_credential_password_");
crate::new_ordered_identity!(VerificationId, "verification_");
crate::new_ordered_identity!(UserVerificationId, "user_verification_");
crate::new_ordered_identity!(SessionId, "session_");

crate::new_unordered_identity!(VerificationToken, "verification_token_");
crate::new_unordered_identity!(SessionToken, "session_token_");

impl CredentialId {
    pub const LOCAL: Self = Self::from_u128(0x019170b0_609a_79fe_9314_044d14863643);
    pub const OTP: Self = Self::from_u128(0x01918d67_5721_7c47_b66e_39be161e6322);
}

impl VerificationId {
    pub const EMAIL_VERIFIED: Self = Self::from_u128(0x019170cb_2286_71b4_9404_8442808614d8);
}
