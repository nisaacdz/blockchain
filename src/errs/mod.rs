#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Errs {
    InvalidPublicKey,
    InvalidPrivateKey,
    InvalidBlock,
    VerificationDoesNotMatch,
    Default,
}
