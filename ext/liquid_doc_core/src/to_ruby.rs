use magnus::{Error, Object, RArray, RHash, Ruby, Symbol, TryConvert, Value};
use std::collections::HashMap;

/// A trait similar to serde::Serialize that allows types to be converted to Ruby values
/// using magnus. This provides a clean interface for serializing Rust types to Ruby.
pub trait ToRuby {
    /// Convert this value to a Ruby value
    fn to_ruby(&self, ruby: &Ruby) -> Result<Value, Error>;
}

/// Helper trait for deriving ToRuby implementations
pub trait ToRubyFields {
    /// Convert this value's fields to a Ruby hash
    fn to_ruby_fields(&self, ruby: &Ruby) -> Result<RHash, Error>;
}

// Implement ToRuby for basic Rust types
impl ToRuby for String {
    fn to_ruby(&self, ruby: &Ruby) -> Result<Value, Error> {
        Ok(ruby.str_new(&self).as_value())
    }
}

impl ToRuby for &str {
    fn to_ruby(&self, ruby: &Ruby) -> Result<Value, Error> {
        Ok(ruby.str_new(self).as_value())
    }
}

impl ToRuby for i32 {
    fn to_ruby(&self, ruby: &Ruby) -> Result<Value, Error> {
        Ok(ruby.integer_from_i64(*self as i64).as_value())
    }
}

impl ToRuby for i64 {
    fn to_ruby(&self, ruby: &Ruby) -> Result<Value, Error> {
        Ok(ruby.integer_from_i64(*self).as_value())
    }
}

impl ToRuby for u32 {
    fn to_ruby(&self, ruby: &Ruby) -> Result<Value, Error> {
        Ok(ruby.integer_from_u64(*self as u64).as_value())
    }
}

impl ToRuby for u64 {
    fn to_ruby(&self, ruby: &Ruby) -> Result<Value, Error> {
        Ok(ruby.integer_from_u64(*self).as_value())
    }
}

impl ToRuby for usize {
    fn to_ruby(&self, ruby: &Ruby) -> Result<Value, Error> {
        Ok(ruby.integer_from_u64(*self as u64).as_value())
    }
}

impl ToRuby for f64 {
    fn to_ruby(&self, ruby: &Ruby) -> Result<Value, Error> {
        Ok(ruby.float_new(*self).as_value())
    }
}

impl ToRuby for f32 {
    fn to_ruby(&self, ruby: &Ruby) -> Result<Value, Error> {
        Ok(ruby.float_new(*self as f64).as_value())
    }
}

impl ToRuby for bool {
    fn to_ruby(&self, ruby: &Ruby) -> Result<Value, Error> {
        Ok(if *self { ruby.qtrue() } else { ruby.qfalse() }.as_value())
    }
}

impl<T> ToRuby for Option<T>
where
    T: ToRuby,
{
    fn to_ruby(&self, ruby: &Ruby) -> Result<Value, Error> {
        match self {
            Some(value) => value.to_ruby(ruby),
            None => Ok(ruby.qnil().as_value()),
        }
    }
}

impl<T> ToRuby for Vec<T>
where
    T: ToRuby,
{
    fn to_ruby(&self, ruby: &Ruby) -> Result<Value, Error> {
        let array = RArray::new();
        for item in self {
            array.push(item.to_ruby(ruby)?)?;
        }
        Ok(array.as_value())
    }
}

impl<T> ToRuby for &[T]
where
    T: ToRuby,
{
    fn to_ruby(&self, ruby: &Ruby) -> Result<Value, Error> {
        let array = RArray::new();
        for item in *self {
            array.push(item.to_ruby(ruby)?)?;
        }
        Ok(array.as_value())
    }
}

impl<K, V> ToRuby for HashMap<K, V>
where
    K: ToRuby,
    V: ToRuby,
{
    fn to_ruby(&self, ruby: &Ruby) -> Result<Value, Error> {
        let hash = RHash::new();
        for (key, value) in self {
            hash.aset(key.to_ruby(ruby)?, value.to_ruby(ruby)?)?;
        }
        Ok(hash.as_value())
    }
}

impl<T> ToRuby for Box<T>
where
    T: ToRuby,
{
    fn to_ruby(&self, ruby: &Ruby) -> Result<Value, Error> {
        (**self).to_ruby(ruby)
    }
}

/// Helper function to create a Ruby hash from field name-value pairs
pub fn ruby_hash_from_fields(ruby: &Ruby, fields: &[(&str, Value)]) -> Result<RHash, Error> {
    let hash = RHash::new();
    for (name, value) in fields {
        hash.aset(Symbol::new(name), *value)?;
    }
    Ok(hash)
}

/// Helper function to create a Ruby object with a type field and additional fields
pub fn ruby_object_with_type(
    ruby: &Ruby,
    type_name: &str,
    fields: &[(&str, Value)],
) -> Result<RHash, Error> {
    let hash = RHash::new();
    hash.aset(Symbol::new("type"), ruby.str_new(type_name))?;

    for (name, value) in fields {
        hash.aset(Symbol::new(name), *value)?;
    }
    Ok(hash)
}

/// Convenience macro for implementing ToRuby for structs with named fields
#[macro_export]
macro_rules! impl_to_ruby_for_struct {
    ($struct_name:ident, $type_name:literal, {
        $( $field:ident $(=> $rename:literal)? ),* $(,)?
    }) => {
        impl $crate::to_ruby::ToRuby for $struct_name {
            fn to_ruby(&self, ruby: &magnus::Ruby) -> Result<magnus::Value, magnus::Error> {
                let mut fields = Vec::new();

                $(
                    let field_name = impl_to_ruby_for_struct!(@field_name $field $(=> $rename)?);
                    fields.push((field_name, self.$field.to_ruby(ruby)?));
                )*

                Ok($crate::to_ruby::ruby_object_with_type(ruby, $type_name, &fields)?.as_value())
            }
        }
    };

    (@field_name $field:ident) => {
        stringify!($field)
    };

    (@field_name $field:ident => $rename:literal) => {
        $rename
    };
}

/// Convenience macro for implementing ToRuby for enums with variants
#[macro_export]
macro_rules! impl_to_ruby_for_enum {
    ($enum_name:ident, {
        $( $variant:ident ($inner:ty) ),* $(,)?
    }) => {
        impl $crate::to_ruby::ToRuby for $enum_name {
            fn to_ruby(&self, ruby: &magnus::Ruby) -> Result<magnus::Value, magnus::Error> {
                match self {
                    $(
                        $enum_name::$variant(inner) => inner.to_ruby(ruby),
                    )*
                }
            }
        }
    };
}

// Implementations for liquid_doc_parser types
use liquid_doc_parser::{
    LiquidAST, LiquidDocDescriptionNode, LiquidDocExampleNode, LiquidDocParamNode, LiquidNode,
    Position, TextNode,
};

impl ToRuby for Position {
    fn to_ruby(&self, ruby: &Ruby) -> Result<Value, Error> {
        let hash = RHash::new();
        hash.aset(Symbol::new("start"), self.start.to_ruby(ruby)?)?;
        hash.aset(Symbol::new("end"), self.end.to_ruby(ruby)?)?;
        Ok(hash.as_value())
    }
}

impl ToRuby for TextNode {
    fn to_ruby(&self, ruby: &Ruby) -> Result<Value, Error> {
        let fields = [
            ("type", ruby.str_new("TextNode").as_value()),
            ("value", self.value.to_ruby(ruby)?),
            ("position", self.position.to_ruby(ruby)?),
            ("source", self.source.to_ruby(ruby)?),
        ];
        Ok(ruby_hash_from_fields(ruby, &fields)?.as_value())
    }
}

impl ToRuby for LiquidDocDescriptionNode {
    fn to_ruby(&self, ruby: &Ruby) -> Result<Value, Error> {
        let fields = [
            ("type", ruby.str_new("LiquidDocDescriptionNode").as_value()),
            ("name", self.name.to_ruby(ruby)?),
            ("position", self.position.to_ruby(ruby)?),
            ("source", self.source.to_ruby(ruby)?),
            ("content", self.content.to_ruby(ruby)?),
            ("isImplicit", self.is_implicit.to_ruby(ruby)?),
            ("isInline", self.is_inline.to_ruby(ruby)?),
        ];
        Ok(ruby_hash_from_fields(ruby, &fields)?.as_value())
    }
}

impl ToRuby for LiquidDocParamNode {
    fn to_ruby(&self, ruby: &Ruby) -> Result<Value, Error> {
        let fields = [
            ("type", ruby.str_new("LiquidDocParamNode").as_value()),
            ("name", self.name.to_ruby(ruby)?),
            ("position", self.position.to_ruby(ruby)?),
            ("source", self.source.to_ruby(ruby)?),
            ("paramName", self.param_name.to_ruby(ruby)?),
            ("paramDescription", self.param_description.to_ruby(ruby)?),
            ("paramType", self.param_type.to_ruby(ruby)?),
            ("required", self.required.to_ruby(ruby)?),
        ];
        Ok(ruby_hash_from_fields(ruby, &fields)?.as_value())
    }
}

impl ToRuby for LiquidDocExampleNode {
    fn to_ruby(&self, ruby: &Ruby) -> Result<Value, Error> {
        let fields = [
            ("type", ruby.str_new("LiquidDocExampleNode").as_value()),
            ("name", self.name.to_ruby(ruby)?),
            ("position", self.position.to_ruby(ruby)?),
            ("source", self.source.to_ruby(ruby)?),
            ("content", self.content.to_ruby(ruby)?),
            ("isInline", self.is_inline.to_ruby(ruby)?),
        ];
        Ok(ruby_hash_from_fields(ruby, &fields)?.as_value())
    }
}

impl ToRuby for LiquidNode {
    fn to_ruby(&self, ruby: &Ruby) -> Result<Value, Error> {
        match self {
            LiquidNode::LiquidDocDescriptionNode(node) => node.to_ruby(ruby),
            LiquidNode::TextNode(node) => node.to_ruby(ruby),
            LiquidNode::LiquidDocParamNode(node) => node.to_ruby(ruby),
            LiquidNode::LiquidDocExampleNode(node) => node.to_ruby(ruby),
            LiquidNode::LiquidRawTag(node) => {
                // For now, just create a basic hash since we don't have the full definition
                let fields = [
                    ("type", ruby.str_new("LiquidRawTag").as_value()),
                    ("name", node.name.to_ruby(ruby)?),
                    ("body", node.body.to_ruby(ruby)?),
                ];
                Ok(ruby_hash_from_fields(ruby, &fields)?.as_value())
            }
        }
    }
}

impl ToRuby for LiquidAST {
    fn to_ruby(&self, ruby: &Ruby) -> Result<Value, Error> {
        let fields = [
            ("type", ruby.str_new("LiquidAST").as_value()),
            ("nodes", self.nodes.to_ruby(ruby)?),
        ];
        Ok(ruby_hash_from_fields(ruby, &fields)?.as_value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use magnus::Ruby;

    #[test]
    fn test_basic_types() {
        let ruby = Ruby::init().unwrap();

        // Test string
        let s = "hello".to_string();
        let ruby_val = s.to_ruby(&ruby).unwrap();
        assert!(ruby_val.is_kind_of(ruby.class_string()));

        // Test integer
        let i = 42i32;
        let ruby_val = i.to_ruby(&ruby).unwrap();
        assert!(ruby_val.is_kind_of(ruby.class_integer()));

        // Test boolean
        let b = true;
        let ruby_val = b.to_ruby(&ruby).unwrap();
        assert_eq!(ruby_val, ruby.qtrue().as_value());

        // Test option
        let opt: Option<String> = Some("test".to_string());
        let ruby_val = opt.to_ruby(&ruby).unwrap();
        assert!(ruby_val.is_kind_of(ruby.class_string()));

        let opt: Option<String> = None;
        let ruby_val = opt.to_ruby(&ruby).unwrap();
        assert_eq!(ruby_val, ruby.qnil().as_value());

        // Test vec
        let vec = vec![1, 2, 3];
        let ruby_val = vec.to_ruby(&ruby).unwrap();
        assert!(ruby_val.is_kind_of(ruby.class_array()));
    }
}
