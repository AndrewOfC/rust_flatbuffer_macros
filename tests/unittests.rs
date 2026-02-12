extern crate core;

mod fb;

#[cfg(test)]
mod tests {
    use super::fb::unittests::*;
    use rust_flatbuffer_macros::{build_flatbuffer, flatbuffer_builderbuilder};
    use rand::{Rng, RngExt};

    flatbuffer_builderbuilder!($ /* note the $ */ UnittestMessage, TestMessage);

    #[test]
    pub fn test_fields_builder() {
        let mut rng = rand::rng();
        let a = rng.random_range(-1000..1000);
        let b = rng.random_range(-1000..1000);

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
        let mut rng = rand::rng();
        let addend_a = rng.random_range(-1000..1000);
        let addend_b = rng.random_range(-1000..1000);

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
} // mod
