/// MerkleRoot defines a merkle root hash.
/// In the Cosmos SDK, the AppHash of a block header becomes the root.
#[cfg_attr(feature = "std", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(
    all(feature = "json-schema", feature = "std"),
    derive(::schemars::JsonSchema)
)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerkleRoot {
    #[prost(bytes = "vec", tag = "1")]
    #[cfg_attr(feature = "std", serde(with = "crate::base64"))]
    #[cfg_attr(all(feature = "json-schema", feature = "std"), schemars(with = "String"))]
    pub hash: ::prost::alloc::vec::Vec<u8>,
}
/// MerklePrefix is merkle path prefixed to the key.
/// The constructed key from the Path and the key will be append(Path.KeyPath,
/// append(Path.KeyPrefix, key...))
#[cfg_attr(feature = "std", derive(::serde::Serialize, ::serde::Deserialize))]
#[cfg_attr(
    all(feature = "json-schema", feature = "std"),
    derive(::schemars::JsonSchema)
)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerklePrefix {
    #[prost(bytes = "vec", tag = "1")]
    #[cfg_attr(feature = "std", serde(with = "crate::base64"))]
    #[cfg_attr(all(feature = "json-schema", feature = "std"), schemars(with = "String"))]
    pub key_prefix: ::prost::alloc::vec::Vec<u8>,
}
/// MerklePath is the path used to verify commitment proofs, which can be an
/// arbitrary structured object (defined by a commitment type).
/// MerklePath is represented from root-to-leaf
#[cfg_attr(feature = "std", derive(::serde::Serialize, ::serde::Deserialize))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerklePath {
    #[prost(string, repeated, tag = "1")]
    pub key_path: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MerkleProof is a wrapper type over a chain of CommitmentProofs.
/// It demonstrates membership or non-membership for an element or set of
/// elements, verifiable in conjunction with a known commitment root. Proofs
/// should be succinct.
/// MerkleProofs are ordered from leaf-to-root
#[cfg_attr(feature = "std", derive(::serde::Serialize, ::serde::Deserialize))]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerkleProof {
    #[prost(message, repeated, tag = "1")]
    pub proofs: ::prost::alloc::vec::Vec<::ics23::CommitmentProof>,
}
