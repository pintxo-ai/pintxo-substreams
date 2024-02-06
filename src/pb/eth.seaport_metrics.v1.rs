// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metrics {
    #[prost(message, repeated, tag="1")]
    pub metrics: ::prost::alloc::vec::Vec<Metric>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metric {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub value: u64,
}
// @@protoc_insertion_point(module)
