#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustomErrs {
    /// Errors From gen
    InvalidPublicKey,
    InvalidPrivateKey,

    /// Erros from blockchain module
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
