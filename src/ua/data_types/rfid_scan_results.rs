use crate::{ua, DataType as _};
use crate::Error;
use open62541_sys::{UA_RfidScanResult, UA_ScanData , UA_String, UA_TYPES_AUTOID, UA_TYPES_AUTOID_RFIDSCANRESULT, UA_DataType};
use crate::data_type;

data_type!(RfidScanResult, UA_RfidScanResult, UA_TYPES_AUTOID, UA_TYPES_AUTOID_RFIDSCANRESULT);

///
/// This is an example implementation for one field (codeType). 
/// This only works when the autoid nodeset is included.
impl RfidScanResult {

    pub fn new(codeType: &str) -> Self {
        Self::init().with_code_type(codeType)
    }

    pub fn with_code_type(mut self, code_type: &str) -> Self {
        ua::String::new(code_type)
            .unwrap()
            .move_into_raw(&mut self.0.codeType);
        self
    }

    pub fn code_type(&self) -> String {
        unsafe {
            std::str::from_utf8(std::slice::from_raw_parts(
                self.0.codeType.data,
                self.0.codeType.length as usize,
            ))
            .expect("Invalid UTF-8 in codeType")
            .to_string()
        }
    }

}
