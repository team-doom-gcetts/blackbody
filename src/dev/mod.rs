
mod hdd;
mod ssd;
mod usb;
mod file;
mod sd_card;
mod logical_volume;


mod wipe_metadata;
mod recovered_file;


use crate::prelude::*;

pub use file::File;
pub use sd_card::SdCard;
pub use hdd::HardDiskDrive;
pub use ssd::SolidStateDrive;
pub use usb::UniversalSerialBus;
pub use logical_volume::LogicalVolume;

pub use wipe_metadata::WipeMetadata;
pub use recovered_file::RecoveredFile;



#[allow(async_fn_in_trait)]
pub trait Device {
  async fn wipe(&mut self,options: WipeOptions)-> Result<WipeMetadata>;
  async fn scan(&self,quick_scan: bool)-> Result<SessionId>;
  async fn crave_by_sig<S: AsRef<[u8]>>(&self,session_id: SessionId,sigs: impl IntoIterator<Item=S>)-> Result<Vec<RecoveredFile>>;
  async fn reconstruct_fragmented_files(&self,session_id: SessionId)-> Result<Vec<RecoveredFile>>;
}

pub struct WipeOptions {
  pub method: WipeMethod,
  pub verify: VerificationMode,
  pub chunk_size: usize,
  pub parallelism: usize,
}

pub enum VerificationMode {
  None,
  Sampled {
    percentage: u8,
  },
  Full,
}

pub enum WipeMethod {
  Overwrite(OverwriteMethod),
  AtaSecureErase,
  AtaSanitize,
  NvmeSanitize,
  NvmeFormat,
}

pub enum OverwriteMethod {
  Zero,
  One,
  Random,
  Pattern(u8),
}


pub enum WipeStandard {
  Nist80088Clear,
  Nist80088Purge,
  Dod522022M,
  Gutmann,
  SecureEraseCmd, // SSD-specific hardware command
}














