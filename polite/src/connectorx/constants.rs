// #[cfg(feature = "dst_arrow")]
use arrow::datatypes::DataType as ArrowDataType;

// #[cfg(feature = "dst_arrow")]
pub const DEFAULT_ARROW_DECIMAL_PRECISION: u8 = 38;

// #[cfg(feature = "dst_arrow")]
pub const DEFAULT_ARROW_DECIMAL_SCALE: i8 = 10;

// #[cfg(feature = "dst_arrow")]
pub const DEFAULT_ARROW_DECIMAL: ArrowDataType =
    ArrowDataType::Decimal128(DEFAULT_ARROW_DECIMAL_PRECISION, DEFAULT_ARROW_DECIMAL_SCALE);

// #[cfg(feature = "dst_arrow")]
pub(crate) const SECONDS_IN_DAY: i64 = 86_400;

#[allow(dead_code)]
const KILO: usize = 1 << 10;

// #[cfg(feature = "dst_arrow")]
pub const RECORD_BATCH_SIZE: usize = 64 * KILO;

pub const CONNECTORX_PROTOCOL: &str = "cxprotocol";
