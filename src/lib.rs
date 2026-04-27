#![no_std]
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
#![no_std]
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

   ($builder:expr, $typ:ty) => {
        {
            paste::paste! {
                let args = [<$typ Args>]::default() ;
                $typ::create($builder, &args)
            }
        }
    } ;

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

/**
 * Macro to create a macro that will build a flatbuffer with the common pattern:
 * ```flatbuffer
 *
 *  table SubTable1 {
 *      a: u32,
 *      b: u32,
 *  }
 *
 *  union Payload {
 *      SubTable1,
 *      SubTable2,
 *  }
 *
 *   table Message {
 *      payload: Payload ; // Note fieldname must be same as Union name in snake case
 *      extradata: int32 ;
 *  }
 *
 *  root_table: Message ;
 * ```
 * Invoking flatbuffer_builderbuilder
 * ```ignore
 * use rust_flatbuffer_macros::flatbuffer_builderbuilder ;
 * flatbuffer_builderbuilder!($ Message, Payload) ;
 *
 *
 * // will create a macro called build_Message_buffer that will build a prefixed buffer of type Message
 * // with a payload containing a member of the Payload union.
 *
 *
 * let payload = build_Message_buffer!(builder, SubTable1, a=1, b=2) ;
 *    // OR
 * let a = 1 ;
 * let b = 2 ;
 * let payload = build_Message_buffer!(builder, SubTable1, a, b) ;
 *
 * // other fields in the root table may be added as well
 * let extradata = 123 ;
 * let buf = build_Message_buffer!(builder, extradata=0 => SubTable1, a=0, b=1) ;
 * ```
 *
 * @param $builder: The flatbuffer builder instance.
 * @param $typ: The type for which the builder is being created.
 * @param $($field:ident $(= $value:expr)?),+: Custom fields and their values.
 */
#[macro_export]
macro_rules! flatbuffer_builderbuilder {
    ($DOLLAR:tt $root:ty, $union:ty) => {
        paste::paste! {
            macro_rules! [<build_ $root _buffer>]
                {
                    ($builder:expr, $bodytype:ident) => {{
                        let body = build_flatbuffer!($builder, $bodytype) ;
                        let args = [ <$root Args> ] {
                            [ <$union:snake _type> ]: $union::$bodytype,
                            [<$union:snake>]: Some(body.as_union_value()),
                            ..[<$root Args>]::default()
                        } ;
                        let msg = $root::create($builder, &args) ;
                        $builder.finish_size_prefixed(msg, None);
                        $builder.finished_data()
                        }} ;

                    ($builder:expr, $bodytype:ident, $DOLLAR($field:ident $DOLLAR(= $value:expr)?),+ ) => {{
                        let body = build_flatbuffer!($builder, $bodytype, $DOLLAR($field $DOLLAR(= $value)?),+ );
                        let args = [ <$root Args> ] {
                            [ <$union:snake _type> ]: $union::$bodytype,
                            [<$union:snake>]: Some(body.as_union_value()),
                            ..[<$root Args>]::default()
                        } ;
                        let msg = $root::create($builder, &args) ;
                        $builder.finish_size_prefixed(msg, None);
                        $builder.finished_data()
                    }} ;

                    ($builder:expr, $DOLLAR($rootfield:ident $DOLLAR(= $rootvalue:expr)?),* => $bodytype:ident, $DOLLAR($field:ident $DOLLAR(= $value:expr)?),+ ) => {{
                        let body = build_flatbuffer!($builder, $bodytype, $DOLLAR($field $DOLLAR(= $value)?),+ );
                        let args = [ <$root Args> ] {
                            [ <$union:snake _type> ]: $union::$bodytype,
                            [<$union:snake>]: Some(body.as_union_value()),
                            $DOLLAR($rootfield $DOLLAR(: $rootvalue)?,)*
                            ..[<$root Args>]::default()
                        } ;
                        let msg = $root::create($builder, &args) ;
                        $builder.finish_size_prefixed(msg, None);
                        $builder.finished_data()
                    }} ;

                }
            }
    }
}
