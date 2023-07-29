///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemWhRatio {
    ///
    #[prost(int32, tag = "1")]
    pub ratio: i32,
    ///
    #[prost(int32, tag = "2")]
    pub width: i32,
    ///
    #[prost(int32, tag = "3")]
    pub height: i32,
}
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WhRatio {
    WHRatio11 = 0,
    WHRatio169 = 1,
    WHRatio34 = 2,
    WHRatioCustom = 3,
}
impl WhRatio {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            WhRatio::WHRatio11 => "W_H_RATIO_1_1",
            WhRatio::WHRatio169 => "W_H_RATIO_16_9",
            WhRatio::WHRatio34 => "W_H_RATIO_3_4",
            WhRatio::WHRatioCustom => "W_H_RATIO_CUSTOM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "W_H_RATIO_1_1" => Some(Self::WHRatio11),
            "W_H_RATIO_16_9" => Some(Self::WHRatio169),
            "W_H_RATIO_3_4" => Some(Self::WHRatio34),
            "W_H_RATIO_CUSTOM" => Some(Self::WHRatioCustom),
            _ => None,
        }
    }
}
