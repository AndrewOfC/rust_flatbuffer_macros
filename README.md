
# Overview 

Macros to simplify building flatbuffers 

## Usage
Given the schema:

```flatbuffers
table AddRequest {
    addend_a: int32 ;
    addend_b: int32 ;
}

table MultiplyRequest {
    multiplicand: int32 ;
    mutiplier: int32 ;
}

union Payload {
    AddRequest,
    MultiplyRequest,
}
table Message {
    payload: Payload ; // Note fieldname must be same as field name in snake case
}

root_type UnittestMessage ;



```
A typical construction would look like this:

```rust

fn build() {

    let body = {
            let args = AddRequestArgs {
                addend_a: 1,
                addend_b: 2,
                ..AddRequestArgs::default()
            };
            AddRequest::create((&mut builder), &args)
        };
        let args = MessageArgs {
            test_message_type: TestMessage::AddRequest,
            test_message: Some(body.as_union_value()),
        };
        let msg = UnittestMessage::create((&mut builder), &args);
        &mut builder.finish_size_prefixed(msg, None);
        let buf = &mut builder.finished_data() ;
}

```

Instead:

```rust
use flatbuffer_macros::{flatbuffer_builderbuilder, build_flatbuffer};
    
// Constructs a macro to build the flatbuffer
flatbuffer_builderbuilder!($ UnittestMessage, TestMessage) ; // note the $

fn build() {
    let buf = build_Message_buffer!(&mut builder, AddRequest, addend_a=1, addend_b=2) ;
}

// OR

fn build2() {
    let addend_a = 1;
    let addend_b = 2;
    let buf = build_AddRequest_buffer!(&mut builder, addend_a, addend_b) ;
}

```