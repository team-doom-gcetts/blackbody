
use tokio::io::AsyncWrite;
use uuid::Uuid;
use crate::prelude::*;

use chrono::{
  Utc,
  DateTime,
};




pub struct ReportManager;


#[allow(unused)]
pub struct SessionId(Uuid);
#[allow(unused)]
pub struct OperationId(Uuid);

pub struct SanitizationCertificate;

pub struct ForensicReport;




impl ReportManager {
  pub fn generate_sanitization_cert(&self,_operation_id: OperationId)-> Result<SanitizationCertificate> {
    todo!()
  }

  pub fn generate_forensic_report(&self,_session_id: SessionId)-> Result<ForensicReport> {
    todo!()
  }

  pub fn export_audit_log<W: AsyncWrite+Unpin>(&self,_start: DateTime<Utc>,_end: DateTime<Utc>,mut _writer: W)-> Result<()> {
    todo!()
  }
}


























