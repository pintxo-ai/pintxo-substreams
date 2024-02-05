// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeaportPurchases {
    #[prost(message, repeated, tag="1")]
    pub purchases: ::prost::alloc::vec::Vec<Purchase>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Purchase {
    #[prost(uint64, tag="1")]
    pub order_type: u64,
    #[prost(string, tag="2")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub token_in_amount: u64,
    #[prost(string, tag="6")]
    pub token_out: ::prost::alloc::string::String,
    #[prost(uint64, tag="7")]
    pub token_out_amount: u64,
}
// @@protoc_insertion_point(module)
