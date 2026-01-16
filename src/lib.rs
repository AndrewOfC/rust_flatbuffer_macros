#![no_std]
mod unittests;
#[path="./fb_ops/mod.rs"]
mod fbserver_operations;

/**
 * Example:
 * ```rust
 *  // given this struct:
 *  pub struct AddRequest {
 *      pub a: u32,
 *      pub b: u32,
 *  }
 * ```
 *   Build a flatbufferuse crate::util;
 * use paste::paste;
 *
 *
 * ```rust
 * let args = AddRequestArgs {
 *      a: 1,
 *      b: 2,
 *  } ;
 *
 *  let req = AddRequest::create(&mut builder, &args) ;
 *  ```
 *
 * Instead:
 *
 * ```rust
 *
 *  let req = build_flatbuffer!(&mut builder, AddRequest, a=1, b=2) ;
 *  ```
 *
 * OR:
 *
 * ```rust
 *  let a = 1 ;
 *  let b = 2 ;
 *  let req = build_flatbuffer!(&mut builder, AddRequest, a, b) ;
 * ```
 */
#[macro_export]
macro_rules! build_flatbuffer {
    ($builder:expr, $typ:ident, $($field:ident = $value:expr),* ) => {
        {
        paste::paste! {
        let args = [<$typ Args>] {
            $($field: $value,)*
        } ;
        $typ::create($builder, &args)
        } }
    } ;

    ($builder:expr, $typ:ident, $($field:ident),* ) => {
        {
        paste::paste! {
        let args = [<$typ Args>] {
            $($field,)*
        } ;
        $typ::create($builder, &args)
        } }
    } ;
}

#[macro_export]
macro_rules! build_flatbuffer_macro {
    ($root_type:ident, $body_type:ident) => {
        #[macro_export]
        macro_rules!  {

        }
    }
}

#[macro_export]
macro_rules! build_request_buffer {
    ($builder:expr, $bodytype:ident, $($field:ident = $value:expr),* ) => {
        {
            let body = build_flatbuffer!($builder, $bodytype, $($field = $value),* );
            let args = RequestMessageArgs {
                request_type: Request::$bodytype,
                request: Some(body.as_union_value()),
            } ;
            let msg = RequestMessage::create($builder, &args) ;
        $builder.finish_size_prefixed(msg, None);
            $builder.finished_data()
        }
    } ;

    ($builder:expr, $bodytype:ident, $($field:ident),* ) => {
        {
            let body = build_flatbuffer!($builder, $bodytype, $($field),* );
            let args = RequestMessageArgs {
                request_type: Request::$bodytype,
                request: Some(body.as_union_value()),
            } ;
            let msg = RequestMessage::create($builder, &args) ;
        $builder.finish_size_prefixed(msg, None);
            $builder.finished_data()
        }
    }
}

#[macro_export]
macro_rules! build_response_buffer {
    ($builder:expr, $bodytype:ident, $($field:ident = $value:expr),* ) => {
        {
            let body = build_flatbuffer!($builder, $bodytype, $($field = $value),* );
            let args = ResponseMessageArgs {
                response_type: Response::$bodytype,
                response: Some(body.as_union_value()),
            } ;
            let msg = ResponseMessage::create($builder, &args) ;
            $builder.finish_size_prefixed(msg, None);
            $builder.finished_data()
        }
    } ;

    ($builder:expr, $bodytype:ident, $($field:ident),* ) => {
        {
            let body = build_flatbuffer!($builder, $bodytype, $($field),* );
            let args = ResponseMessageArgs {
                response_type: Response::$bodytype,
                response: Some(body.as_union_value()),
            } ;
            let msg = ResponseMessage::create($builder, &args) ;
            $builder.finish_size_prefixed(msg, None);
            $builder.finished_data()
        }
    }
}


#[macro_export]
macro_rules! union_values {
    ($union_type:ident) => {
        fn write_union_values() {
            let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
            let dest_path = Path::new(&out_dir).join("Request_order.dat");
            let mut file = File::create(&dest_path).expect("Failed to create Request_order.dat");

            let mut i = 0;
            loop {
                let o  = $union_type(i) ;
                match o.variant_name() {
                    Some(name) => {
                        writeln!(file, "{}:{}", i, name).expect("Failed to write to file");
                    }
                    None => break
                }
                i += 1;
            }
        }
    };
}
