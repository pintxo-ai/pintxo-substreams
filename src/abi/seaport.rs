    const INTERNAL_ERR: &'static str = "`ethabi_derive` internal error";
    /// Contract's functions.
    #[allow(dead_code, unused_imports, unused_variables)]
    pub mod functions {
        use super::INTERNAL_ERR;
        #[derive(Debug, Clone, PartialEq)]
        pub struct Cancel {
            pub orders: Vec<
                (
                    Vec<u8>,
                    Vec<u8>,
                    Vec<
                        (
                            substreams::scalar::BigInt,
                            Vec<u8>,
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                        ),
                    >,
                    Vec<
                        (
                            substreams::scalar::BigInt,
                            Vec<u8>,
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                            Vec<u8>,
                        ),
                    >,
                    substreams::scalar::BigInt,
                    substreams::scalar::BigInt,
                    substreams::scalar::BigInt,
                    [u8; 32usize],
                    substreams::scalar::BigInt,
                    [u8; 32usize],
                    substreams::scalar::BigInt,
                ),
            >,
        }
        impl Cancel {
            const METHOD_ID: [u8; 4] = [253u8, 159u8, 30u8, 16u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(
                                    ethabi::ParamType::Tuple(
                                        vec![
                                            ethabi::ParamType::Address, ethabi::ParamType::Address,
                                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(8usize),
                                            ethabi::ParamType::Address,
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize)]))),
                                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(8usize),
                                            ethabi::ParamType::Address,
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Address]))),
                                            ethabi::ParamType::Uint(8usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::FixedBytes(32usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::FixedBytes(32usize),
                                            ethabi::ParamType::Uint(256usize)
                                        ],
                                    ),
                                ),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    orders: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                            (
                                tuple_elements[0usize]
                                    .clone()
                                    .into_address()
                                    .expect(INTERNAL_ERR)
                                    .as_bytes()
                                    .to_vec(),
                                tuple_elements[1usize]
                                    .clone()
                                    .into_address()
                                    .expect(INTERNAL_ERR)
                                    .as_bytes()
                                    .to_vec(),
                                tuple_elements[2usize]
                                    .clone()
                                    .into_array()
                                    .expect(INTERNAL_ERR)
                                    .into_iter()
                                    .map(|inner| {
                                        let tuple_elements = inner
                                            .into_tuple()
                                            .expect(INTERNAL_ERR);
                                        (
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                            tuple_elements[1usize]
                                                .clone()
                                                .into_address()
                                                .expect(INTERNAL_ERR)
                                                .as_bytes()
                                                .to_vec(),
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[2usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[3usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[4usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                        )
                                    })
                                    .collect(),
                                tuple_elements[3usize]
                                    .clone()
                                    .into_array()
                                    .expect(INTERNAL_ERR)
                                    .into_iter()
                                    .map(|inner| {
                                        let tuple_elements = inner
                                            .into_tuple()
                                            .expect(INTERNAL_ERR);
                                        (
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                            tuple_elements[1usize]
                                                .clone()
                                                .into_address()
                                                .expect(INTERNAL_ERR)
                                                .as_bytes()
                                                .to_vec(),
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[2usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[3usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[4usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                            tuple_elements[5usize]
                                                .clone()
                                                .into_address()
                                                .expect(INTERNAL_ERR)
                                                .as_bytes()
                                                .to_vec(),
                                        )
                                    })
                                    .collect(),
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[4usize]
                                        .clone()
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                },
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[5usize]
                                        .clone()
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                },
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[6usize]
                                        .clone()
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                },
                                {
                                    let mut result = [0u8; 32];
                                    let v = tuple_elements[7usize]
                                        .clone()
                                        .into_fixed_bytes()
                                        .expect(INTERNAL_ERR);
                                    result.copy_from_slice(&v);
                                    result
                                },
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[8usize]
                                        .clone()
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                },
                                {
                                    let mut result = [0u8; 32];
                                    let v = tuple_elements[9usize]
                                        .clone()
                                        .into_fixed_bytes()
                                        .expect(INTERNAL_ERR);
                                    result.copy_from_slice(&v);
                                    result
                                },
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[10usize]
                                        .clone()
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                },
                            )
                        })
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let v = self
                                .orders
                                .iter()
                                .map(|inner| ethabi::Token::Tuple(
                                    vec![
                                        ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                        .0)), ethabi::Token::Address(ethabi::Address::from_slice(&
                                        inner.1)), { let v = inner.2.iter().map(| inner |
                                        ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                        .1)),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.2.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),)])).collect(); ethabi::Token::Array(v) }, {
                                        let v = inner.3.iter().map(| inner |
                                        ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                        .1)),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.2.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                        .5))])).collect(); ethabi::Token::Array(v) },
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.5.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.6.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),), ethabi::Token::FixedBytes(inner.7.as_ref()
                                        .to_vec()),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.8.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),), ethabi::Token::FixedBytes(inner.9.as_ref()
                                        .to_vec()),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.10.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),)
                                    ],
                                ))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<bool, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<bool, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Bool],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_bool()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<bool> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for Cancel {
            const NAME: &'static str = "cancel";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<bool> for Cancel {
            fn output(data: &[u8]) -> Result<bool, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct FulfillAdvancedOrder {
            pub param0: (
                (
                    Vec<u8>,
                    Vec<u8>,
                    Vec<
                        (
                            substreams::scalar::BigInt,
                            Vec<u8>,
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                        ),
                    >,
                    Vec<
                        (
                            substreams::scalar::BigInt,
                            Vec<u8>,
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                            Vec<u8>,
                        ),
                    >,
                    substreams::scalar::BigInt,
                    substreams::scalar::BigInt,
                    substreams::scalar::BigInt,
                    [u8; 32usize],
                    substreams::scalar::BigInt,
                    [u8; 32usize],
                    substreams::scalar::BigInt,
                ),
                substreams::scalar::BigInt,
                substreams::scalar::BigInt,
                Vec<u8>,
                Vec<u8>,
            ),
            pub param1: Vec<
                (
                    substreams::scalar::BigInt,
                    substreams::scalar::BigInt,
                    substreams::scalar::BigInt,
                    substreams::scalar::BigInt,
                    Vec<[u8; 32usize]>,
                ),
            >,
            pub fulfiller_conduit_key: [u8; 32usize],
            pub recipient: Vec<u8>,
        }
        impl FulfillAdvancedOrder {
            const METHOD_ID: [u8; 4] = [231u8, 172u8, 171u8, 36u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                    ethabi::ParamType::Address,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(8usize),
                                    ethabi::ParamType::Address,
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize)]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(8usize),
                                    ethabi::ParamType::Address,
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Address]))),
                                    ethabi::ParamType::Uint(8usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::FixedBytes(32usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::FixedBytes(32usize),
                                    ethabi::ParamType::Uint(256usize)]),
                                    ethabi::ParamType::Uint(120usize),
                                    ethabi::ParamType::Uint(120usize), ethabi::ParamType::Bytes,
                                    ethabi::ParamType::Bytes
                                ],
                            ),
                            ethabi::ParamType::Array(
                                Box::new(
                                    ethabi::ParamType::Tuple(
                                        vec![
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(8usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::FixedBytes(32usize)))
                                        ],
                                    ),
                                ),
                            ),
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::Address,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    param0: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            {
                                let tuple_elements = tuple_elements[0usize]
                                    .clone()
                                    .into_tuple()
                                    .expect(INTERNAL_ERR);
                                (
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_address()
                                        .expect(INTERNAL_ERR)
                                        .as_bytes()
                                        .to_vec(),
                                    tuple_elements[2usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[0usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_address()
                                                    .expect(INTERNAL_ERR)
                                                    .as_bytes()
                                                    .to_vec(),
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[2usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[3usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[4usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                            )
                                        })
                                        .collect(),
                                    tuple_elements[3usize]
                                        .clone()
                                        .into_array()
                                        .expect(INTERNAL_ERR)
                                        .into_iter()
                                        .map(|inner| {
                                            let tuple_elements = inner
                                                .into_tuple()
                                                .expect(INTERNAL_ERR);
                                            (
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[0usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_address()
                                                    .expect(INTERNAL_ERR)
                                                    .as_bytes()
                                                    .to_vec(),
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[2usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[3usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                                {
                                                    let mut v = [0 as u8; 32];
                                                    tuple_elements[4usize]
                                                        .clone()
                                                        .into_uint()
                                                        .expect(INTERNAL_ERR)
                                                        .to_big_endian(v.as_mut_slice());
                                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                },
                                                tuple_elements[5usize]
                                                    .clone()
                                                    .into_address()
                                                    .expect(INTERNAL_ERR)
                                                    .as_bytes()
                                                    .to_vec(),
                                            )
                                        })
                                        .collect(),
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[4usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[5usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[6usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let mut result = [0u8; 32];
                                        let v = tuple_elements[7usize]
                                            .clone()
                                            .into_fixed_bytes()
                                            .expect(INTERNAL_ERR);
                                        result.copy_from_slice(&v);
                                        result
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[8usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                    {
                                        let mut result = [0u8; 32];
                                        let v = tuple_elements[9usize]
                                            .clone()
                                            .into_fixed_bytes()
                                            .expect(INTERNAL_ERR);
                                        result.copy_from_slice(&v);
                                        result
                                    },
                                    {
                                        let mut v = [0 as u8; 32];
                                        tuple_elements[10usize]
                                            .clone()
                                            .into_uint()
                                            .expect(INTERNAL_ERR)
                                            .to_big_endian(v.as_mut_slice());
                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                    },
                                )
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[1usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[2usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            tuple_elements[3usize]
                                .clone()
                                .into_bytes()
                                .expect(INTERNAL_ERR),
                            tuple_elements[4usize]
                                .clone()
                                .into_bytes()
                                .expect(INTERNAL_ERR),
                        )
                    },
                    param1: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                            (
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                },
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                },
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[2usize]
                                        .clone()
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                },
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[3usize]
                                        .clone()
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                },
                                tuple_elements[4usize]
                                    .clone()
                                    .into_array()
                                    .expect(INTERNAL_ERR)
                                    .into_iter()
                                    .map(|inner| {
                                        let mut result = [0u8; 32];
                                        let v = inner.into_fixed_bytes().expect(INTERNAL_ERR);
                                        result.copy_from_slice(&v);
                                        result
                                    })
                                    .collect(),
                            )
                        })
                        .collect(),
                    fulfiller_conduit_key: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    recipient: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Tuple(
                            vec![
                                ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                self.param0.0.0)),
                                ethabi::Token::Address(ethabi::Address::from_slice(& self
                                .param0.0.1)), { let v = self.param0.0.2.iter().map(| inner
                                |
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                .1)),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.2.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)])).collect(); ethabi::Token::Array(v) }, {
                                let v = self.param0.0.3.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                .1)),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.2.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                .5))])).collect(); ethabi::Token::Array(v) },
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .param0.0.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .param0.0.5.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .param0.0.6.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),), ethabi::Token::FixedBytes(self.param0.0.7
                                .as_ref().to_vec()),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .param0.0.8.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),), ethabi::Token::FixedBytes(self.param0.0.9
                                .as_ref().to_vec()),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .param0.0.10.clone().to_bytes_be() {
                                (num_bigint::Sign::Plus, bytes) => bytes,
                                (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)]),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .param0.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .param0.2.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),), ethabi::Token::Bytes(self.param0.3
                                .clone()), ethabi::Token::Bytes(self.param0.4.clone())
                            ],
                        ),
                        {
                            let v = self
                                .param1
                                .iter()
                                .map(|inner| ethabi::Token::Tuple(
                                    vec![
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.2.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),), { let v = inner.4.iter().map(| inner |
                                        ethabi::Token::FixedBytes(inner.as_ref().to_vec()))
                                        .collect(); ethabi::Token::Array(v) }
                                    ],
                                ))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        ethabi::Token::FixedBytes(
                            self.fulfiller_conduit_key.as_ref().to_vec(),
                        ),
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.recipient),
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<bool, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<bool, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Bool],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_bool()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<bool> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for FulfillAdvancedOrder {
            const NAME: &'static str = "fulfillAdvancedOrder";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<bool> for FulfillAdvancedOrder {
            fn output(data: &[u8]) -> Result<bool, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct FulfillAvailableAdvancedOrders {
            pub param0: Vec<
                (
                    (
                        Vec<u8>,
                        Vec<u8>,
                        Vec<
                            (
                                substreams::scalar::BigInt,
                                Vec<u8>,
                                substreams::scalar::BigInt,
                                substreams::scalar::BigInt,
                                substreams::scalar::BigInt,
                            ),
                        >,
                        Vec<
                            (
                                substreams::scalar::BigInt,
                                Vec<u8>,
                                substreams::scalar::BigInt,
                                substreams::scalar::BigInt,
                                substreams::scalar::BigInt,
                                Vec<u8>,
                            ),
                        >,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        [u8; 32usize],
                        substreams::scalar::BigInt,
                        [u8; 32usize],
                        substreams::scalar::BigInt,
                    ),
                    substreams::scalar::BigInt,
                    substreams::scalar::BigInt,
                    Vec<u8>,
                    Vec<u8>,
                ),
            >,
            pub param1: Vec<
                (
                    substreams::scalar::BigInt,
                    substreams::scalar::BigInt,
                    substreams::scalar::BigInt,
                    substreams::scalar::BigInt,
                    Vec<[u8; 32usize]>,
                ),
            >,
            pub param2: Vec<
                Vec<(substreams::scalar::BigInt, substreams::scalar::BigInt)>,
            >,
            pub param3: Vec<
                Vec<(substreams::scalar::BigInt, substreams::scalar::BigInt)>,
            >,
            pub fulfiller_conduit_key: [u8; 32usize],
            pub recipient: Vec<u8>,
            pub maximum_fulfilled: substreams::scalar::BigInt,
        }
        impl FulfillAvailableAdvancedOrders {
            const METHOD_ID: [u8; 4] = [135u8, 32u8, 27u8, 65u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(
                                    ethabi::ParamType::Tuple(
                                        vec![
                                            ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                            ethabi::ParamType::Address,
                                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(8usize),
                                            ethabi::ParamType::Address,
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize)]))),
                                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(8usize),
                                            ethabi::ParamType::Address,
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Address]))),
                                            ethabi::ParamType::Uint(8usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::FixedBytes(32usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::FixedBytes(32usize),
                                            ethabi::ParamType::Uint(256usize)]),
                                            ethabi::ParamType::Uint(120usize),
                                            ethabi::ParamType::Uint(120usize), ethabi::ParamType::Bytes,
                                            ethabi::ParamType::Bytes
                                        ],
                                    ),
                                ),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(
                                    ethabi::ParamType::Tuple(
                                        vec![
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(8usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::FixedBytes(32usize)))
                                        ],
                                    ),
                                ),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(
                                    ethabi::ParamType::Array(
                                        Box::new(
                                            ethabi::ParamType::Tuple(
                                                vec![
                                                    ethabi::ParamType::Uint(256usize),
                                                    ethabi::ParamType::Uint(256usize)
                                                ],
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(
                                    ethabi::ParamType::Array(
                                        Box::new(
                                            ethabi::ParamType::Tuple(
                                                vec![
                                                    ethabi::ParamType::Uint(256usize),
                                                    ethabi::ParamType::Uint(256usize)
                                                ],
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Uint(256usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    param0: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                            (
                                {
                                    let tuple_elements = tuple_elements[0usize]
                                        .clone()
                                        .into_tuple()
                                        .expect(INTERNAL_ERR);
                                    (
                                        tuple_elements[0usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        tuple_elements[2usize]
                                            .clone()
                                            .into_array()
                                            .expect(INTERNAL_ERR)
                                            .into_iter()
                                            .map(|inner| {
                                                let tuple_elements = inner
                                                    .into_tuple()
                                                    .expect(INTERNAL_ERR);
                                                (
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[0usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_address()
                                                        .expect(INTERNAL_ERR)
                                                        .as_bytes()
                                                        .to_vec(),
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[2usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[3usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[4usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                )
                                            })
                                            .collect(),
                                        tuple_elements[3usize]
                                            .clone()
                                            .into_array()
                                            .expect(INTERNAL_ERR)
                                            .into_iter()
                                            .map(|inner| {
                                                let tuple_elements = inner
                                                    .into_tuple()
                                                    .expect(INTERNAL_ERR);
                                                (
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[0usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_address()
                                                        .expect(INTERNAL_ERR)
                                                        .as_bytes()
                                                        .to_vec(),
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[2usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[3usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[4usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    tuple_elements[5usize]
                                                        .clone()
                                                        .into_address()
                                                        .expect(INTERNAL_ERR)
                                                        .as_bytes()
                                                        .to_vec(),
                                                )
                                            })
                                            .collect(),
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[4usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[5usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[6usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut result = [0u8; 32];
                                            let v = tuple_elements[7usize]
                                                .clone()
                                                .into_fixed_bytes()
                                                .expect(INTERNAL_ERR);
                                            result.copy_from_slice(&v);
                                            result
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[8usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut result = [0u8; 32];
                                            let v = tuple_elements[9usize]
                                                .clone()
                                                .into_fixed_bytes()
                                                .expect(INTERNAL_ERR);
                                            result.copy_from_slice(&v);
                                            result
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[10usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                    )
                                },
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                },
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[2usize]
                                        .clone()
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                },
                                tuple_elements[3usize]
                                    .clone()
                                    .into_bytes()
                                    .expect(INTERNAL_ERR),
                                tuple_elements[4usize]
                                    .clone()
                                    .into_bytes()
                                    .expect(INTERNAL_ERR),
                            )
                        })
                        .collect(),
                    param1: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                            (
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                },
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                },
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[2usize]
                                        .clone()
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                },
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[3usize]
                                        .clone()
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                },
                                tuple_elements[4usize]
                                    .clone()
                                    .into_array()
                                    .expect(INTERNAL_ERR)
                                    .into_iter()
                                    .map(|inner| {
                                        let mut result = [0u8; 32];
                                        let v = inner.into_fixed_bytes().expect(INTERNAL_ERR);
                                        result.copy_from_slice(&v);
                                        result
                                    })
                                    .collect(),
                            )
                        })
                        .collect(),
                    param2: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            inner
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| {
                                    let tuple_elements = inner
                                        .into_tuple()
                                        .expect(INTERNAL_ERR);
                                    (
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[0usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[1usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                    )
                                })
                                .collect()
                        })
                        .collect(),
                    param3: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            inner
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| {
                                    let tuple_elements = inner
                                        .into_tuple()
                                        .expect(INTERNAL_ERR);
                                    (
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[0usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[1usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                    )
                                })
                                .collect()
                        })
                        .collect(),
                    fulfiller_conduit_key: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    recipient: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    maximum_fulfilled: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let v = self
                                .param0
                                .iter()
                                .map(|inner| ethabi::Token::Tuple(
                                    vec![
                                        ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                        inner.0.0)),
                                        ethabi::Token::Address(ethabi::Address::from_slice(& inner.0
                                        .1)), { let v = inner.0.2.iter().map(| inner |
                                        ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                        .1)),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.2.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),)])).collect(); ethabi::Token::Array(v) }, {
                                        let v = inner.0.3.iter().map(| inner |
                                        ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                        .1)),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.2.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                        .5))])).collect(); ethabi::Token::Array(v) },
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.5.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.6.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),), ethabi::Token::FixedBytes(inner.0.7
                                        .as_ref().to_vec()),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.8.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),), ethabi::Token::FixedBytes(inner.0.9
                                        .as_ref().to_vec()),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.10.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),)]),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.2.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),), ethabi::Token::Bytes(inner.3.clone()),
                                        ethabi::Token::Bytes(inner.4.clone())
                                    ],
                                ))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .param1
                                .iter()
                                .map(|inner| ethabi::Token::Tuple(
                                    vec![
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.2.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),), { let v = inner.4.iter().map(| inner |
                                        ethabi::Token::FixedBytes(inner.as_ref().to_vec()))
                                        .collect(); ethabi::Token::Array(v) }
                                    ],
                                ))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .param2
                                .iter()
                                .map(|inner| {
                                    let v = inner
                                        .iter()
                                        .map(|inner| ethabi::Token::Tuple(
                                            vec![
                                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                                inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                                (num_bigint::Sign::Minus, _) => {
                                                panic!("negative numbers are not supported") }, }
                                                .as_slice(),),),
                                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                                inner.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                                (num_bigint::Sign::Minus, _) => {
                                                panic!("negative numbers are not supported") }, }
                                                .as_slice(),),)
                                            ],
                                        ))
                                        .collect();
                                    ethabi::Token::Array(v)
                                })
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .param3
                                .iter()
                                .map(|inner| {
                                    let v = inner
                                        .iter()
                                        .map(|inner| ethabi::Token::Tuple(
                                            vec![
                                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                                inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                                (num_bigint::Sign::Minus, _) => {
                                                panic!("negative numbers are not supported") }, }
                                                .as_slice(),),),
                                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                                inner.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                                (num_bigint::Sign::Minus, _) => {
                                                panic!("negative numbers are not supported") }, }
                                                .as_slice(),),)
                                            ],
                                        ))
                                        .collect();
                                    ethabi::Token::Array(v)
                                })
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        ethabi::Token::FixedBytes(
                            self.fulfiller_conduit_key.as_ref().to_vec(),
                        ),
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.recipient),
                        ),
                        ethabi::Token::Uint(
                            ethabi::Uint::from_big_endian(
                                match self.maximum_fulfilled.clone().to_bytes_be() {
                                    (num_bigint::Sign::Plus, bytes) => bytes,
                                    (num_bigint::Sign::NoSign, bytes) => bytes,
                                    (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported")
                                    }
                                }
                                    .as_slice(),
                            ),
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<
                (
                    Vec<bool>,
                    Vec<
                        (
                            (
                                substreams::scalar::BigInt,
                                Vec<u8>,
                                substreams::scalar::BigInt,
                                substreams::scalar::BigInt,
                                Vec<u8>,
                            ),
                            Vec<u8>,
                            [u8; 32usize],
                        ),
                    >,
                ),
                String,
            > {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(
                data: &[u8],
            ) -> Result<
                (
                    Vec<bool>,
                    Vec<
                        (
                            (
                                substreams::scalar::BigInt,
                                Vec<u8>,
                                substreams::scalar::BigInt,
                                substreams::scalar::BigInt,
                                Vec<u8>,
                            ),
                            Vec<u8>,
                            [u8; 32usize],
                        ),
                    >,
                ),
                String,
            > {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Bool)),
                            ethabi::ParamType::Array(
                                Box::new(
                                    ethabi::ParamType::Tuple(
                                        vec![
                                            ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(8usize),
                                            ethabi::ParamType::Address,
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Address]), ethabi::ParamType::Address,
                                            ethabi::ParamType::FixedBytes(32usize)
                                        ],
                                    ),
                                ),
                            ),
                        ],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                values.reverse();
                Ok((
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_bool().expect(INTERNAL_ERR))
                        .collect(),
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                            (
                                {
                                    let tuple_elements = tuple_elements[0usize]
                                        .clone()
                                        .into_tuple()
                                        .expect(INTERNAL_ERR);
                                    (
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[0usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[2usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[3usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        tuple_elements[4usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                    )
                                },
                                tuple_elements[1usize]
                                    .clone()
                                    .into_address()
                                    .expect(INTERNAL_ERR)
                                    .as_bytes()
                                    .to_vec(),
                                {
                                    let mut result = [0u8; 32];
                                    let v = tuple_elements[2usize]
                                        .clone()
                                        .into_fixed_bytes()
                                        .expect(INTERNAL_ERR);
                                    result.copy_from_slice(&v);
                                    result
                                },
                            )
                        })
                        .collect(),
                ))
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(
                &self,
                address: Vec<u8>,
            ) -> Option<
                (
                    Vec<bool>,
                    Vec<
                        (
                            (
                                substreams::scalar::BigInt,
                                Vec<u8>,
                                substreams::scalar::BigInt,
                                substreams::scalar::BigInt,
                                Vec<u8>,
                            ),
                            Vec<u8>,
                            [u8; 32usize],
                        ),
                    >,
                ),
            > {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for FulfillAvailableAdvancedOrders {
            const NAME: &'static str = "fulfillAvailableAdvancedOrders";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<
            (
                Vec<bool>,
                Vec<
                    (
                        (
                            substreams::scalar::BigInt,
                            Vec<u8>,
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                            Vec<u8>,
                        ),
                        Vec<u8>,
                        [u8; 32usize],
                    ),
                >,
            ),
        > for FulfillAvailableAdvancedOrders {
            fn output(
                data: &[u8],
            ) -> Result<
                (
                    Vec<bool>,
                    Vec<
                        (
                            (
                                substreams::scalar::BigInt,
                                Vec<u8>,
                                substreams::scalar::BigInt,
                                substreams::scalar::BigInt,
                                Vec<u8>,
                            ),
                            Vec<u8>,
                            [u8; 32usize],
                        ),
                    >,
                ),
                String,
            > {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct FulfillAvailableOrders {
            pub param0: Vec<
                (
                    (
                        Vec<u8>,
                        Vec<u8>,
                        Vec<
                            (
                                substreams::scalar::BigInt,
                                Vec<u8>,
                                substreams::scalar::BigInt,
                                substreams::scalar::BigInt,
                                substreams::scalar::BigInt,
                            ),
                        >,
                        Vec<
                            (
                                substreams::scalar::BigInt,
                                Vec<u8>,
                                substreams::scalar::BigInt,
                                substreams::scalar::BigInt,
                                substreams::scalar::BigInt,
                                Vec<u8>,
                            ),
                        >,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        [u8; 32usize],
                        substreams::scalar::BigInt,
                        [u8; 32usize],
                        substreams::scalar::BigInt,
                    ),
                    Vec<u8>,
                ),
            >,
            pub param1: Vec<
                Vec<(substreams::scalar::BigInt, substreams::scalar::BigInt)>,
            >,
            pub param2: Vec<
                Vec<(substreams::scalar::BigInt, substreams::scalar::BigInt)>,
            >,
            pub fulfiller_conduit_key: [u8; 32usize],
            pub maximum_fulfilled: substreams::scalar::BigInt,
        }
        impl FulfillAvailableOrders {
            const METHOD_ID: [u8; 4] = [237u8, 152u8, 165u8, 116u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(
                                    ethabi::ParamType::Tuple(
                                        vec![
                                            ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                            ethabi::ParamType::Address,
                                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(8usize),
                                            ethabi::ParamType::Address,
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize)]))),
                                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(8usize),
                                            ethabi::ParamType::Address,
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Address]))),
                                            ethabi::ParamType::Uint(8usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::FixedBytes(32usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::FixedBytes(32usize),
                                            ethabi::ParamType::Uint(256usize)]),
                                            ethabi::ParamType::Bytes
                                        ],
                                    ),
                                ),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(
                                    ethabi::ParamType::Array(
                                        Box::new(
                                            ethabi::ParamType::Tuple(
                                                vec![
                                                    ethabi::ParamType::Uint(256usize),
                                                    ethabi::ParamType::Uint(256usize)
                                                ],
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(
                                    ethabi::ParamType::Array(
                                        Box::new(
                                            ethabi::ParamType::Tuple(
                                                vec![
                                                    ethabi::ParamType::Uint(256usize),
                                                    ethabi::ParamType::Uint(256usize)
                                                ],
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    param0: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                            (
                                {
                                    let tuple_elements = tuple_elements[0usize]
                                        .clone()
                                        .into_tuple()
                                        .expect(INTERNAL_ERR);
                                    (
                                        tuple_elements[0usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        tuple_elements[2usize]
                                            .clone()
                                            .into_array()
                                            .expect(INTERNAL_ERR)
                                            .into_iter()
                                            .map(|inner| {
                                                let tuple_elements = inner
                                                    .into_tuple()
                                                    .expect(INTERNAL_ERR);
                                                (
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[0usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_address()
                                                        .expect(INTERNAL_ERR)
                                                        .as_bytes()
                                                        .to_vec(),
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[2usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[3usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[4usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                )
                                            })
                                            .collect(),
                                        tuple_elements[3usize]
                                            .clone()
                                            .into_array()
                                            .expect(INTERNAL_ERR)
                                            .into_iter()
                                            .map(|inner| {
                                                let tuple_elements = inner
                                                    .into_tuple()
                                                    .expect(INTERNAL_ERR);
                                                (
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[0usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_address()
                                                        .expect(INTERNAL_ERR)
                                                        .as_bytes()
                                                        .to_vec(),
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[2usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[3usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[4usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    tuple_elements[5usize]
                                                        .clone()
                                                        .into_address()
                                                        .expect(INTERNAL_ERR)
                                                        .as_bytes()
                                                        .to_vec(),
                                                )
                                            })
                                            .collect(),
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[4usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[5usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[6usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut result = [0u8; 32];
                                            let v = tuple_elements[7usize]
                                                .clone()
                                                .into_fixed_bytes()
                                                .expect(INTERNAL_ERR);
                                            result.copy_from_slice(&v);
                                            result
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[8usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut result = [0u8; 32];
                                            let v = tuple_elements[9usize]
                                                .clone()
                                                .into_fixed_bytes()
                                                .expect(INTERNAL_ERR);
                                            result.copy_from_slice(&v);
                                            result
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[10usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                    )
                                },
                                tuple_elements[1usize]
                                    .clone()
                                    .into_bytes()
                                    .expect(INTERNAL_ERR),
                            )
                        })
                        .collect(),
                    param1: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            inner
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| {
                                    let tuple_elements = inner
                                        .into_tuple()
                                        .expect(INTERNAL_ERR);
                                    (
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[0usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[1usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                    )
                                })
                                .collect()
                        })
                        .collect(),
                    param2: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            inner
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| {
                                    let tuple_elements = inner
                                        .into_tuple()
                                        .expect(INTERNAL_ERR);
                                    (
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[0usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[1usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                    )
                                })
                                .collect()
                        })
                        .collect(),
                    fulfiller_conduit_key: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    maximum_fulfilled: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let v = self
                                .param0
                                .iter()
                                .map(|inner| ethabi::Token::Tuple(
                                    vec![
                                        ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                        inner.0.0)),
                                        ethabi::Token::Address(ethabi::Address::from_slice(& inner.0
                                        .1)), { let v = inner.0.2.iter().map(| inner |
                                        ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                        .1)),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.2.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),)])).collect(); ethabi::Token::Array(v) }, {
                                        let v = inner.0.3.iter().map(| inner |
                                        ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                        .1)),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.2.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                        .5))])).collect(); ethabi::Token::Array(v) },
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.5.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.6.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),), ethabi::Token::FixedBytes(inner.0.7
                                        .as_ref().to_vec()),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.8.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),), ethabi::Token::FixedBytes(inner.0.9
                                        .as_ref().to_vec()),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.10.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),)]), ethabi::Token::Bytes(inner.1.clone())
                                    ],
                                ))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .param1
                                .iter()
                                .map(|inner| {
                                    let v = inner
                                        .iter()
                                        .map(|inner| ethabi::Token::Tuple(
                                            vec![
                                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                                inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                                (num_bigint::Sign::Minus, _) => {
                                                panic!("negative numbers are not supported") }, }
                                                .as_slice(),),),
                                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                                inner.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                                (num_bigint::Sign::Minus, _) => {
                                                panic!("negative numbers are not supported") }, }
                                                .as_slice(),),)
                                            ],
                                        ))
                                        .collect();
                                    ethabi::Token::Array(v)
                                })
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .param2
                                .iter()
                                .map(|inner| {
                                    let v = inner
                                        .iter()
                                        .map(|inner| ethabi::Token::Tuple(
                                            vec![
                                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                                inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                                (num_bigint::Sign::Minus, _) => {
                                                panic!("negative numbers are not supported") }, }
                                                .as_slice(),),),
                                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                                inner.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                                (num_bigint::Sign::Minus, _) => {
                                                panic!("negative numbers are not supported") }, }
                                                .as_slice(),),)
                                            ],
                                        ))
                                        .collect();
                                    ethabi::Token::Array(v)
                                })
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        ethabi::Token::FixedBytes(
                            self.fulfiller_conduit_key.as_ref().to_vec(),
                        ),
                        ethabi::Token::Uint(
                            ethabi::Uint::from_big_endian(
                                match self.maximum_fulfilled.clone().to_bytes_be() {
                                    (num_bigint::Sign::Plus, bytes) => bytes,
                                    (num_bigint::Sign::NoSign, bytes) => bytes,
                                    (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported")
                                    }
                                }
                                    .as_slice(),
                            ),
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<
                (
                    Vec<bool>,
                    Vec<
                        (
                            (
                                substreams::scalar::BigInt,
                                Vec<u8>,
                                substreams::scalar::BigInt,
                                substreams::scalar::BigInt,
                                Vec<u8>,
                            ),
                            Vec<u8>,
                            [u8; 32usize],
                        ),
                    >,
                ),
                String,
            > {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(
                data: &[u8],
            ) -> Result<
                (
                    Vec<bool>,
                    Vec<
                        (
                            (
                                substreams::scalar::BigInt,
                                Vec<u8>,
                                substreams::scalar::BigInt,
                                substreams::scalar::BigInt,
                                Vec<u8>,
                            ),
                            Vec<u8>,
                            [u8; 32usize],
                        ),
                    >,
                ),
                String,
            > {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Bool)),
                            ethabi::ParamType::Array(
                                Box::new(
                                    ethabi::ParamType::Tuple(
                                        vec![
                                            ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(8usize),
                                            ethabi::ParamType::Address,
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Address]), ethabi::ParamType::Address,
                                            ethabi::ParamType::FixedBytes(32usize)
                                        ],
                                    ),
                                ),
                            ),
                        ],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                values.reverse();
                Ok((
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| inner.into_bool().expect(INTERNAL_ERR))
                        .collect(),
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                            (
                                {
                                    let tuple_elements = tuple_elements[0usize]
                                        .clone()
                                        .into_tuple()
                                        .expect(INTERNAL_ERR);
                                    (
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[0usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[2usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[3usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        tuple_elements[4usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                    )
                                },
                                tuple_elements[1usize]
                                    .clone()
                                    .into_address()
                                    .expect(INTERNAL_ERR)
                                    .as_bytes()
                                    .to_vec(),
                                {
                                    let mut result = [0u8; 32];
                                    let v = tuple_elements[2usize]
                                        .clone()
                                        .into_fixed_bytes()
                                        .expect(INTERNAL_ERR);
                                    result.copy_from_slice(&v);
                                    result
                                },
                            )
                        })
                        .collect(),
                ))
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(
                &self,
                address: Vec<u8>,
            ) -> Option<
                (
                    Vec<bool>,
                    Vec<
                        (
                            (
                                substreams::scalar::BigInt,
                                Vec<u8>,
                                substreams::scalar::BigInt,
                                substreams::scalar::BigInt,
                                Vec<u8>,
                            ),
                            Vec<u8>,
                            [u8; 32usize],
                        ),
                    >,
                ),
            > {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for FulfillAvailableOrders {
            const NAME: &'static str = "fulfillAvailableOrders";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<
            (
                Vec<bool>,
                Vec<
                    (
                        (
                            substreams::scalar::BigInt,
                            Vec<u8>,
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                            Vec<u8>,
                        ),
                        Vec<u8>,
                        [u8; 32usize],
                    ),
                >,
            ),
        > for FulfillAvailableOrders {
            fn output(
                data: &[u8],
            ) -> Result<
                (
                    Vec<bool>,
                    Vec<
                        (
                            (
                                substreams::scalar::BigInt,
                                Vec<u8>,
                                substreams::scalar::BigInt,
                                substreams::scalar::BigInt,
                                Vec<u8>,
                            ),
                            Vec<u8>,
                            [u8; 32usize],
                        ),
                    >,
                ),
                String,
            > {
                Self::output(data)
            }
        }
        
       #[derive(Debug, Clone, PartialEq)]
        pub struct GetContractOffererNonce {
            pub contract_offerer: Vec<u8>,
        }
        impl GetContractOffererNonce {
            const METHOD_ID: [u8; 4] = [169u8, 0u8, 134u8, 107u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    contract_offerer: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.contract_offerer),
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<substreams::scalar::BigInt, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<substreams::scalar::BigInt, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok({
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                })
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<substreams::scalar::BigInt> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for GetContractOffererNonce {
            const NAME: &'static str = "getContractOffererNonce";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<substreams::scalar::BigInt>
        for GetContractOffererNonce {
            fn output(data: &[u8]) -> Result<substreams::scalar::BigInt, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetCounter {
            pub offerer: Vec<u8>,
        }
        impl GetCounter {
            const METHOD_ID: [u8; 4] = [240u8, 126u8, 195u8, 115u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Address],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    offerer: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[ethabi::Token::Address(ethabi::Address::from_slice(&self.offerer))],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<substreams::scalar::BigInt, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<substreams::scalar::BigInt, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok({
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                })
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<substreams::scalar::BigInt> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for GetCounter {
            const NAME: &'static str = "getCounter";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<substreams::scalar::BigInt>
        for GetCounter {
            fn output(data: &[u8]) -> Result<substreams::scalar::BigInt, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetOrderHash {
            pub param0: (
                Vec<u8>,
                Vec<u8>,
                Vec<
                    (
                        substreams::scalar::BigInt,
                        Vec<u8>,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                    ),
                >,
                Vec<
                    (
                        substreams::scalar::BigInt,
                        Vec<u8>,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        Vec<u8>,
                    ),
                >,
                substreams::scalar::BigInt,
                substreams::scalar::BigInt,
                substreams::scalar::BigInt,
                [u8; 32usize],
                substreams::scalar::BigInt,
                [u8; 32usize],
                substreams::scalar::BigInt,
            ),
        }
        impl GetOrderHash {
            const METHOD_ID: [u8; 4] = [121u8, 223u8, 114u8, 189u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Address, ethabi::ParamType::Address,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(8usize),
                                    ethabi::ParamType::Address,
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize)]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(8usize),
                                    ethabi::ParamType::Address,
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Address]))),
                                    ethabi::ParamType::Uint(8usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::FixedBytes(32usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::FixedBytes(32usize),
                                    ethabi::ParamType::Uint(256usize)
                                ],
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    param0: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            tuple_elements[0usize]
                                .clone()
                                .into_address()
                                .expect(INTERNAL_ERR)
                                .as_bytes()
                                .to_vec(),
                            tuple_elements[1usize]
                                .clone()
                                .into_address()
                                .expect(INTERNAL_ERR)
                                .as_bytes()
                                .to_vec(),
                            tuple_elements[2usize]
                                .clone()
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| {
                                    let tuple_elements = inner
                                        .into_tuple()
                                        .expect(INTERNAL_ERR);
                                    (
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[0usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[2usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[3usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[4usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                    )
                                })
                                .collect(),
                            tuple_elements[3usize]
                                .clone()
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| {
                                    let tuple_elements = inner
                                        .into_tuple()
                                        .expect(INTERNAL_ERR);
                                    (
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[0usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[2usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[3usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[4usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        tuple_elements[5usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                    )
                                })
                                .collect(),
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[4usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[5usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[6usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            {
                                let mut result = [0u8; 32];
                                let v = tuple_elements[7usize]
                                    .clone()
                                    .into_fixed_bytes()
                                    .expect(INTERNAL_ERR);
                                result.copy_from_slice(&v);
                                result
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[8usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            {
                                let mut result = [0u8; 32];
                                let v = tuple_elements[9usize]
                                    .clone()
                                    .into_fixed_bytes()
                                    .expect(INTERNAL_ERR);
                                result.copy_from_slice(&v);
                                result
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[10usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                        )
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        ethabi::Token::Tuple(
                            vec![
                                ethabi::Token::Address(ethabi::Address::from_slice(& self
                                .param0.0)),
                                ethabi::Token::Address(ethabi::Address::from_slice(& self
                                .param0.1)), { let v = self.param0.2.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                .1)),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.2.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)])).collect(); ethabi::Token::Array(v) }, {
                                let v = self.param0.3.iter().map(| inner |
                                ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                .1)),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.2.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                inner.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                .5))])).collect(); ethabi::Token::Array(v) },
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .param0.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .param0.5.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .param0.6.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),), ethabi::Token::FixedBytes(self.param0.7
                                .as_ref().to_vec()),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .param0.8.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),), ethabi::Token::FixedBytes(self.param0.9
                                .as_ref().to_vec()),
                                ethabi::Token::Uint(ethabi::Uint::from_big_endian(match self
                                .param0.10.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                (num_bigint::Sign::Minus, _) => {
                                panic!("negative numbers are not supported") }, }
                                .as_slice(),),)
                            ],
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<[u8; 32usize], String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<[u8; 32usize], String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::FixedBytes(32usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok({
                    let mut result = [0u8; 32];
                    let v = values
                        .pop()
                        .expect("one output data should have existed")
                        .into_fixed_bytes()
                        .expect(INTERNAL_ERR);
                    result.copy_from_slice(&v);
                    result
                })
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<[u8; 32usize]> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for GetOrderHash {
            const NAME: &'static str = "getOrderHash";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<[u8; 32usize]> for GetOrderHash {
            fn output(data: &[u8]) -> Result<[u8; 32usize], String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct GetOrderStatus {
            pub order_hash: [u8; 32usize],
        }
        impl GetOrderStatus {
            const METHOD_ID: [u8; 4] = [70u8, 66u8, 58u8, 167u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::FixedBytes(32usize)],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    order_hash: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[ethabi::Token::FixedBytes(self.order_hash.as_ref().to_vec())],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<
                (bool, bool, substreams::scalar::BigInt, substreams::scalar::BigInt),
                String,
            > {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(
                data: &[u8],
            ) -> Result<
                (bool, bool, substreams::scalar::BigInt, substreams::scalar::BigInt),
                String,
            > {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Bool,
                            ethabi::ParamType::Bool,
                            ethabi::ParamType::Uint(256usize),
                            ethabi::ParamType::Uint(256usize),
                        ],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                values.reverse();
                Ok((
                    values.pop().expect(INTERNAL_ERR).into_bool().expect(INTERNAL_ERR),
                    values.pop().expect(INTERNAL_ERR).into_bool().expect(INTERNAL_ERR),
                    {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                    {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                ))
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(
                &self,
                address: Vec<u8>,
            ) -> Option<
                (bool, bool, substreams::scalar::BigInt, substreams::scalar::BigInt),
            > {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for GetOrderStatus {
            const NAME: &'static str = "getOrderStatus";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<
            (bool, bool, substreams::scalar::BigInt, substreams::scalar::BigInt),
        > for GetOrderStatus {
            fn output(
                data: &[u8],
            ) -> Result<
                (bool, bool, substreams::scalar::BigInt, substreams::scalar::BigInt),
                String,
            > {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct IncrementCounter {}
        impl IncrementCounter {
            const METHOD_ID: [u8; 4] = [91u8, 52u8, 185u8, 102u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<substreams::scalar::BigInt, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<substreams::scalar::BigInt, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok({
                    let mut v = [0 as u8; 32];
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_uint()
                        .expect(INTERNAL_ERR)
                        .to_big_endian(v.as_mut_slice());
                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                })
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<substreams::scalar::BigInt> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for IncrementCounter {
            const NAME: &'static str = "incrementCounter";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<substreams::scalar::BigInt>
        for IncrementCounter {
            fn output(data: &[u8]) -> Result<substreams::scalar::BigInt, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Information {}
        impl Information {
            const METHOD_ID: [u8; 4] = [244u8, 123u8, 119u8, 64u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<(String, [u8; 32usize], Vec<u8>), String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(
                data: &[u8],
            ) -> Result<(String, [u8; 32usize], Vec<u8>), String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::String,
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::Address,
                        ],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                values.reverse();
                Ok((
                    values.pop().expect(INTERNAL_ERR).into_string().expect(INTERNAL_ERR),
                    {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                ))
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(
                &self,
                address: Vec<u8>,
            ) -> Option<(String, [u8; 32usize], Vec<u8>)> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for Information {
            const NAME: &'static str = "information";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<(String, [u8; 32usize], Vec<u8>)>
        for Information {
            fn output(data: &[u8]) -> Result<(String, [u8; 32usize], Vec<u8>), String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct MatchAdvancedOrders {
            pub param0: Vec<
                (
                    (
                        Vec<u8>,
                        Vec<u8>,
                        Vec<
                            (
                                substreams::scalar::BigInt,
                                Vec<u8>,
                                substreams::scalar::BigInt,
                                substreams::scalar::BigInt,
                                substreams::scalar::BigInt,
                            ),
                        >,
                        Vec<
                            (
                                substreams::scalar::BigInt,
                                Vec<u8>,
                                substreams::scalar::BigInt,
                                substreams::scalar::BigInt,
                                substreams::scalar::BigInt,
                                Vec<u8>,
                            ),
                        >,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        [u8; 32usize],
                        substreams::scalar::BigInt,
                        [u8; 32usize],
                        substreams::scalar::BigInt,
                    ),
                    substreams::scalar::BigInt,
                    substreams::scalar::BigInt,
                    Vec<u8>,
                    Vec<u8>,
                ),
            >,
            pub param1: Vec<
                (
                    substreams::scalar::BigInt,
                    substreams::scalar::BigInt,
                    substreams::scalar::BigInt,
                    substreams::scalar::BigInt,
                    Vec<[u8; 32usize]>,
                ),
            >,
            pub param2: Vec<
                (
                    Vec<(substreams::scalar::BigInt, substreams::scalar::BigInt)>,
                    Vec<(substreams::scalar::BigInt, substreams::scalar::BigInt)>,
                ),
            >,
            pub recipient: Vec<u8>,
        }
        impl MatchAdvancedOrders {
            const METHOD_ID: [u8; 4] = [242u8, 209u8, 43u8, 18u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(
                                    ethabi::ParamType::Tuple(
                                        vec![
                                            ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                            ethabi::ParamType::Address,
                                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(8usize),
                                            ethabi::ParamType::Address,
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize)]))),
                                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(8usize),
                                            ethabi::ParamType::Address,
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Address]))),
                                            ethabi::ParamType::Uint(8usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::FixedBytes(32usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::FixedBytes(32usize),
                                            ethabi::ParamType::Uint(256usize)]),
                                            ethabi::ParamType::Uint(120usize),
                                            ethabi::ParamType::Uint(120usize), ethabi::ParamType::Bytes,
                                            ethabi::ParamType::Bytes
                                        ],
                                    ),
                                ),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(
                                    ethabi::ParamType::Tuple(
                                        vec![
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(8usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::FixedBytes(32usize)))
                                        ],
                                    ),
                                ),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(
                                    ethabi::ParamType::Tuple(
                                        vec![
                                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize)]))),
                                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize)])))
                                        ],
                                    ),
                                ),
                            ),
                            ethabi::ParamType::Address,
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    param0: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                            (
                                {
                                    let tuple_elements = tuple_elements[0usize]
                                        .clone()
                                        .into_tuple()
                                        .expect(INTERNAL_ERR);
                                    (
                                        tuple_elements[0usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        tuple_elements[2usize]
                                            .clone()
                                            .into_array()
                                            .expect(INTERNAL_ERR)
                                            .into_iter()
                                            .map(|inner| {
                                                let tuple_elements = inner
                                                    .into_tuple()
                                                    .expect(INTERNAL_ERR);
                                                (
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[0usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_address()
                                                        .expect(INTERNAL_ERR)
                                                        .as_bytes()
                                                        .to_vec(),
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[2usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[3usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[4usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                )
                                            })
                                            .collect(),
                                        tuple_elements[3usize]
                                            .clone()
                                            .into_array()
                                            .expect(INTERNAL_ERR)
                                            .into_iter()
                                            .map(|inner| {
                                                let tuple_elements = inner
                                                    .into_tuple()
                                                    .expect(INTERNAL_ERR);
                                                (
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[0usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_address()
                                                        .expect(INTERNAL_ERR)
                                                        .as_bytes()
                                                        .to_vec(),
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[2usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[3usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[4usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    tuple_elements[5usize]
                                                        .clone()
                                                        .into_address()
                                                        .expect(INTERNAL_ERR)
                                                        .as_bytes()
                                                        .to_vec(),
                                                )
                                            })
                                            .collect(),
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[4usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[5usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[6usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut result = [0u8; 32];
                                            let v = tuple_elements[7usize]
                                                .clone()
                                                .into_fixed_bytes()
                                                .expect(INTERNAL_ERR);
                                            result.copy_from_slice(&v);
                                            result
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[8usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut result = [0u8; 32];
                                            let v = tuple_elements[9usize]
                                                .clone()
                                                .into_fixed_bytes()
                                                .expect(INTERNAL_ERR);
                                            result.copy_from_slice(&v);
                                            result
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[10usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                    )
                                },
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                },
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[2usize]
                                        .clone()
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                },
                                tuple_elements[3usize]
                                    .clone()
                                    .into_bytes()
                                    .expect(INTERNAL_ERR),
                                tuple_elements[4usize]
                                    .clone()
                                    .into_bytes()
                                    .expect(INTERNAL_ERR),
                            )
                        })
                        .collect(),
                    param1: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                            (
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                },
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[1usize]
                                        .clone()
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                },
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[2usize]
                                        .clone()
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                },
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[3usize]
                                        .clone()
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                },
                                tuple_elements[4usize]
                                    .clone()
                                    .into_array()
                                    .expect(INTERNAL_ERR)
                                    .into_iter()
                                    .map(|inner| {
                                        let mut result = [0u8; 32];
                                        let v = inner.into_fixed_bytes().expect(INTERNAL_ERR);
                                        result.copy_from_slice(&v);
                                        result
                                    })
                                    .collect(),
                            )
                        })
                        .collect(),
                    param2: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                            (
                                tuple_elements[0usize]
                                    .clone()
                                    .into_array()
                                    .expect(INTERNAL_ERR)
                                    .into_iter()
                                    .map(|inner| {
                                        let tuple_elements = inner
                                            .into_tuple()
                                            .expect(INTERNAL_ERR);
                                        (
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                        )
                                    })
                                    .collect(),
                                tuple_elements[1usize]
                                    .clone()
                                    .into_array()
                                    .expect(INTERNAL_ERR)
                                    .into_iter()
                                    .map(|inner| {
                                        let tuple_elements = inner
                                            .into_tuple()
                                            .expect(INTERNAL_ERR);
                                        (
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                        )
                                    })
                                    .collect(),
                            )
                        })
                        .collect(),
                    recipient: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let v = self
                                .param0
                                .iter()
                                .map(|inner| ethabi::Token::Tuple(
                                    vec![
                                        ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                        inner.0.0)),
                                        ethabi::Token::Address(ethabi::Address::from_slice(& inner.0
                                        .1)), { let v = inner.0.2.iter().map(| inner |
                                        ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                        .1)),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.2.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),)])).collect(); ethabi::Token::Array(v) }, {
                                        let v = inner.0.3.iter().map(| inner |
                                        ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                        .1)),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.2.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                        .5))])).collect(); ethabi::Token::Array(v) },
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.5.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.6.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),), ethabi::Token::FixedBytes(inner.0.7
                                        .as_ref().to_vec()),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.8.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),), ethabi::Token::FixedBytes(inner.0.9
                                        .as_ref().to_vec()),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.10.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),)]),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.2.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),), ethabi::Token::Bytes(inner.3.clone()),
                                        ethabi::Token::Bytes(inner.4.clone())
                                    ],
                                ))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .param1
                                .iter()
                                .map(|inner| ethabi::Token::Tuple(
                                    vec![
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.2.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),), { let v = inner.4.iter().map(| inner |
                                        ethabi::Token::FixedBytes(inner.as_ref().to_vec()))
                                        .collect(); ethabi::Token::Array(v) }
                                    ],
                                ))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .param2
                                .iter()
                                .map(|inner| ethabi::Token::Tuple(
                                    vec![
                                        { let v = inner.0.iter().map(| inner |
                                        ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),)])).collect(); ethabi::Token::Array(v) }, {
                                        let v = inner.1.iter().map(| inner |
                                        ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),)])).collect(); ethabi::Token::Array(v) }
                                    ],
                                ))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        ethabi::Token::Address(
                            ethabi::Address::from_slice(&self.recipient),
                        ),
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<
                Vec<
                    (
                        (
                            substreams::scalar::BigInt,
                            Vec<u8>,
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                            Vec<u8>,
                        ),
                        Vec<u8>,
                        [u8; 32usize],
                    ),
                >,
                String,
            > {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(
                data: &[u8],
            ) -> Result<
                Vec<
                    (
                        (
                            substreams::scalar::BigInt,
                            Vec<u8>,
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                            Vec<u8>,
                        ),
                        Vec<u8>,
                        [u8; 32usize],
                    ),
                >,
                String,
            > {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(
                                    ethabi::ParamType::Tuple(
                                        vec![
                                            ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(8usize),
                                            ethabi::ParamType::Address,
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Address]), ethabi::ParamType::Address,
                                            ethabi::ParamType::FixedBytes(32usize)
                                        ],
                                    ),
                                ),
                            ),
                        ],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                            (
                                {
                                    let tuple_elements = tuple_elements[0usize]
                                        .clone()
                                        .into_tuple()
                                        .expect(INTERNAL_ERR);
                                    (
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[0usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[2usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[3usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        tuple_elements[4usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                    )
                                },
                                tuple_elements[1usize]
                                    .clone()
                                    .into_address()
                                    .expect(INTERNAL_ERR)
                                    .as_bytes()
                                    .to_vec(),
                                {
                                    let mut result = [0u8; 32];
                                    let v = tuple_elements[2usize]
                                        .clone()
                                        .into_fixed_bytes()
                                        .expect(INTERNAL_ERR);
                                    result.copy_from_slice(&v);
                                    result
                                },
                            )
                        })
                        .collect(),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(
                &self,
                address: Vec<u8>,
            ) -> Option<
                Vec<
                    (
                        (
                            substreams::scalar::BigInt,
                            Vec<u8>,
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                            Vec<u8>,
                        ),
                        Vec<u8>,
                        [u8; 32usize],
                    ),
                >,
            > {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for MatchAdvancedOrders {
            const NAME: &'static str = "matchAdvancedOrders";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<
            Vec<
                (
                    (
                        substreams::scalar::BigInt,
                        Vec<u8>,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        Vec<u8>,
                    ),
                    Vec<u8>,
                    [u8; 32usize],
                ),
            >,
        > for MatchAdvancedOrders {
            fn output(
                data: &[u8],
            ) -> Result<
                Vec<
                    (
                        (
                            substreams::scalar::BigInt,
                            Vec<u8>,
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                            Vec<u8>,
                        ),
                        Vec<u8>,
                        [u8; 32usize],
                    ),
                >,
                String,
            > {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct MatchOrders {
            pub param0: Vec<
                (
                    (
                        Vec<u8>,
                        Vec<u8>,
                        Vec<
                            (
                                substreams::scalar::BigInt,
                                Vec<u8>,
                                substreams::scalar::BigInt,
                                substreams::scalar::BigInt,
                                substreams::scalar::BigInt,
                            ),
                        >,
                        Vec<
                            (
                                substreams::scalar::BigInt,
                                Vec<u8>,
                                substreams::scalar::BigInt,
                                substreams::scalar::BigInt,
                                substreams::scalar::BigInt,
                                Vec<u8>,
                            ),
                        >,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        [u8; 32usize],
                        substreams::scalar::BigInt,
                        [u8; 32usize],
                        substreams::scalar::BigInt,
                    ),
                    Vec<u8>,
                ),
            >,
            pub param1: Vec<
                (
                    Vec<(substreams::scalar::BigInt, substreams::scalar::BigInt)>,
                    Vec<(substreams::scalar::BigInt, substreams::scalar::BigInt)>,
                ),
            >,
        }
        impl MatchOrders {
            const METHOD_ID: [u8; 4] = [168u8, 23u8, 68u8, 4u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(
                                    ethabi::ParamType::Tuple(
                                        vec![
                                            ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                            ethabi::ParamType::Address,
                                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(8usize),
                                            ethabi::ParamType::Address,
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize)]))),
                                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(8usize),
                                            ethabi::ParamType::Address,
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Address]))),
                                            ethabi::ParamType::Uint(8usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::FixedBytes(32usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::FixedBytes(32usize),
                                            ethabi::ParamType::Uint(256usize)]),
                                            ethabi::ParamType::Bytes
                                        ],
                                    ),
                                ),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(
                                    ethabi::ParamType::Tuple(
                                        vec![
                                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize)]))),
                                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize)])))
                                        ],
                                    ),
                                ),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    param0: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                            (
                                {
                                    let tuple_elements = tuple_elements[0usize]
                                        .clone()
                                        .into_tuple()
                                        .expect(INTERNAL_ERR);
                                    (
                                        tuple_elements[0usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        tuple_elements[2usize]
                                            .clone()
                                            .into_array()
                                            .expect(INTERNAL_ERR)
                                            .into_iter()
                                            .map(|inner| {
                                                let tuple_elements = inner
                                                    .into_tuple()
                                                    .expect(INTERNAL_ERR);
                                                (
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[0usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_address()
                                                        .expect(INTERNAL_ERR)
                                                        .as_bytes()
                                                        .to_vec(),
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[2usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[3usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[4usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                )
                                            })
                                            .collect(),
                                        tuple_elements[3usize]
                                            .clone()
                                            .into_array()
                                            .expect(INTERNAL_ERR)
                                            .into_iter()
                                            .map(|inner| {
                                                let tuple_elements = inner
                                                    .into_tuple()
                                                    .expect(INTERNAL_ERR);
                                                (
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[0usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_address()
                                                        .expect(INTERNAL_ERR)
                                                        .as_bytes()
                                                        .to_vec(),
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[2usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[3usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[4usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    tuple_elements[5usize]
                                                        .clone()
                                                        .into_address()
                                                        .expect(INTERNAL_ERR)
                                                        .as_bytes()
                                                        .to_vec(),
                                                )
                                            })
                                            .collect(),
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[4usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[5usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[6usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut result = [0u8; 32];
                                            let v = tuple_elements[7usize]
                                                .clone()
                                                .into_fixed_bytes()
                                                .expect(INTERNAL_ERR);
                                            result.copy_from_slice(&v);
                                            result
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[8usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut result = [0u8; 32];
                                            let v = tuple_elements[9usize]
                                                .clone()
                                                .into_fixed_bytes()
                                                .expect(INTERNAL_ERR);
                                            result.copy_from_slice(&v);
                                            result
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[10usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                    )
                                },
                                tuple_elements[1usize]
                                    .clone()
                                    .into_bytes()
                                    .expect(INTERNAL_ERR),
                            )
                        })
                        .collect(),
                    param1: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                            (
                                tuple_elements[0usize]
                                    .clone()
                                    .into_array()
                                    .expect(INTERNAL_ERR)
                                    .into_iter()
                                    .map(|inner| {
                                        let tuple_elements = inner
                                            .into_tuple()
                                            .expect(INTERNAL_ERR);
                                        (
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                        )
                                    })
                                    .collect(),
                                tuple_elements[1usize]
                                    .clone()
                                    .into_array()
                                    .expect(INTERNAL_ERR)
                                    .into_iter()
                                    .map(|inner| {
                                        let tuple_elements = inner
                                            .into_tuple()
                                            .expect(INTERNAL_ERR);
                                        (
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[0usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                            {
                                                let mut v = [0 as u8; 32];
                                                tuple_elements[1usize]
                                                    .clone()
                                                    .into_uint()
                                                    .expect(INTERNAL_ERR)
                                                    .to_big_endian(v.as_mut_slice());
                                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                            },
                                        )
                                    })
                                    .collect(),
                            )
                        })
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let v = self
                                .param0
                                .iter()
                                .map(|inner| ethabi::Token::Tuple(
                                    vec![
                                        ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                        inner.0.0)),
                                        ethabi::Token::Address(ethabi::Address::from_slice(& inner.0
                                        .1)), { let v = inner.0.2.iter().map(| inner |
                                        ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                        .1)),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.2.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),)])).collect(); ethabi::Token::Array(v) }, {
                                        let v = inner.0.3.iter().map(| inner |
                                        ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                        .1)),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.2.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                        .5))])).collect(); ethabi::Token::Array(v) },
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.5.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.6.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),), ethabi::Token::FixedBytes(inner.0.7
                                        .as_ref().to_vec()),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.8.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),), ethabi::Token::FixedBytes(inner.0.9
                                        .as_ref().to_vec()),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.10.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),)]), ethabi::Token::Bytes(inner.1.clone())
                                    ],
                                ))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                        {
                            let v = self
                                .param1
                                .iter()
                                .map(|inner| ethabi::Token::Tuple(
                                    vec![
                                        { let v = inner.0.iter().map(| inner |
                                        ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),)])).collect(); ethabi::Token::Array(v) }, {
                                        let v = inner.1.iter().map(| inner |
                                        ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.1.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),)])).collect(); ethabi::Token::Array(v) }
                                    ],
                                ))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<
                Vec<
                    (
                        (
                            substreams::scalar::BigInt,
                            Vec<u8>,
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                            Vec<u8>,
                        ),
                        Vec<u8>,
                        [u8; 32usize],
                    ),
                >,
                String,
            > {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(
                data: &[u8],
            ) -> Result<
                Vec<
                    (
                        (
                            substreams::scalar::BigInt,
                            Vec<u8>,
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                            Vec<u8>,
                        ),
                        Vec<u8>,
                        [u8; 32usize],
                    ),
                >,
                String,
            > {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(
                                    ethabi::ParamType::Tuple(
                                        vec![
                                            ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(8usize),
                                            ethabi::ParamType::Address,
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Address]), ethabi::ParamType::Address,
                                            ethabi::ParamType::FixedBytes(32usize)
                                        ],
                                    ),
                                ),
                            ),
                        ],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                            (
                                {
                                    let tuple_elements = tuple_elements[0usize]
                                        .clone()
                                        .into_tuple()
                                        .expect(INTERNAL_ERR);
                                    (
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[0usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[2usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[3usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        tuple_elements[4usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                    )
                                },
                                tuple_elements[1usize]
                                    .clone()
                                    .into_address()
                                    .expect(INTERNAL_ERR)
                                    .as_bytes()
                                    .to_vec(),
                                {
                                    let mut result = [0u8; 32];
                                    let v = tuple_elements[2usize]
                                        .clone()
                                        .into_fixed_bytes()
                                        .expect(INTERNAL_ERR);
                                    result.copy_from_slice(&v);
                                    result
                                },
                            )
                        })
                        .collect(),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(
                &self,
                address: Vec<u8>,
            ) -> Option<
                Vec<
                    (
                        (
                            substreams::scalar::BigInt,
                            Vec<u8>,
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                            Vec<u8>,
                        ),
                        Vec<u8>,
                        [u8; 32usize],
                    ),
                >,
            > {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for MatchOrders {
            const NAME: &'static str = "matchOrders";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<
            Vec<
                (
                    (
                        substreams::scalar::BigInt,
                        Vec<u8>,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        Vec<u8>,
                    ),
                    Vec<u8>,
                    [u8; 32usize],
                ),
            >,
        > for MatchOrders {
            fn output(
                data: &[u8],
            ) -> Result<
                Vec<
                    (
                        (
                            substreams::scalar::BigInt,
                            Vec<u8>,
                            substreams::scalar::BigInt,
                            substreams::scalar::BigInt,
                            Vec<u8>,
                        ),
                        Vec<u8>,
                        [u8; 32usize],
                    ),
                >,
                String,
            > {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Name {}
        impl Name {
            const METHOD_ID: [u8; 4] = [6u8, 253u8, 222u8, 3u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Ok(Self {})
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(&[]);
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<String, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<String, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::String],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_string()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<String> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for Name {
            const NAME: &'static str = "name";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<String> for Name {
            fn output(data: &[u8]) -> Result<String, String> {
                Self::output(data)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct Validate {
            pub param0: Vec<
                (
                    (
                        Vec<u8>,
                        Vec<u8>,
                        Vec<
                            (
                                substreams::scalar::BigInt,
                                Vec<u8>,
                                substreams::scalar::BigInt,
                                substreams::scalar::BigInt,
                                substreams::scalar::BigInt,
                            ),
                        >,
                        Vec<
                            (
                                substreams::scalar::BigInt,
                                Vec<u8>,
                                substreams::scalar::BigInt,
                                substreams::scalar::BigInt,
                                substreams::scalar::BigInt,
                                Vec<u8>,
                            ),
                        >,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        [u8; 32usize],
                        substreams::scalar::BigInt,
                        [u8; 32usize],
                        substreams::scalar::BigInt,
                    ),
                    Vec<u8>,
                ),
            >,
        }
        impl Validate {
            const METHOD_ID: [u8; 4] = [136u8, 20u8, 119u8, 50u8];
            pub fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                let maybe_data = call.input.get(4..);
                if maybe_data.is_none() {
                    return Err("no data to decode".to_string());
                }
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(
                                    ethabi::ParamType::Tuple(
                                        vec![
                                            ethabi::ParamType::Tuple(vec![ethabi::ParamType::Address,
                                            ethabi::ParamType::Address,
                                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(8usize),
                                            ethabi::ParamType::Address,
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize)]))),
                                            ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(8usize),
                                            ethabi::ParamType::Address,
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Address]))),
                                            ethabi::ParamType::Uint(8usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::FixedBytes(32usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::FixedBytes(32usize),
                                            ethabi::ParamType::Uint(256usize)]),
                                            ethabi::ParamType::Bytes
                                        ],
                                    ),
                                ),
                            ),
                        ],
                        maybe_data.unwrap(),
                    )
                    .map_err(|e| format!("unable to decode call.input: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    param0: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                            (
                                {
                                    let tuple_elements = tuple_elements[0usize]
                                        .clone()
                                        .into_tuple()
                                        .expect(INTERNAL_ERR);
                                    (
                                        tuple_elements[0usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        tuple_elements[2usize]
                                            .clone()
                                            .into_array()
                                            .expect(INTERNAL_ERR)
                                            .into_iter()
                                            .map(|inner| {
                                                let tuple_elements = inner
                                                    .into_tuple()
                                                    .expect(INTERNAL_ERR);
                                                (
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[0usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_address()
                                                        .expect(INTERNAL_ERR)
                                                        .as_bytes()
                                                        .to_vec(),
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[2usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[3usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[4usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                )
                                            })
                                            .collect(),
                                        tuple_elements[3usize]
                                            .clone()
                                            .into_array()
                                            .expect(INTERNAL_ERR)
                                            .into_iter()
                                            .map(|inner| {
                                                let tuple_elements = inner
                                                    .into_tuple()
                                                    .expect(INTERNAL_ERR);
                                                (
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[0usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    tuple_elements[1usize]
                                                        .clone()
                                                        .into_address()
                                                        .expect(INTERNAL_ERR)
                                                        .as_bytes()
                                                        .to_vec(),
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[2usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[3usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    {
                                                        let mut v = [0 as u8; 32];
                                                        tuple_elements[4usize]
                                                            .clone()
                                                            .into_uint()
                                                            .expect(INTERNAL_ERR)
                                                            .to_big_endian(v.as_mut_slice());
                                                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                                    },
                                                    tuple_elements[5usize]
                                                        .clone()
                                                        .into_address()
                                                        .expect(INTERNAL_ERR)
                                                        .as_bytes()
                                                        .to_vec(),
                                                )
                                            })
                                            .collect(),
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[4usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[5usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[6usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut result = [0u8; 32];
                                            let v = tuple_elements[7usize]
                                                .clone()
                                                .into_fixed_bytes()
                                                .expect(INTERNAL_ERR);
                                            result.copy_from_slice(&v);
                                            result
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[8usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut result = [0u8; 32];
                                            let v = tuple_elements[9usize]
                                                .clone()
                                                .into_fixed_bytes()
                                                .expect(INTERNAL_ERR);
                                            result.copy_from_slice(&v);
                                            result
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[10usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                    )
                                },
                                tuple_elements[1usize]
                                    .clone()
                                    .into_bytes()
                                    .expect(INTERNAL_ERR),
                            )
                        })
                        .collect(),
                })
            }
            pub fn encode(&self) -> Vec<u8> {
                let data = ethabi::encode(
                    &[
                        {
                            let v = self
                                .param0
                                .iter()
                                .map(|inner| ethabi::Token::Tuple(
                                    vec![
                                        ethabi::Token::Tuple(vec![ethabi::Token::Address(ethabi::Address::from_slice(&
                                        inner.0.0)),
                                        ethabi::Token::Address(ethabi::Address::from_slice(& inner.0
                                        .1)), { let v = inner.0.2.iter().map(| inner |
                                        ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                        .1)),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.2.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),)])).collect(); ethabi::Token::Array(v) }, {
                                        let v = inner.0.3.iter().map(| inner |
                                        ethabi::Token::Tuple(vec![ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                        .1)),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.2.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.3.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Address(ethabi::Address::from_slice(& inner
                                        .5))])).collect(); ethabi::Token::Array(v) },
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.4.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.5.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.6.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),), ethabi::Token::FixedBytes(inner.0.7
                                        .as_ref().to_vec()),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.8.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),), ethabi::Token::FixedBytes(inner.0.9
                                        .as_ref().to_vec()),
                                        ethabi::Token::Uint(ethabi::Uint::from_big_endian(match
                                        inner.0.10.clone().to_bytes_be() { (num_bigint::Sign::Plus,
                                        bytes) => bytes, (num_bigint::Sign::NoSign, bytes) => bytes,
                                        (num_bigint::Sign::Minus, _) => {
                                        panic!("negative numbers are not supported") }, }
                                        .as_slice(),),)]), ethabi::Token::Bytes(inner.1.clone())
                                    ],
                                ))
                                .collect();
                            ethabi::Token::Array(v)
                        },
                    ],
                );
                let mut encoded = Vec::with_capacity(4 + data.len());
                encoded.extend(Self::METHOD_ID);
                encoded.extend(data);
                encoded
            }
            pub fn output_call(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<bool, String> {
                Self::output(call.return_data.as_ref())
            }
            pub fn output(data: &[u8]) -> Result<bool, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Bool],
                        data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode output data: {:?}", e))?;
                Ok(
                    values
                        .pop()
                        .expect("one output data should have existed")
                        .into_bool()
                        .expect(INTERNAL_ERR),
                )
            }
            pub fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                match call.input.get(0..4) {
                    Some(signature) => Self::METHOD_ID == signature,
                    None => false,
                }
            }
            pub fn call(&self, address: Vec<u8>) -> Option<bool> {
                use substreams_ethereum::pb::eth::rpc;
                let rpc_calls = rpc::RpcCalls {
                    calls: vec![
                        rpc::RpcCall { to_addr : address, data : self.encode(), }
                    ],
                };
                let responses = substreams_ethereum::rpc::eth_call(&rpc_calls).responses;
                let response = responses
                    .get(0)
                    .expect("one response should have existed");
                if response.failed {
                    return None;
                }
                match Self::output(response.raw.as_ref()) {
                    Ok(data) => Some(data),
                    Err(err) => {
                        use substreams_ethereum::Function;
                        substreams::log::info!(
                            "Call output for function `{}` failed to decode with error: {}",
                            Self::NAME, err
                        );
                        None
                    }
                }
            }
        }
        impl substreams_ethereum::Function for Validate {
            const NAME: &'static str = "validate";
            fn match_call(call: &substreams_ethereum::pb::eth::v2::Call) -> bool {
                Self::match_call(call)
            }
            fn decode(
                call: &substreams_ethereum::pb::eth::v2::Call,
            ) -> Result<Self, String> {
                Self::decode(call)
            }
            fn encode(&self) -> Vec<u8> {
                self.encode()
            }
        }
        impl substreams_ethereum::rpc::RPCDecodable<bool> for Validate {
            fn output(data: &[u8]) -> Result<bool, String> {
                Self::output(data)
            }
        }
    }
    /// Contract's events.
    #[allow(dead_code, unused_imports, unused_variables)]
    pub mod events {
        use super::INTERNAL_ERR;
        #[derive(Debug, Clone, PartialEq)]
        pub struct CounterIncremented {
            pub new_counter: substreams::scalar::BigInt,
            pub offerer: Vec<u8>,
        }
        impl CounterIncremented {
            const TOPIC_ID: [u8; 32] = [
                114u8,
                28u8,
                32u8,
                18u8,
                18u8,
                151u8,
                81u8,
                43u8,
                114u8,
                130u8,
                27u8,
                151u8,
                245u8,
                50u8,
                104u8,
                119u8,
                234u8,
                142u8,
                207u8,
                75u8,
                185u8,
                148u8,
                143u8,
                234u8,
                91u8,
                252u8,
                182u8,
                69u8,
                48u8,
                116u8,
                211u8,
                127u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 2usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::Uint(256usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    offerer: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'offerer' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    new_counter: {
                        let mut v = [0 as u8; 32];
                        values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_uint()
                            .expect(INTERNAL_ERR)
                            .to_big_endian(v.as_mut_slice());
                        substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                    },
                })
            }
        }
        impl substreams_ethereum::Event for CounterIncremented {
            const NAME: &'static str = "CounterIncremented";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct OrderCancelled {
            pub order_hash: [u8; 32usize],
            pub offerer: Vec<u8>,
            pub zone: Vec<u8>,
        }
        impl OrderCancelled {
            const TOPIC_ID: [u8; 32] = [
                107u8,
                172u8,
                192u8,
                29u8,
                190u8,
                68u8,
                36u8,
                150u8,
                6u8,
                143u8,
                125u8,
                35u8,
                78u8,
                221u8,
                129u8,
                31u8,
                26u8,
                95u8,
                131u8,
                50u8,
                67u8,
                224u8,
                174u8,
                200u8,
                36u8,
                248u8,
                106u8,
                184u8,
                97u8,
                243u8,
                201u8,
                13u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() != 32usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[ethabi::ParamType::FixedBytes(32usize)],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    offerer: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'offerer' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    zone: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'zone' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    order_hash: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                })
            }
        }
        impl substreams_ethereum::Event for OrderCancelled {
            const NAME: &'static str = "OrderCancelled";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct OrderFulfilled {
            pub order_hash: [u8; 32usize],
            pub offerer: Vec<u8>,
            pub zone: Vec<u8>,
            pub recipient: Vec<u8>,
            pub offer: Vec<
                (
                    substreams::scalar::BigInt,
                    Vec<u8>,
                    substreams::scalar::BigInt,
                    substreams::scalar::BigInt,
                ),
            >,
            pub consideration: Vec<
                (
                    substreams::scalar::BigInt,
                    Vec<u8>,
                    substreams::scalar::BigInt,
                    substreams::scalar::BigInt,
                    Vec<u8>,
                ),
            >,
        }
        impl OrderFulfilled {
            const TOPIC_ID: [u8; 32] = [
                157u8,
                154u8,
                248u8,
                227u8,
                141u8,
                102u8,
                198u8,
                46u8,
                44u8,
                18u8,
                240u8,
                34u8,
                82u8,
                73u8,
                253u8,
                157u8,
                114u8,
                28u8,
                84u8,
                184u8,
                63u8,
                72u8,
                217u8,
                53u8,
                44u8,
                151u8,
                198u8,
                202u8,
                205u8,
                203u8,
                111u8,
                49u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 3usize {
                    return false;
                }
                if log.data.len() < 192usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::Address,
                            ethabi::ParamType::Array(
                                Box::new(
                                    ethabi::ParamType::Tuple(
                                        vec![
                                            ethabi::ParamType::Uint(8usize), ethabi::ParamType::Address,
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize)
                                        ],
                                    ),
                                ),
                            ),
                            ethabi::ParamType::Array(
                                Box::new(
                                    ethabi::ParamType::Tuple(
                                        vec![
                                            ethabi::ParamType::Uint(8usize), ethabi::ParamType::Address,
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Uint(256usize),
                                            ethabi::ParamType::Address
                                        ],
                                    ),
                                ),
                            ),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    offerer: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[1usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'offerer' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    zone: ethabi::decode(
                            &[ethabi::ParamType::Address],
                            log.topics[2usize].as_ref(),
                        )
                        .map_err(|e| {
                            format!(
                                "unable to decode param 'zone' from topic of type 'address': {:?}",
                                e
                            )
                        })?
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    order_hash: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    recipient: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_address()
                        .expect(INTERNAL_ERR)
                        .as_bytes()
                        .to_vec(),
                    offer: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                            (
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                },
                                tuple_elements[1usize]
                                    .clone()
                                    .into_address()
                                    .expect(INTERNAL_ERR)
                                    .as_bytes()
                                    .to_vec(),
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[2usize]
                                        .clone()
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                },
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[3usize]
                                        .clone()
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                },
                            )
                        })
                        .collect(),
                    consideration: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            let tuple_elements = inner.into_tuple().expect(INTERNAL_ERR);
                            (
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[0usize]
                                        .clone()
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                },
                                tuple_elements[1usize]
                                    .clone()
                                    .into_address()
                                    .expect(INTERNAL_ERR)
                                    .as_bytes()
                                    .to_vec(),
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[2usize]
                                        .clone()
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                },
                                {
                                    let mut v = [0 as u8; 32];
                                    tuple_elements[3usize]
                                        .clone()
                                        .into_uint()
                                        .expect(INTERNAL_ERR)
                                        .to_big_endian(v.as_mut_slice());
                                    substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                },
                                tuple_elements[4usize]
                                    .clone()
                                    .into_address()
                                    .expect(INTERNAL_ERR)
                                    .as_bytes()
                                    .to_vec(),
                            )
                        })
                        .collect(),
                })
            }
        }
        impl substreams_ethereum::Event for OrderFulfilled {
            const NAME: &'static str = "OrderFulfilled";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct OrderValidated {
            pub order_hash: [u8; 32usize],
            pub order_parameters: (
                Vec<u8>,
                Vec<u8>,
                Vec<
                    (
                        substreams::scalar::BigInt,
                        Vec<u8>,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                    ),
                >,
                Vec<
                    (
                        substreams::scalar::BigInt,
                        Vec<u8>,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        substreams::scalar::BigInt,
                        Vec<u8>,
                    ),
                >,
                substreams::scalar::BigInt,
                substreams::scalar::BigInt,
                substreams::scalar::BigInt,
                [u8; 32usize],
                substreams::scalar::BigInt,
                [u8; 32usize],
                substreams::scalar::BigInt,
            ),
        }
        impl OrderValidated {
            const TOPIC_ID: [u8; 32] = [
                242u8,
                128u8,
                121u8,
                30u8,
                254u8,
                120u8,
                46u8,
                220u8,
                240u8,
                108u8,
                225u8,
                92u8,
                143u8,
                77u8,
                255u8,
                23u8,
                96u8,
                29u8,
                179u8,
                184u8,
                142u8,
                179u8,
                128u8,
                90u8,
                13u8,
                183u8,
                215u8,
                127u8,
                175u8,
                117u8,
                127u8,
                4u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() < 448usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::FixedBytes(32usize),
                            ethabi::ParamType::Tuple(
                                vec![
                                    ethabi::ParamType::Address, ethabi::ParamType::Address,
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(8usize),
                                    ethabi::ParamType::Address,
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize)]))),
                                    ethabi::ParamType::Array(Box::new(ethabi::ParamType::Tuple(vec![ethabi::ParamType::Uint(8usize),
                                    ethabi::ParamType::Address,
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Address]))),
                                    ethabi::ParamType::Uint(8usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::FixedBytes(32usize),
                                    ethabi::ParamType::Uint(256usize),
                                    ethabi::ParamType::FixedBytes(32usize),
                                    ethabi::ParamType::Uint(256usize)
                                ],
                            ),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    order_hash: {
                        let mut result = [0u8; 32];
                        let v = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_fixed_bytes()
                            .expect(INTERNAL_ERR);
                        result.copy_from_slice(&v);
                        result
                    },
                    order_parameters: {
                        let tuple_elements = values
                            .pop()
                            .expect(INTERNAL_ERR)
                            .into_tuple()
                            .expect(INTERNAL_ERR);
                        (
                            tuple_elements[0usize]
                                .clone()
                                .into_address()
                                .expect(INTERNAL_ERR)
                                .as_bytes()
                                .to_vec(),
                            tuple_elements[1usize]
                                .clone()
                                .into_address()
                                .expect(INTERNAL_ERR)
                                .as_bytes()
                                .to_vec(),
                            tuple_elements[2usize]
                                .clone()
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| {
                                    let tuple_elements = inner
                                        .into_tuple()
                                        .expect(INTERNAL_ERR);
                                    (
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[0usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[2usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[3usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[4usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                    )
                                })
                                .collect(),
                            tuple_elements[3usize]
                                .clone()
                                .into_array()
                                .expect(INTERNAL_ERR)
                                .into_iter()
                                .map(|inner| {
                                    let tuple_elements = inner
                                        .into_tuple()
                                        .expect(INTERNAL_ERR);
                                    (
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[0usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        tuple_elements[1usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[2usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[3usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        {
                                            let mut v = [0 as u8; 32];
                                            tuple_elements[4usize]
                                                .clone()
                                                .into_uint()
                                                .expect(INTERNAL_ERR)
                                                .to_big_endian(v.as_mut_slice());
                                            substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                                        },
                                        tuple_elements[5usize]
                                            .clone()
                                            .into_address()
                                            .expect(INTERNAL_ERR)
                                            .as_bytes()
                                            .to_vec(),
                                    )
                                })
                                .collect(),
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[4usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[5usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[6usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            {
                                let mut result = [0u8; 32];
                                let v = tuple_elements[7usize]
                                    .clone()
                                    .into_fixed_bytes()
                                    .expect(INTERNAL_ERR);
                                result.copy_from_slice(&v);
                                result
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[8usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                            {
                                let mut result = [0u8; 32];
                                let v = tuple_elements[9usize]
                                    .clone()
                                    .into_fixed_bytes()
                                    .expect(INTERNAL_ERR);
                                result.copy_from_slice(&v);
                                result
                            },
                            {
                                let mut v = [0 as u8; 32];
                                tuple_elements[10usize]
                                    .clone()
                                    .into_uint()
                                    .expect(INTERNAL_ERR)
                                    .to_big_endian(v.as_mut_slice());
                                substreams::scalar::BigInt::from_unsigned_bytes_be(&v)
                            },
                        )
                    },
                })
            }
        }
        impl substreams_ethereum::Event for OrderValidated {
            const NAME: &'static str = "OrderValidated";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
        #[derive(Debug, Clone, PartialEq)]
        pub struct OrdersMatched {
            pub order_hashes: Vec<[u8; 32usize]>,
        }
        impl OrdersMatched {
            const TOPIC_ID: [u8; 32] = [
                75u8,
                159u8,
                45u8,
                54u8,
                225u8,
                180u8,
                201u8,
                61u8,
                230u8,
                44u8,
                192u8,
                119u8,
                176u8,
                11u8,
                26u8,
                145u8,
                216u8,
                75u8,
                108u8,
                49u8,
                180u8,
                161u8,
                78u8,
                1u8,
                39u8,
                24u8,
                220u8,
                202u8,
                35u8,
                6u8,
                137u8,
                231u8,
            ];
            pub fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                if log.topics.len() != 1usize {
                    return false;
                }
                if log.data.len() < 64usize {
                    return false;
                }
                return log.topics.get(0).expect("bounds already checked").as_ref()
                    == Self::TOPIC_ID;
            }
            pub fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                let mut values = ethabi::decode(
                        &[
                            ethabi::ParamType::Array(
                                Box::new(ethabi::ParamType::FixedBytes(32usize)),
                            ),
                        ],
                        log.data.as_ref(),
                    )
                    .map_err(|e| format!("unable to decode log.data: {:?}", e))?;
                values.reverse();
                Ok(Self {
                    order_hashes: values
                        .pop()
                        .expect(INTERNAL_ERR)
                        .into_array()
                        .expect(INTERNAL_ERR)
                        .into_iter()
                        .map(|inner| {
                            let mut result = [0u8; 32];
                            let v = inner.into_fixed_bytes().expect(INTERNAL_ERR);
                            result.copy_from_slice(&v);
                            result
                        })
                        .collect(),
                })
            }
        }
        impl substreams_ethereum::Event for OrdersMatched {
            const NAME: &'static str = "OrdersMatched";
            fn match_log(log: &substreams_ethereum::pb::eth::v2::Log) -> bool {
                Self::match_log(log)
            }
            fn decode(
                log: &substreams_ethereum::pb::eth::v2::Log,
            ) -> Result<Self, String> {
                Self::decode(log)
            }
        }
    }