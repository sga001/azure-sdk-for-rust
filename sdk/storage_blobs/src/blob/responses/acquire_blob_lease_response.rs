use azure_core::headers::*;
use azure_core::prelude::*;
use azure_core::RequestId;
use chrono::{DateTime, Utc};

azure_storage::response_from_headers!(AcquireBlobLeaseResponse,
               etag_from_headers => etag: String,
               last_modified_from_headers => last_modified: DateTime<Utc>,
                       lease_id_from_headers => lease_id: LeaseId,
               request_id_from_headers => request_id: RequestId,
               date_from_headers => date: DateTime<Utc>
);
