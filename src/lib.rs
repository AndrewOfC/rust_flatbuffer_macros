// MIT License
//
// Copyright (c) 2026 Andrew Ellis Page
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
// SPDX short identifier: MIT

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
                    ($builder:expr, $bodytype:ident) => {{
                        let body = build_flatbuffer!($builder, $bodytype) ;
                        let args = [ <$root Args> ] {
                            [ <$union:snake _type> ]: $union::$bodytype,
                            [<$union:snake>]: Some(body.as_union_value())
                        } ;
                        let msg = $root::create($builder, &args) ;
                        $builder.finish_size_prefixed(msg, None);
                        $builder.finished_data()
                        }} ;

                    ($builder:expr, $bodytype:ident, $DOLLAR($field:ident = $value:expr),* ) => {{
                        let body = build_flatbuffer!($builder, $bodytype, $DOLLAR($field = $value),* );
                        let args = [ <$root Args> ] {
                            [ <$union:snake _type> ]: $union::$bodytype,
                            [<$union:snake>]: Some(body.as_union_value())
                        } ;
                        let msg = $root::create($builder, &args) ;
                        $builder.finish_size_prefixed(msg, None);
                        $builder.finished_data()
                    }} ;

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
}
