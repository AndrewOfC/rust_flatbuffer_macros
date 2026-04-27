
# v1.1.0

## Interleaved field parameters

Field parameters can now be interleaved between field=expression and local_var.  Example:

```flatbuffer
table AddRequest {
    addend_a: int32 ;
    addend_b: int32 ;
}

table MultiplyRequest {
    multiplicand: int32 ;
    multiplier: int32 ;
}

union Payload {
    AddRequest,
    MultiplyRequest,
}
table Message {
    payload: Payload ; // Note fieldname must be same as field name in snake case
}
```

```rust
build_flatufferbuilder!($ /* note the $/* Message, Payload) ;

fn build_message() -> Result<(), String> {
    let mut builder = FlatBufferBuilder::new();
    let addend_a = 0
    let b = 1 ;
    
    let buf = build_Message_buffer!(&mut builder, AddRequest, addend_a, addend_b=b+1);
    
    Ok(())
}
```

## Other root fields can be populated

```flatbuffers
union Payload {
    AddRequest,
    MultiplyRequest,
}
table Message {
    serialno: int32 ;
    extra_data: int32 ;
    payload: Payload ; // Note fieldname must be same as field name in snake case
}
```

```rust
fn build_message() -> Result<(), String> {
    let mut builder = FlatBufferBuilder::new();
    let addend_a = 0 ;
    let b = 1 ;
    let (serialno, extra) = (0, 1) ;
    
    let buf = build_Message_buffer!(&mut builder, serialno, extra_data=extra => AddRequest, addend_a, addend_b=b+1);
    Ok(())
}
```