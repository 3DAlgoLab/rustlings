#[cfg(test)]
mod test {
    use super::*;

    use ethers::types::Address;
    use std::str::FromStr;

    trait EthereumAddress {
        fn convert_address(&self) -> Result<Address, &'static str>;
    }

    impl EthereumAddress for &str {
        fn convert_address(&self) -> Result<Address, &'static str> {
            match Address::from_str(self) {
                Ok(address) => Ok(address),
                Err(_) => Err("Invalid Ethereum Address String"),
            }
        }
    }

    impl EthereumAddress for Address {
        fn convert_address(&self) -> Result<Address, &'static str> {
            Ok(*self)
        }
    }

    fn get_ethereum_data<T: EthereumAddress>(address: T) -> Address {
        let converted_address = address.convert_address().unwrap();
        converted_address
    }

    #[test]
    fn test_polymorphism() {
        let addr = Address::from_str("0xB0E0698F196E16cd353D409fb19E3536076B7CaE").unwrap();
        dbg!(addr);
    }
}
