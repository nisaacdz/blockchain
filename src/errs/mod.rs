#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustomErrs {
    InvalidPublicKey,
    InvalidPrivateKey,
    InvalidBlock,
    VerificationDoesNotMatch,
    Default,
    CouldNotInsertRecordsIntoDatabase,
    CouldNotInsertHashIntoDatabase,
    CannotEstablishDatabaseConnection,
    NoSuchTableInDatabase,
    InvalidSignature,
    EmptyBlocksNotAllowed,
    NoDatabaseConnected,
    CannotCreateSuchTable,
}
