use hyper::StatusCode;
use std::fmt::{self, Display};

/// See <https://docs.aws.amazon.com/AmazonS3/latest/API/ErrorResponses.html>
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub(super) enum S3ErrorCode {
    AccessDenied,
    AccountProblem,
    AllAccessDisabled,
    AmbiguousGrantByEmailAddress,
    AuthorizationHeaderMalformed,
    BadDigest,
    BucketAlreadyExists,
    BucketAlreadyOwnedByYou,
    BucketNotEmpty,
    CredentialsNotSupported,
    CrossLocationLoggingProhibited,
    EntityTooSmall,
    EntityTooLarge,
    ExpiredToken,
    IllegalLocationConstraintException,
    IllegalVersioningConfigurationException,
    IncompleteBody,
    IncorrectNumberOfFilesInPostRequest,
    InlineDataTooLarge,
    InternalError,
    InvalidAccessKeyId,
    InvalidAddressingHeader,
    InvalidArgument,
    InvalidBucketName,
    InvalidBucketState,
    InvalidDigest,
    InvalidEncryptionAlgorithmError,
    InvalidLocationConstraint,
    InvalidObjectState,
    InvalidPart,
    InvalidPartOrder,
    InvalidPayer,
    InvalidPolicyDocument,
    InvalidRange,
    InvalidRequest,
    InvalidSecurity,
    InvalidSOAPRequest,
    InvalidStorageClass,
    InvalidTargetBucketForLogging,
    InvalidToken,
    InvalidURI,
    KeyTooLongError,
    MalformedACLError,
    MalformedPOSTRequest,
    MalformedXML,
    MaxMessageLengthExceeded,
    MaxPostPreDataLengthExceededError,
    MetadataTooLarge,
    MethodNotAllowed,
    MissingAttachment,
    MissingContentLength,
    MissingRequestBodyError,
    MissingSecurityElement,
    MissingSecurityHeader,
    NoLoggingStatusForKey,
    NoSuchBucket,
    NoSuchBucketPolicy,
    NoSuchKey,
    NoSuchLifecycleConfiguration,
    NoSuchUpload,
    NoSuchVersion,
    NotImplemented,
    NotSignedUp,
    OperationAborted,
    PermanentRedirect,
    PreconditionFailed,
    Redirect,
    RestoreAlreadyInProgress,
    RequestIsNotMultiPartContent,
    RequestTimeout,
    RequestTimeTooSkewed,
    RequestTorrentOfBucketError,
    ServerSideEncryptionConfigurationNotFoundError,
    ServiceUnavailable,
    SignatureDoesNotMatch,
    SlowDown,
    TemporaryRedirect,
    TokenRefreshRequired,
    TooManyBuckets,
    UnexpectedContent,
    UnresolvableGrantByEmailAddress,
    UserKeyMustBeSpecified,
}

impl Display for S3ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::Debug>::fmt(self, f)
    }
}
impl S3ErrorCode {
    #[allow(dead_code)] // TODO: remove it
    #[allow(clippy::match_same_arms)]
    pub(super) fn as_status_code(self) -> Option<StatusCode> {
        match self {
            Self::AccessDenied => Some(StatusCode::FORBIDDEN),
            Self::AccountProblem => Some(StatusCode::FORBIDDEN),
            Self::AllAccessDisabled => Some(StatusCode::FORBIDDEN),
            Self::AmbiguousGrantByEmailAddress => Some(StatusCode::BAD_REQUEST),
            Self::AuthorizationHeaderMalformed => Some(StatusCode::BAD_REQUEST),
            Self::BadDigest => Some(StatusCode::BAD_REQUEST),
            Self::BucketAlreadyExists => Some(StatusCode::CONFLICT),
            Self::BucketAlreadyOwnedByYou => Some(StatusCode::CONFLICT),
            Self::BucketNotEmpty => Some(StatusCode::CONFLICT),
            Self::CredentialsNotSupported => Some(StatusCode::BAD_REQUEST),
            Self::CrossLocationLoggingProhibited => Some(StatusCode::FORBIDDEN),
            Self::EntityTooSmall => Some(StatusCode::BAD_REQUEST),
            Self::EntityTooLarge => Some(StatusCode::BAD_REQUEST),
            Self::ExpiredToken => Some(StatusCode::BAD_REQUEST),
            Self::IllegalLocationConstraintException => Some(StatusCode::BAD_REQUEST),
            Self::IllegalVersioningConfigurationException => Some(StatusCode::BAD_REQUEST),
            Self::IncompleteBody => Some(StatusCode::BAD_REQUEST),
            Self::IncorrectNumberOfFilesInPostRequest => Some(StatusCode::BAD_REQUEST),
            Self::InlineDataTooLarge => Some(StatusCode::BAD_REQUEST),
            Self::InternalError => Some(StatusCode::INTERNAL_SERVER_ERROR),
            Self::InvalidAccessKeyId => Some(StatusCode::FORBIDDEN),
            Self::InvalidAddressingHeader => None,
            Self::InvalidArgument => Some(StatusCode::BAD_REQUEST),
            Self::InvalidBucketName => Some(StatusCode::BAD_REQUEST),
            Self::InvalidBucketState => Some(StatusCode::CONFLICT),
            Self::InvalidDigest => Some(StatusCode::BAD_REQUEST),
            Self::InvalidEncryptionAlgorithmError => Some(StatusCode::BAD_REQUEST),
            Self::InvalidLocationConstraint => Some(StatusCode::BAD_REQUEST),
            Self::InvalidObjectState => Some(StatusCode::FORBIDDEN),
            Self::InvalidPart => Some(StatusCode::BAD_REQUEST),
            Self::InvalidPartOrder => Some(StatusCode::BAD_REQUEST),
            Self::InvalidPayer => Some(StatusCode::FORBIDDEN),
            Self::InvalidPolicyDocument => Some(StatusCode::BAD_REQUEST),
            Self::InvalidRange => Some(StatusCode::RANGE_NOT_SATISFIABLE),
            Self::InvalidRequest => Some(StatusCode::BAD_REQUEST),
            Self::InvalidSecurity => Some(StatusCode::FORBIDDEN),
            Self::InvalidSOAPRequest => Some(StatusCode::BAD_REQUEST),
            Self::InvalidStorageClass => Some(StatusCode::BAD_REQUEST),
            Self::InvalidTargetBucketForLogging => Some(StatusCode::BAD_REQUEST),
            Self::InvalidToken => Some(StatusCode::BAD_REQUEST),
            Self::InvalidURI => Some(StatusCode::BAD_REQUEST),
            Self::KeyTooLongError => Some(StatusCode::BAD_REQUEST),
            Self::MalformedACLError => Some(StatusCode::BAD_REQUEST),
            Self::MalformedPOSTRequest => Some(StatusCode::BAD_REQUEST),
            Self::MalformedXML => Some(StatusCode::BAD_REQUEST),
            Self::MaxMessageLengthExceeded => Some(StatusCode::BAD_REQUEST),
            Self::MaxPostPreDataLengthExceededError => Some(StatusCode::BAD_REQUEST),
            Self::MetadataTooLarge => Some(StatusCode::BAD_REQUEST),
            Self::MethodNotAllowed => Some(StatusCode::METHOD_NOT_ALLOWED),
            Self::MissingAttachment => None,
            Self::MissingContentLength => Some(StatusCode::LENGTH_REQUIRED),
            Self::MissingRequestBodyError => Some(StatusCode::BAD_REQUEST),
            Self::MissingSecurityElement => Some(StatusCode::BAD_REQUEST),
            Self::MissingSecurityHeader => Some(StatusCode::BAD_REQUEST),
            Self::NoLoggingStatusForKey => Some(StatusCode::BAD_REQUEST),
            Self::NoSuchBucket => Some(StatusCode::NOT_FOUND),
            Self::NoSuchBucketPolicy => Some(StatusCode::NOT_FOUND),
            Self::NoSuchKey => Some(StatusCode::NOT_FOUND),
            Self::NoSuchLifecycleConfiguration => Some(StatusCode::NOT_FOUND),
            Self::NoSuchUpload => Some(StatusCode::NOT_FOUND),
            Self::NoSuchVersion => Some(StatusCode::NOT_FOUND),
            Self::NotImplemented => Some(StatusCode::NOT_IMPLEMENTED),
            Self::NotSignedUp => Some(StatusCode::FORBIDDEN),
            Self::OperationAborted => Some(StatusCode::CONFLICT),
            Self::PermanentRedirect => Some(StatusCode::MOVED_PERMANENTLY),
            Self::PreconditionFailed => Some(StatusCode::PRECONDITION_FAILED),
            Self::Redirect => Some(StatusCode::TEMPORARY_REDIRECT),
            Self::RestoreAlreadyInProgress => Some(StatusCode::CONFLICT),
            Self::RequestIsNotMultiPartContent => Some(StatusCode::BAD_REQUEST),
            Self::RequestTimeout => Some(StatusCode::BAD_REQUEST),
            Self::RequestTimeTooSkewed => Some(StatusCode::FORBIDDEN),
            Self::RequestTorrentOfBucketError => Some(StatusCode::BAD_REQUEST),
            Self::ServerSideEncryptionConfigurationNotFoundError => Some(StatusCode::BAD_REQUEST),
            Self::ServiceUnavailable => Some(StatusCode::SERVICE_UNAVAILABLE),
            Self::SignatureDoesNotMatch => Some(StatusCode::FORBIDDEN),
            Self::SlowDown => Some(StatusCode::SERVICE_UNAVAILABLE),
            Self::TemporaryRedirect => Some(StatusCode::TEMPORARY_REDIRECT),
            Self::TokenRefreshRequired => Some(StatusCode::BAD_REQUEST),
            Self::TooManyBuckets => Some(StatusCode::BAD_REQUEST),
            Self::UnexpectedContent => Some(StatusCode::BAD_REQUEST),
            Self::UnresolvableGrantByEmailAddress => Some(StatusCode::BAD_REQUEST),
            Self::UserKeyMustBeSpecified => Some(StatusCode::BAD_REQUEST),
        }
    }
}