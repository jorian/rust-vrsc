extern crate serde;
// extern crate serde_json;
// extern crate serde_test;

pub mod util {
    pub mod key {
        use std::fmt::{self, Write};
        use secp256k1::{self, Secp256k1};
        use bitcoin::util::base58;
        use serde::Deserializer;
        use serde::de::Visitor;
        use bitcoin::hashes::core::fmt::Formatter;
        use std::str::FromStr;
        use std::error;

        #[derive(Debug)]
        pub enum Error {
            Base58(base58::Error),
            Secp256k1(secp256k1::Error)
        }

        impl fmt::Display for Error {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match *self {
                    Error::Base58(ref e) => write!(f, "base58 error: {}", e),
                    Error::Secp256k1(ref e) => write!(f, "secp256k1 error: {}", e),
                }
            }
        }

        impl error::Error for Error {
            fn cause(&self) -> Option<&dyn error::Error> {
                match *self {
                    Error::Base58(ref e) => Some(e),
                    Error::Secp256k1(ref e) => Some(e),
                }
            }
        }

        #[doc(hidden)]
        impl From<base58::Error> for Error {
            fn from(e: base58::Error) -> Error {
                Error::Base58(e)
            }
        }

        #[doc(hidden)]
        impl From<secp256k1::Error> for Error {
            fn from(e: secp256k1::Error) -> Error {
                Error::Secp256k1(e)
            }
        }

        pub struct PrivateKey {
            pub compressed: bool,
            pub key: secp256k1::SecretKey
        }

        impl PrivateKey {
            pub fn public_key<C: secp256k1::Signing>(&self, secp: &Secp256k1<C>) -> PublicKey {
                unimplemented!()
            }

            pub fn fmt_wif(&self, fmt: &mut dyn fmt::Write) -> fmt::Result {
                let mut ret = [0; 34];
                ret[0] = 188;
                ret[1..33].copy_from_slice(&self.key[..]);
                let privkey = if self.compressed {
                    ret[33] = 1;
                    base58::check_encode_slice(&ret[..])
                } else {
                    base58::check_encode_slice(&ret[..33])
                };
                fmt.write_str(&privkey)
            }

            pub fn from_wif(wif: &str) -> Result<PrivateKey, Error> {
                let data = base58::from(wif)?;

                let compressed = match data.len() {
                    33 => false,
                    34 => true,
                    _ => { return Err(Error::Base58(base58::Error::InvalidLength(data.len())))}
                };

                Ok(PrivateKey {
                    compressed,
                    key: secp256k1::SecretKey::from_slice(&data[1..33])?
                })
            }
        }

        impl fmt::Display for PrivateKey {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                self.fmt_wif(f)
            }
        }

        impl fmt::Debug for PrivateKey {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "[private key data]")
            }
        }

        impl FromStr for PrivateKey {
            type Err = bitcoin::util::key::Error;
            fn from_str(s: &str) -> Result<PrivateKey, bitcoin::util::key::Error> {
                PrivateKey::from_wif(s)
            }
        }

        impl ::serde::Serialize for PrivateKey {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
                S: ::serde::Serializer {
                serializer.collect_str(self)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for PrivateKey {
            fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error> where
                D: Deserializer<'de> {
                struct WifVisitor;

                impl<'de> ::serde::de::Visitor<'de> for WifVisitor {
                    type Value = PrivateKey;

                    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                        formatter.write_str("an ASCII WIF string")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where
                        E: ::serde::de::Error, {
                        if let Ok(s) = ::std::str::from_utf8(v) {

                        }
                    }

                    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> where
                        E: ::serde::de::Error, {
                        if let Ok(s) = ::std::str::from_utf8(v) {
                            PrivateKey::from_str(s).map_err(E::custom)
                        }
                    }
                }
        }

        }

        pub struct PublicKey {
            pub compressed: bool,
            pub key: secp256k1::PublicKey
        }

        impl PublicKey {
            pub fn to_wif(&self) -> String {
                unimplemented!()
            }
        }
    }
}

pub use util::key::PublicKey;
pub use util::key::PrivateKey;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
