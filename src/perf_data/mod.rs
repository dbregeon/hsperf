/// This module contains the  Rust representation of the hot spot perf data struct
/// as described in https://github.com/openjdk/jdk21/blob/master/src/hotspot/share/runtime
mod data_type;
mod perf_data_entry_header;
mod perf_data_prolog;
mod unit;
mod variability;

pub(crate) type PerfDataProlog = perf_data_prolog::PerfDataProlog;
pub(crate) type PerfDataEntryHeader = perf_data_entry_header::PerfDataEntryHeader;
pub(crate) type Unit = unit::Unit;
pub(crate) type DataType = data_type::DataType;
pub(crate) type Endianness = perf_data_prolog::Endianness;
pub(crate) type Variability = variability::Variability;
