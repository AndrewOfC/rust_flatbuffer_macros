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
extern crate core;
mod fb;

#[cfg(test)]
mod tests {
    use super::fb::unittests::*;
    use rand::{Rng, RngExt};
    use rust_flatbuffer_macros::{build_flatbuffer, flatbuffer_builderbuilder};

    flatbuffer_builderbuilder!($ /* note the $ */ UnittestMessage, TestMessage);
    
    pub fn quickrand() -> i32 {
        let mut rng = rand::rng();
        rng.random_range(-1000..1000)
    }

    #[test]
    pub fn test_fields_builder() {
        let a = quickrand();
        let b = quickrand();

        let mut builder = flatbuffers::FlatBufferBuilder::new();
        let buf =
            build_UnittestMessage_buffer!(&mut builder, AddRequest, addend_a = a, addend_b = b);

        let root_message = flatbuffers::root::<UnittestMessage>(&buf[4..]).unwrap();

        assert!(root_message.test_message_type() == TestMessage::AddRequest);

        let payload = root_message.test_message_as_add_request().unwrap();
        assert!(payload.addend_a() == a);
        assert!(payload.addend_b() == b);
    } // fn

    #[test]
    pub fn test_localvars_builder() {
        let addend_a = quickrand();
        let addend_b = quickrand();

        let mut builder = flatbuffers::FlatBufferBuilder::new();

        let buf = build_UnittestMessage_buffer!(&mut builder, AddRequest, addend_a, addend_b);

        let root_message = flatbuffers::root::<UnittestMessage>(&buf[4..]).unwrap();

        assert!(root_message.test_message_type() == TestMessage::AddRequest);

        let payload = root_message.test_message_as_add_request().unwrap();
        assert!(payload.addend_a() == addend_a);
        assert!(payload.addend_b() == addend_b);
    }

    #[test]
    pub fn test_empty_builder() {
        let mut builder = flatbuffers::FlatBufferBuilder::new();
        let buf = build_UnittestMessage_buffer!(&mut builder, UnhandledRequest);

        let rootmessage = flatbuffers::root::<UnittestMessage>(&buf[4..]).unwrap();

        assert!(rootmessage.test_message_type() == TestMessage::UnhandledRequest);
    }

    #[test]
    pub fn test_string_builder() {
        let mut builder = flatbuffers::FlatBufferBuilder::new();
        let s1 = Some(builder.create_string("foo"));
        let buf = build_UnittestMessage_buffer!(
            &mut builder,
            StringMessage,
            s1 = s1,
            s2 = Some(builder.create_string("bar"))
        );

        let rootmessage = flatbuffers::root::<UnittestMessage>(&buf[4..]).unwrap();

        assert!(rootmessage.test_message_type() == TestMessage::StringMessage);

        let payload = rootmessage.test_message_as_string_message().unwrap();
        assert_eq!(payload.s1(), Some("foo"));
        assert_eq!(payload.s2(), Some("bar"));
    }

    #[test]
    pub fn test_4fields_builder() {
        let mut builder = flatbuffers::FlatBufferBuilder::new();
        let (addend_a, b, addend_c, d) = (quickrand(),quickrand(),quickrand(),quickrand());
        let buf = build_UnittestMessage_buffer!(
            &mut builder,
            AddRequest4, addend_a, addend_b = b, addend_c, addend_d = d
        );
        let rootmessage = flatbuffers::root::<UnittestMessage>(&buf[4..]).unwrap();

        assert!(rootmessage.test_message_type() == TestMessage::AddRequest4);
        let payload = rootmessage.test_message_as_add_request_4().unwrap();

        assert_eq!(payload.addend_a(), addend_a);
        assert_eq!(payload.addend_b(), b);
        assert_eq!(payload.addend_c(), addend_c);
        assert_eq!(payload.addend_d(), d);

        builder.reset();

        /*
         * test a different interspersing of fields
         */
        let (addend_a, b, c, addend_d) = (quickrand(),quickrand(),quickrand(),quickrand());
        let buf = build_UnittestMessage_buffer!(
            &mut builder,
            AddRequest4,
            addend_a, addend_b = b, addend_c=c, addend_d) ;

        let rootmessage = flatbuffers::root::<UnittestMessage>(&buf[4..]).unwrap();

        assert!(rootmessage.test_message_type() == TestMessage::AddRequest4);
        let payload = rootmessage.test_message_as_add_request_4().unwrap();

        assert_eq!(payload.addend_a(), addend_a);
        assert_eq!(payload.addend_b(), b);
        assert_eq!(payload.addend_c(), c);
        assert_eq!(payload.addend_d(), addend_d);
    }

    #[test]
    fn test_addtional_root_fields() {
        let addend_a = quickrand();
        let addend_b = quickrand();
        let (serialno, extradata) = (quickrand(), quickrand());

        let mut builder = flatbuffers::FlatBufferBuilder::new();

        let buf = build_UnittestMessage_buffer!(&mut builder, serialno=serialno, extradata => AddRequest, addend_a, addend_b);

        let root_message = flatbuffers::root::<UnittestMessage>(&buf[4..]).unwrap();
        assert!(root_message.serialno() == serialno);
        assert!(root_message.extradata() == extradata);

        assert!(root_message.test_message_type() == TestMessage::AddRequest);

        let payload = root_message.test_message_as_add_request().unwrap();
        assert!(payload.addend_a() == addend_a);
        assert!(payload.addend_b() == addend_b);
    }

} // mod
