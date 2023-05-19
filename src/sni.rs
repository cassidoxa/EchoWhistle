// This module auto-generated from the sni proto
// https://github.com/alttpo/sni

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DevicesRequest {
    /// optional list of device kind filters
    ///
    /// TODO: repeated DeviceCapability capabilities;
    #[prost(string, repeated, tag = "1")]
    pub kinds: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DevicesResponse {
    #[prost(message, repeated, tag = "1")]
    pub devices: ::prost::alloc::vec::Vec<devices_response::Device>,
}
/// Nested message and enum types in `DevicesResponse`.
pub mod devices_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Device {
        /// URI that describes exactly how to connect to the device, e.g.:
        /// RetroArch:  "ra://127.0.0.1:55355"
        /// FX Pak Pro: "fxpakpro://./dev/cu.usbmodemDEMO000000001" (MacOS)
        ///              "fxpakpro://./COM4"                         (Windows)
        ///              "fxpakpro://./dev/ttyACM0"                  (Linux)
        /// uri is used as the unique identifier of the device for clients to refer to
        #[prost(string, tag = "1")]
        pub uri: ::prost::alloc::string::String,
        /// friendly display name of the device
        #[prost(string, tag = "2")]
        pub display_name: ::prost::alloc::string::String,
        /// device kind, e.g. "fxpakpro", "retroarch", "lua"
        #[prost(string, tag = "3")]
        pub kind: ::prost::alloc::string::String,
        /// all device capabilities:
        #[prost(enumeration = "super::DeviceCapability", repeated, tag = "4")]
        pub capabilities: ::prost::alloc::vec::Vec<i32>,
        /// default address space for the device:
        #[prost(enumeration = "super::AddressSpace", tag = "5")]
        pub default_address_space: i32,
        /// \[DEPRECATED\] console system supported, e.g. "snes", "n64"
        /// since devices can support multiple systems, it's better to fetch platform from DeviceInfo.FetchFields method
        #[deprecated]
        #[prost(string, tag = "6")]
        pub system: ::prost::alloc::string::String,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetSystemRequest {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetSystemResponse {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetToMenuRequest {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetToMenuResponse {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PauseEmulationRequest {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    /// true to pause emulation, false to unpause
    #[prost(bool, tag = "2")]
    pub paused: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PauseEmulationResponse {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub paused: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PauseToggleEmulationRequest {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PauseToggleEmulationResponse {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetectMemoryMappingRequest {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    /// optional fallback value to set in case detection fails for some reason
    #[prost(enumeration = "MemoryMapping", optional, tag = "2")]
    pub fallback_memory_mapping: ::core::option::Option<i32>,
    /// optional ROM header (from bus address $00:FFB0, at least $30 bytes long) to use for detection
    /// if not provided, the header will be read from the device
    #[prost(bytes = "vec", optional, tag = "3")]
    pub rom_header00_ffb0: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetectMemoryMappingResponse {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    /// the memory mapping mode detected
    #[prost(enumeration = "MemoryMapping", tag = "2")]
    pub memory_mapping: i32,
    /// true if confident we detected a mapping; false if using a fallback or default value
    #[prost(bool, tag = "3")]
    pub confidence: bool,
    /// the ROM header read from $00:FFB0, length is $50 bytes if server reads it, otherwise length of `romHeader00FFB0` from request
    #[prost(bytes = "vec", tag = "4")]
    pub rom_header00_ffb0: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadMemoryRequest {
    #[prost(uint32, tag = "1")]
    pub request_address: u32,
    #[prost(enumeration = "AddressSpace", tag = "2")]
    pub request_address_space: i32,
    #[prost(enumeration = "MemoryMapping", tag = "4")]
    pub request_memory_mapping: i32,
    #[prost(uint32, tag = "3")]
    pub size: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadMemoryResponse {
    #[prost(uint32, tag = "1")]
    pub request_address: u32,
    #[prost(enumeration = "AddressSpace", tag = "2")]
    pub request_address_space: i32,
    #[prost(enumeration = "MemoryMapping", tag = "6")]
    pub request_memory_mapping: i32,
    /// the address sent to the device and its space
    #[prost(uint32, tag = "3")]
    pub device_address: u32,
    #[prost(enumeration = "AddressSpace", tag = "4")]
    pub device_address_space: i32,
    #[prost(bytes = "vec", tag = "5")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteMemoryRequest {
    #[prost(uint32, tag = "1")]
    pub request_address: u32,
    #[prost(enumeration = "AddressSpace", tag = "2")]
    pub request_address_space: i32,
    #[prost(enumeration = "MemoryMapping", tag = "4")]
    pub request_memory_mapping: i32,
    #[prost(bytes = "vec", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteMemoryResponse {
    #[prost(uint32, tag = "1")]
    pub request_address: u32,
    #[prost(enumeration = "AddressSpace", tag = "2")]
    pub request_address_space: i32,
    #[prost(enumeration = "MemoryMapping", tag = "6")]
    pub request_memory_mapping: i32,
    #[prost(uint32, tag = "3")]
    pub device_address: u32,
    #[prost(enumeration = "AddressSpace", tag = "4")]
    pub device_address_space: i32,
    #[prost(uint32, tag = "5")]
    pub size: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SingleReadMemoryRequest {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub request: ::core::option::Option<ReadMemoryRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SingleReadMemoryResponse {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub response: ::core::option::Option<ReadMemoryResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SingleWriteMemoryRequest {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub request: ::core::option::Option<WriteMemoryRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SingleWriteMemoryResponse {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub response: ::core::option::Option<WriteMemoryResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiReadMemoryRequest {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub requests: ::prost::alloc::vec::Vec<ReadMemoryRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiReadMemoryResponse {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub responses: ::prost::alloc::vec::Vec<ReadMemoryResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiWriteMemoryRequest {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub requests: ::prost::alloc::vec::Vec<WriteMemoryRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiWriteMemoryResponse {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub responses: ::prost::alloc::vec::Vec<WriteMemoryResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadDirectoryRequest {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirEntry {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration = "DirEntryType", tag = "2")]
    pub r#type: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadDirectoryResponse {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub entries: ::prost::alloc::vec::Vec<DirEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MakeDirectoryRequest {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MakeDirectoryResponse {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveFileRequest {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveFileResponse {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameFileRequest {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub new_filename: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameFileResponse {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub new_filename: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutFileRequest {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutFileResponse {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub size: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFileRequest {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFileResponse {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub size: u32,
    #[prost(bytes = "vec", tag = "4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BootFileRequest {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BootFileResponse {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldsRequest {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    #[prost(enumeration = "Field", repeated, tag = "2")]
    pub fields: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldsResponse {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    #[prost(enumeration = "Field", repeated, tag = "2")]
    pub fields: ::prost::alloc::vec::Vec<i32>,
    #[prost(string, repeated, tag = "3")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NwaCommandRequest {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    /// emu-nwaccess command name:
    #[prost(string, tag = "2")]
    pub command: ::prost::alloc::string::String,
    /// command arguments:
    #[prost(string, tag = "3")]
    pub args: ::prost::alloc::string::String,
    /// an optional binary argument:
    #[prost(bytes = "vec", optional, tag = "4")]
    pub binary_arg: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NwaCommandResponse {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub ascii_reply: ::prost::alloc::vec::Vec<nwa_command_response::NwaasciiItem>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub binary_replay: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// Nested message and enum types in `NWACommandResponse`.
pub mod nwa_command_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NwaasciiItem {
        #[prost(map = "string, string", tag = "1")]
        pub item: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
    }
}
/// address space used to interpret an address in:
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AddressSpace {
    /// The default is the FX Pak Pro / SD2SNES's address space:
    /// $00_0000..$DF_FFFF =   ROM contents, linearly mapped
    /// $E0_0000..$EF_FFFF =  SRAM contents, linearly mapped
    /// $F5_0000..$F6_FFFF =  WRAM contents, linearly mapped
    /// $F7_0000..$F7_FFFF =  VRAM contents, linearly mapped
    /// $F8_0000..$F8_FFFF =   APU contents, linearly mapped
    /// $F9_0000..$F9_01FF = CGRAM contents, linearly mapped
    /// $F9_0200..$F9_041F =   OAM contents, linearly mapped
    /// $F9_0420..$F9_04FF =  MISC contents, linearly mapped
    /// $F9_0500..$F9_06FF =         PPUREG, linearly mapped
    /// $F9_0700..$F9_08FF =         CPUREG, linearly mapped
    /// translated device address depends on device being talked to and its current MemoryMapping mode
    ///
    /// SNES
    FxPakPro = 0,
    /// The SNES's main A-bus; address depends on device's current MemoryMapping mode, e.g. LoROM, HiROM, ExHiROM, etc.
    SnesABus = 1,
    /// Do not do any address translation; simply pass the raw address to the device as-is:
    Raw = 2,
}
impl AddressSpace {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AddressSpace::FxPakPro => "FxPakPro",
            AddressSpace::SnesABus => "SnesABus",
            AddressSpace::Raw => "Raw",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "FxPakPro" => Some(Self::FxPakPro),
            "SnesABus" => Some(Self::SnesABus),
            "Raw" => Some(Self::Raw),
            _ => None,
        }
    }
}
/// memory mapping mode of a SNES cart:
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MemoryMapping {
    Unknown = 0,
    HiRom = 1,
    LoRom = 2,
    /// (48-64Mbit)
    ExHiRom = 3,
    /// TODO: BSX = 5;
    Sa1 = 4,
}
impl MemoryMapping {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MemoryMapping::Unknown => "Unknown",
            MemoryMapping::HiRom => "HiROM",
            MemoryMapping::LoRom => "LoROM",
            MemoryMapping::ExHiRom => "ExHiROM",
            MemoryMapping::Sa1 => "SA1",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Unknown" => Some(Self::Unknown),
            "HiROM" => Some(Self::HiRom),
            "LoROM" => Some(Self::LoRom),
            "ExHiROM" => Some(Self::ExHiRom),
            "SA1" => Some(Self::Sa1),
            _ => None,
        }
    }
}
/// capabilities of a device
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DeviceCapability {
    None = 0,
    ReadMemory = 1,
    WriteMemory = 2,
    ExecuteAsm = 3,
    ResetSystem = 4,
    PauseUnpauseEmulation = 5,
    PauseToggleEmulation = 6,
    ResetToMenu = 7,
    FetchFields = 8,
    ReadDirectory = 10,
    MakeDirectory = 11,
    RemoveFile = 12,
    RenameFile = 13,
    PutFile = 14,
    GetFile = 15,
    BootFile = 16,
    NwaCommand = 20,
}
impl DeviceCapability {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DeviceCapability::None => "None",
            DeviceCapability::ReadMemory => "ReadMemory",
            DeviceCapability::WriteMemory => "WriteMemory",
            DeviceCapability::ExecuteAsm => "ExecuteASM",
            DeviceCapability::ResetSystem => "ResetSystem",
            DeviceCapability::PauseUnpauseEmulation => "PauseUnpauseEmulation",
            DeviceCapability::PauseToggleEmulation => "PauseToggleEmulation",
            DeviceCapability::ResetToMenu => "ResetToMenu",
            DeviceCapability::FetchFields => "FetchFields",
            DeviceCapability::ReadDirectory => "ReadDirectory",
            DeviceCapability::MakeDirectory => "MakeDirectory",
            DeviceCapability::RemoveFile => "RemoveFile",
            DeviceCapability::RenameFile => "RenameFile",
            DeviceCapability::PutFile => "PutFile",
            DeviceCapability::GetFile => "GetFile",
            DeviceCapability::BootFile => "BootFile",
            DeviceCapability::NwaCommand => "NWACommand",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "None" => Some(Self::None),
            "ReadMemory" => Some(Self::ReadMemory),
            "WriteMemory" => Some(Self::WriteMemory),
            "ExecuteASM" => Some(Self::ExecuteAsm),
            "ResetSystem" => Some(Self::ResetSystem),
            "PauseUnpauseEmulation" => Some(Self::PauseUnpauseEmulation),
            "PauseToggleEmulation" => Some(Self::PauseToggleEmulation),
            "ResetToMenu" => Some(Self::ResetToMenu),
            "FetchFields" => Some(Self::FetchFields),
            "ReadDirectory" => Some(Self::ReadDirectory),
            "MakeDirectory" => Some(Self::MakeDirectory),
            "RemoveFile" => Some(Self::RemoveFile),
            "RenameFile" => Some(Self::RenameFile),
            "PutFile" => Some(Self::PutFile),
            "GetFile" => Some(Self::GetFile),
            "BootFile" => Some(Self::BootFile),
            "NWACommand" => Some(Self::NwaCommand),
            _ => None,
        }
    }
}
/// fields to query from DeviceInfo.FetchFields
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Field {
    DeviceName = 0,
    DeviceVersion = 1,
    DeviceStatus = 2,
    CoreName = 20,
    CoreVersion = 21,
    CorePlatform = 22,
    RomFileName = 40,
    RomHashType = 41,
    RomHashValue = 42,
}
impl Field {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Field::DeviceName => "DeviceName",
            Field::DeviceVersion => "DeviceVersion",
            Field::DeviceStatus => "DeviceStatus",
            Field::CoreName => "CoreName",
            Field::CoreVersion => "CoreVersion",
            Field::CorePlatform => "CorePlatform",
            Field::RomFileName => "RomFileName",
            Field::RomHashType => "RomHashType",
            Field::RomHashValue => "RomHashValue",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DeviceName" => Some(Self::DeviceName),
            "DeviceVersion" => Some(Self::DeviceVersion),
            "DeviceStatus" => Some(Self::DeviceStatus),
            "CoreName" => Some(Self::CoreName),
            "CoreVersion" => Some(Self::CoreVersion),
            "CorePlatform" => Some(Self::CorePlatform),
            "RomFileName" => Some(Self::RomFileName),
            "RomHashType" => Some(Self::RomHashType),
            "RomHashValue" => Some(Self::RomHashValue),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DirEntryType {
    Directory = 0,
    File = 1,
}
impl DirEntryType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DirEntryType::Directory => "Directory",
            DirEntryType::File => "File",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Directory" => Some(Self::Directory),
            "File" => Some(Self::File),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod devices_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct DevicesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DevicesClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DevicesClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DevicesClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            DevicesClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// detect and list devices currently connected to the system:
        pub async fn list_devices(
            &mut self,
            request: impl tonic::IntoRequest<super::DevicesRequest>,
        ) -> std::result::Result<tonic::Response<super::DevicesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/Devices/ListDevices");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("Devices", "ListDevices"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod device_control_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct DeviceControlClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DeviceControlClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DeviceControlClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DeviceControlClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            DeviceControlClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// only available if DeviceCapability ResetSystem is present
        pub async fn reset_system(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetSystemRequest>,
        ) -> std::result::Result<tonic::Response<super::ResetSystemResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/DeviceControl/ResetSystem");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("DeviceControl", "ResetSystem"));
            self.inner.unary(req, path, codec).await
        }
        /// only available if DeviceCapability ResetToMenu is present
        pub async fn reset_to_menu(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetToMenuRequest>,
        ) -> std::result::Result<tonic::Response<super::ResetToMenuResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/DeviceControl/ResetToMenu");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("DeviceControl", "ResetToMenu"));
            self.inner.unary(req, path, codec).await
        }
        /// only available if DeviceCapability PauseUnpauseEmulation is present
        pub async fn pause_unpause_emulation(
            &mut self,
            request: impl tonic::IntoRequest<super::PauseEmulationRequest>,
        ) -> std::result::Result<tonic::Response<super::PauseEmulationResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/DeviceControl/PauseUnpauseEmulation");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("DeviceControl", "PauseUnpauseEmulation"));
            self.inner.unary(req, path, codec).await
        }
        /// only available if DeviceCapability PauseToggleEmulation is present
        pub async fn pause_toggle_emulation(
            &mut self,
            request: impl tonic::IntoRequest<super::PauseToggleEmulationRequest>,
        ) -> std::result::Result<tonic::Response<super::PauseToggleEmulationResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/DeviceControl/PauseToggleEmulation");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("DeviceControl", "PauseToggleEmulation"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod device_memory_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct DeviceMemoryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DeviceMemoryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DeviceMemoryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DeviceMemoryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            DeviceMemoryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// detect the current memory mapping for the given device by reading 00:FFB0 header:
        pub async fn mapping_detect(
            &mut self,
            request: impl tonic::IntoRequest<super::DetectMemoryMappingRequest>,
        ) -> std::result::Result<tonic::Response<super::DetectMemoryMappingResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/DeviceMemory/MappingDetect");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("DeviceMemory", "MappingDetect"));
            self.inner.unary(req, path, codec).await
        }
        /// read a single memory segment with a given size from the given device:
        pub async fn single_read(
            &mut self,
            request: impl tonic::IntoRequest<super::SingleReadMemoryRequest>,
        ) -> std::result::Result<tonic::Response<super::SingleReadMemoryResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/DeviceMemory/SingleRead");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("DeviceMemory", "SingleRead"));
            self.inner.unary(req, path, codec).await
        }
        /// write a single memory segment with given data to the given device:
        pub async fn single_write(
            &mut self,
            request: impl tonic::IntoRequest<super::SingleWriteMemoryRequest>,
        ) -> std::result::Result<tonic::Response<super::SingleWriteMemoryResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/DeviceMemory/SingleWrite");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("DeviceMemory", "SingleWrite"));
            self.inner.unary(req, path, codec).await
        }
        /// read multiple memory segments with given sizes from the given device:
        pub async fn multi_read(
            &mut self,
            request: impl tonic::IntoRequest<super::MultiReadMemoryRequest>,
        ) -> std::result::Result<tonic::Response<super::MultiReadMemoryResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/DeviceMemory/MultiRead");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("DeviceMemory", "MultiRead"));
            self.inner.unary(req, path, codec).await
        }
        /// write multiple memory segments with given data to the given device:
        pub async fn multi_write(
            &mut self,
            request: impl tonic::IntoRequest<super::MultiWriteMemoryRequest>,
        ) -> std::result::Result<tonic::Response<super::MultiWriteMemoryResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/DeviceMemory/MultiWrite");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("DeviceMemory", "MultiWrite"));
            self.inner.unary(req, path, codec).await
        }
        /// stream read multiple memory segments with given sizes from the given device:
        pub async fn stream_read(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::MultiReadMemoryRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::MultiReadMemoryResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/DeviceMemory/StreamRead");
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("DeviceMemory", "StreamRead"));
            self.inner.streaming(req, path, codec).await
        }
        /// stream write multiple memory segments with given data to the given device:
        pub async fn stream_write(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::MultiWriteMemoryRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::MultiWriteMemoryResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/DeviceMemory/StreamWrite");
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("DeviceMemory", "StreamWrite"));
            self.inner.streaming(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod device_filesystem_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct DeviceFilesystemClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DeviceFilesystemClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DeviceFilesystemClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DeviceFilesystemClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            DeviceFilesystemClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn read_directory(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadDirectoryRequest>,
        ) -> std::result::Result<tonic::Response<super::ReadDirectoryResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/DeviceFilesystem/ReadDirectory");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("DeviceFilesystem", "ReadDirectory"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn make_directory(
            &mut self,
            request: impl tonic::IntoRequest<super::MakeDirectoryRequest>,
        ) -> std::result::Result<tonic::Response<super::MakeDirectoryResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/DeviceFilesystem/MakeDirectory");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("DeviceFilesystem", "MakeDirectory"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_file(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveFileRequest>,
        ) -> std::result::Result<tonic::Response<super::RemoveFileResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/DeviceFilesystem/RemoveFile");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("DeviceFilesystem", "RemoveFile"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn rename_file(
            &mut self,
            request: impl tonic::IntoRequest<super::RenameFileRequest>,
        ) -> std::result::Result<tonic::Response<super::RenameFileResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/DeviceFilesystem/RenameFile");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("DeviceFilesystem", "RenameFile"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn put_file(
            &mut self,
            request: impl tonic::IntoRequest<super::PutFileRequest>,
        ) -> std::result::Result<tonic::Response<super::PutFileResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/DeviceFilesystem/PutFile");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("DeviceFilesystem", "PutFile"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_file(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFileRequest>,
        ) -> std::result::Result<tonic::Response<super::GetFileResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/DeviceFilesystem/GetFile");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("DeviceFilesystem", "GetFile"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn boot_file(
            &mut self,
            request: impl tonic::IntoRequest<super::BootFileRequest>,
        ) -> std::result::Result<tonic::Response<super::BootFileResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/DeviceFilesystem/BootFile");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("DeviceFilesystem", "BootFile"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod device_info_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct DeviceInfoClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DeviceInfoClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DeviceInfoClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DeviceInfoClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            DeviceInfoClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn fetch_fields(
            &mut self,
            request: impl tonic::IntoRequest<super::FieldsRequest>,
        ) -> std::result::Result<tonic::Response<super::FieldsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/DeviceInfo/FetchFields");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("DeviceInfo", "FetchFields"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod device_nwa_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct DeviceNwaClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DeviceNwaClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DeviceNwaClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DeviceNwaClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            DeviceNwaClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn nwa_command(
            &mut self,
            request: impl tonic::IntoRequest<super::NwaCommandRequest>,
        ) -> std::result::Result<tonic::Response<super::NwaCommandResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/DeviceNWA/NWACommand");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("DeviceNWA", "NWACommand"));
            self.inner.unary(req, path, codec).await
        }
    }
}
