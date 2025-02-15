#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-preview-2021-12")]
pub mod package_preview_2021_12;
#[cfg(all(feature = "package-preview-2021-12", not(feature = "no-default-tag")))]
pub use package_preview_2021_12::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2021-08")]
pub mod package_2021_08;
#[cfg(all(feature = "package-2021-08", not(feature = "no-default-tag")))]
pub use package_2021_08::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-preview-2021-04")]
pub mod package_preview_2021_04;
#[cfg(all(feature = "package-preview-2021-04", not(feature = "no-default-tag")))]
pub use package_preview_2021_04::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-preview-2021-01")]
pub mod package_preview_2021_01;
#[cfg(all(feature = "package-preview-2021-01", not(feature = "no-default-tag")))]
pub use package_preview_2021_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2020-12")]
pub mod package_2020_12;
#[cfg(all(feature = "package-2020-12", not(feature = "no-default-tag")))]
pub use package_2020_12::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-preview-2020-06")]
pub mod package_preview_2020_06;
#[cfg(all(feature = "package-preview-2020-06", not(feature = "no-default-tag")))]
pub use package_preview_2020_06::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2019-12")]
pub mod package_2019_12;
#[cfg(all(feature = "package-2019-12", not(feature = "no-default-tag")))]
pub use package_2019_12::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-preview-2019-12")]
pub mod package_preview_2019_12;
#[cfg(all(feature = "package-preview-2019-12", not(feature = "no-default-tag")))]
pub use package_preview_2019_12::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2019-01")]
pub mod package_2019_01;
#[cfg(all(feature = "package-2019-01", not(feature = "no-default-tag")))]
pub use package_2019_01::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2018-06-preview")]
pub mod package_2018_06_preview;
#[cfg(all(feature = "package-2018-06-preview", not(feature = "no-default-tag")))]
pub use package_2018_06_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
