# Overview

A useful Rust macro that defines other Rust macros.

# Highlights

* Condenses appoximately 12+ lines of code into 1.
* Provides an example of using Rust's [variadic](https://doc.rust-lang.org/rust-by-example/macros/variadics.html) macro syntax.
* An example of meta-coding (i.e., code that generates code)

# Installation

In cargo.toml
```toml
[dependencies]
rust_flatbuffer_macros = "1.1.0"
```

# FlatBuffers

[FlatBuffers](https://flatbuffers.dev) is a cross platform serialization library that provides performance and 
memory efficiency.  This is particularly useful in [no_std](https://doc.rust-lang.org/rust-by-example/macros/variadics.html) environments.

# Example:
Given the common use-case of using a FlatBuffer union to 'multiplex' a message 
with the schema:

```flatbuffers
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

root_type Message ;
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
        let msg = Message::create((&mut builder), &args);
        &mut builder.finish_size_prefixed(msg, None);
        let buf = &mut builder.finished_data() ;
}

```

Instead:

```rust
use flatbuffer_macros::{flatbuffer_builderbuilder, build_flatbuffer};
    
// Constructs a macro to build the flatbuffer
flatbuffer_builderbuilder!($ Message, TestMessage) ; // note the $

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

# Internals (nuts&amp;bolts)

The power of Rust macros comes from the ability to interact with the compiler
as code is being compiled.  The compiler will pass a macro syntatical elements that 
can be modified and expanded into

## Inner Macro

### Flatbuffer naming

For a given flatbuffer struct 'AddRequest', the [flatc compiler](https://flatbuffers.dev/flatc/) will generate a struct named 'AddRequestArgs'.
To construct a flatbuffer representation of AddRequest, you allocate and populate an AddRequestArgs struct.

### Expansion

Here is one piece(arm) the 'engine' that this macro is built upon:

```rust
macro_rules! build_flatbuffer {
    ($builder:expr, $typ:ty, $($field:ident $(= $value:expr)?),+ ) => {
        {
        paste::paste! {
        let args = [<$typ Args>] {
            $($field $(: $value)?,)*
            ..[<$typ Args>]::default()
        } ;
        $typ::create($builder, &args)
        } }
    } ;
}
```

* $builder - is the [FlatBufferBuilder](https://docs.rs/flatbuffers/latest/flatbuffers/struct.FlatBufferBuilder.html) and is a macro! type 'expr'(i.e., a piece of rust syntax that is a valid expression)
* $typ - is the flatbuffer struct name
* $field - of type 'ident' (i.e., a valid rust identifier), possibly a local variable
* $value - is the expression to be assigned to the field if there is no local.

The [paste!](https://docs.rs/paste/latest/paste/#more-elaborate-example) macro will expand the
[<$typ Args>] piece and turn AddRequest into AddRequestArgs.

The $($field:ident $(= $value:expr)?)* accepts one or more local variable or  field&amp;value pairs, 
separated by an '='.  Then the macro will expand to the "variable" or "field: value" pairs within the AddRequestArgs 
definition.  

When we apply this macro to:
```rust
let fb = build_flatbuffer!(&mut builder, AddRequest, addend_a = a, addend_b = b);
```
 it will expand to the following:

```rust
 {
    let args = AddRequestArgs {
        addend_a: a,
        addend_b: b,
        ..AddRequestArgs::default()
    };
    AddRequest::create((&mut builder), &args)
}
```
Which is a rust expression that evaluates to a flatbuffer AddRequest struct which then can
be populated into another flatbuffer struct or union.  

There are two other arms to the macro, one that handles an empty message, and another that
leverages rusts ability to incorporate local variables that match field names to populate
a the Args struct.

```rust
    let addend_a = 2;
    let addend_b = 3;
    let b = build_flatbuffer!(&mut builder, AddRequest, addend_a, addend_b);
```
Which expands to:
```rust
 
        let args = AddRequestArgs {
            addend_a,
            addend_b,
            ..AddRequestArgs::default()
        };
        AddRequest::create((&mut builder), &args)
```

## Outer Macro

The tricky part is the outer macro that defines the inner macro.  Here we're up
against some syntactical rules in Rust where we must provide a token for the '$'
that will be used to define the inner macro.  With the outer macro signature:

```rust
macro_rules! flatbuffer_builderbuilder {
    ($DOLLAR:tt $root:ident, $union:ident) {
        ...
    }
} 
```

we invoke it as:

```rust
flatbuffer_builderbuilder!($ Message, Payload);
```

The $ is taken as the $DOLLAR token and the remaining parameters are taken in as 
$root(root_type defined in the flatbuffer schema) and $union(the internal payload).
This is done to assist the rust compiler is parsing the macro paramters that expand 
variadic arguments:

```rust
macro_rules! flatbuffer_builderbuilder {
    ($DOLLAR:tt $root:ident, $union:ident) => {
        paste::paste! {
            ($builder:expr, $bodytype:ident, $DOLLAR($field:ident),* ) => {{
                        let body = build_flatbuffer!($builder, $bodytype, $DOLLAR($field),* );
                        let args = [ <$root Args> ] {
                            [ <$union:snake _type> ]: $union::$bodytype,
                            [<$union:snake>]: Some(body.as_union_value())
                        } ;
                        let msg = $root::create($builder, &args) ;
                        $builder.finish_size_prefixed(msg, None);
                        $builder.finished_data()
                    }} ;
        }
    }
}
```

# Addtional Root Fields
Extending the original example:
```flatbuffers

union Payload {
    AddRequest,
    MultiplyRequest,
}
table Message {
    payload: Payload ;
    extra_field1: int32 ;
    extra_field2: int32 ;
}

root_type Message ;
```

The extra fields can be added with a variant of the macro:
```rust

let b = build_flatbuffer!(&mut builder, extra_field1=0, extra_field2=2 => AddRequest, addend_a, addend_b);
```

The '=>' separates the root fields from the populating of the inner fields of the union member.

# Troubleshooting

If we don't use the $DOLLAR token we get the following error:
```text
   Compiling rust_flatbuffer_macros v0.1.0 (/Users/andrew/projects/rustserver2/rust_flatbuffer_macros)
error: attempted to repeat an expression containing no syntax variables matched as repeating at this depth
   --> /Users/andrew/projects/rustserver2/rust_flatbuffer_macros/src/lib.rs:123:55
    |
123 |                     ($builder:expr, $bodytype:ident, $($field:ident),* ) => {{
```


# flatc setup

The flatc version with ubuntu and other distros sometimes lags
behind the one in use with rust's flatbuffers-build crate.  If
so carefully check the version of the flatbuffers-build crate, it will
typically reference the version of flatc in use (e.g v25.12.19)

## Building
To build the appropriate version:

```bash
mkdir tmp
cd tmp
git clone https://github.com/google/flatbuffers.git
cd flatbuffers
git checkout v25.12.19
mkdir release
cd release
cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/opt/flatbuffers ..
make -j `nproc`
sudo make install
export PATH=/opt/flatbuffers/bin:$PATH # possibly adding it to your profile
```

## In build.rs

```rust 
    // Compile the schemas.
    // NOTE: For multiple schemas, the order can matter if there are dependencies.
    // For simple cases, iterating over the directory contents is often sufficient.
    BuilderOptions::new_with_files(&fbs_files)
        .set_compiler("/opt/flatbuffers/bin/flatc")
        .set_output_path("./tests/fb")
        .add_flatc_arguments(&["--reflect-types", "--rust-module-root-file"])
        .compile()
        .expect("FlatBuffers compilation failed");

```
