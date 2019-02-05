//! Koleksi macro library internal

macro_rules! implement_crypto_wrapper {
    ( $(#[$attr:meta])*  struct $name:ident, $size:expr) => {
        implement_crypto_wrapper!( $(#[$attr])* struct $name, $crate::crypto::ds::$name, $name, $size );
    };
    ( $(#[$attr:meta])* struct $name:ident, $source:path, $source_name:ident, $size:expr) => {
        /// Crypto object wrapper
        #[derive(Clone, PartialEq, Eq)]
        $(#[$attr])*
        pub struct $name($source);

        impl $name {
            #[doc(hidden)]
            pub fn new(bytes_array: [u8; $size]) -> Self {
                $name($source(bytes_array))
            }

            /// Creates new instance from bytes slice.
            pub fn from_slice(bytes: &[u8]) -> Option<Self> {
                use $source;
                $source_name::from_slice(bytes).map(Self)
            }

            /// Convert to hex string
            pub fn to_hex(&self) -> String {
                hex::encode(&self.0[..])
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}({}..)", stringify!($name), &self.to_hex()[..8])
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}({}..)", stringify!($name), &self.to_hex()[..8])
            }
        }

        impl ::hex::FromHex for $name {
            type Error = ::hex::FromHexError;

            fn from_hex<T: AsRef<[u8]>>(v: T) -> Result<Self, Self::Error> {
                let bytes = Vec::<u8>::from_hex(v)?;
                if let Some(self_value) = Self::from_slice(bytes.as_ref()) {
                    Ok(self_value)
                } else {
                    Err(::hex::FromHexError::InvalidStringLength)
                }
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = ::hex::FromHexError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                use hex::FromHex;
                $name::from_hex(s)
            }
        }
    };
}

macro_rules! api_endpoint {
    ($name:ident, $qt:ty, $rv:ty, (|$schema:ident, $query:ident| $( $cs:tt )+ ) ) => {
        pub fn $name(state: &AppState, $query: $qt) -> ApiResult<$rv> {
            let $schema = Schema::new(state.db());

            {$($cs)+}
        }
    };
}

macro_rules! api_tx_endpoint {
    ($name:ident, $qt:ty, $rv:ty, (|$schema:ident, $query:ident| $( $cs:tt )+ ) ) => {
        pub fn $name(state: &AppState, $query: TxQuery<$qt>) -> ApiResult<$rv> {
            let $schema = Schema::new(state.db());

            {$($cs)+}
        }
    };
}
