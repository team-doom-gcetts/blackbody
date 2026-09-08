
use crate::prelude::*;
use tokio::io::AsyncWrite;



pub struct RecoveredFile;


impl RecoveredFile {
  pub async fn extract<W: AsyncWrite+Unpin>(&self,mut _writer: W)-> Result<()> {
    todo!()
  }
}








