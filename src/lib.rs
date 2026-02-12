#![no_std]

extern crate std;

/**
 *
 * ```rust.ignore
 *  // given this struct:
 *  pub struct AddRequest {
 *      pub a: u32,
 *      pub b: u32,
 *  }
 * ```
 *   Build a flatbuffer
 *
 * ```rust.ignore
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
 * ```rust.ignore
 *
 *  let req = build_flatbuffer!(&mut builder, AddRequest, a=1, b=2) ;
 *  ```
 *
 * OR:
 *
 * ```rust.ignore
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

