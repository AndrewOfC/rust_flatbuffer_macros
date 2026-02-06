#![no_std]

extern crate std;

/**
 * Example:./"
 * ```rust
 *  // given this struct:
 *  pub struct AddRequest {
 *      pub a: u32,
 *      pub b: u32,
 *  }
 * ```
 *   Build a flatbuffer
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

   ($builder:expr, $typ:ident) => {
        {
            paste::paste! {
                let args = [<$typ Args>]::default() ;
                $typ::create($builder, &args)
            }
        }
    } ;

($builder:expr, $typ:ident, $($field:ident = $value:expr),* ) => {
        {
        paste::paste! {
        let args = [<$typ Args>] {
            $($field: $value,)*
            ..[<$typ Args>]::default()
        } ;
        $typ::create($builder, &args)
        } }
    } ;

    ($builder:expr, $typ:ident, $($field:ident),* ) => {
        {
        paste::paste! {
        let args = [<$typ Args>] {
            $($field,)*
            ..[<$typ Args>]::default()
        } ;
        $typ::create($builder, &args)
        } }
    } ;
}


#[macro_export]
macro_rules! flatbuffer_builderbuilder {
    ($DOLLAR:tt $root:ident, $union:ident) => {
        paste::paste! {
            macro_rules! [<build_ $root _buffer>]
                {
                    ($DOLLAR builder:expr, $bodytype:ident) => {{
                        let body = build_flatbuffer!($builder, $bodytype) ;
                        let args = [ <$root Args> ] {
                            [ <$union:snake _type> ]: $union::$bodytype,
                            [<$union:snake>]: Some(body.as_union_value())
                        } ;
                        let msg = $root::create($builder, &args) ;
                        $builder.finish_size_prefixed(msg, None);
                        $builder.finished_data()
                        }} ;

                    ($DOLLAR builder:expr, $bodytype:ident, $DOLLAR($DOLLAR field:ident = $DOLLAR value:expr),* ) => {{
                        let body = build_flatbuffer!($builder, $bodytype, $DOLLAR($DOLLAR field = $DOLLAR value),* );
                        let args = [ <$root Args> ] {
                            [ <$union:snake _type> ]: $union::$bodytype,
                            [<$union:snake>]: Some(body.as_union_value())
                        } ;
                        let msg = $root::create($builder, &args) ;
                        $builder.finish_size_prefixed(msg, None);
                        $builder.finished_data()
                    }} ;

                    ($DOLLAR builder:expr, $bodytype:ident, $DOLLAR($DOLLAR field:ident),* ) => {{
                        let body = build_flatbuffer!($builder, $bodytype, $DOLLAR($DOLLAR field),* );
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
}

#[macro_export]
macro_rules! union_values {
    ($union_type:ident) => {
        paste::paste! {
            pub fn [<write_ $union_type _values>]() {
                use std::env;
                use std::fs::File;
                use std::io::Write;
                use std::path::Path;

                let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
                let filename = format!("{}_order.dat", stringify!($union_type));
                let dest_path = Path::new(&out_dir).join(filename);
                let mut file = File::create(&dest_path).expect(&format!("Failed to create {}_order.dat", stringify!($union_type)));

                let mut i = 0;
                loop {
                    let o = $union_type(i);
                    match o.variant_name() {
                        Some(name) => {
                            writeln!(file, "{}:{}", i, name).expect("Failed to write to file");
                        }
                        None => break
                    }
                    i += 1;
                }
            }
        }
    };
}
