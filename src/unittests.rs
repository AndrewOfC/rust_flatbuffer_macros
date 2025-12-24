#![no_std]

#[cfg(test)]
mod tests {
    use crate::fbserver_operations::fbserver_operations::{
        MultiplyRequest, MultiplyRequestArgs
        ,
        Request, RequestMessage, RequestMessageArgs};
    use crate::{build_flatbuffer, build_request_buffer};

    pub fn test_existing() {
        let mut builder = flatbuffers::FlatBufferBuilder::new();
        let result : u64 = 1 as u64;
        let multiplier = 2 as u32;
        let multiplicand = 3 as u32;
        let buf = build_request_buffer!(&mut builder, MultiplyRequest, multiplicand=2, multiplier=3) ;
    
        let buf2 = build_request_buffer!(&mut builder, MultiplyRequest, multiplicand, multiplier) ;

    }
}