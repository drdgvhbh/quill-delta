extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use serde_json::Value;
use std::collections::HashMap;

/// Wrapper to manipulate Delta easily
/// ```
/// extern crate quill_delta;
/// use quill_delta::*;
///
/// let delta = Delta::new()
///     .retain(2, none())
///     .insert("Hallo Welt", none());
///
/// let delta: Delta = vec![
///     retain(2),
///     insert("Hallo Welt")
/// ].into();
///
/// ```
// https://github.com/maximkornilov/types-quill-delta/blob/master/index.d.ts
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Delta {
    #[serde(flatten)]
    pub ops: Vec<DeltaOperation>,
}

pub fn none() -> Attributes {
    HashMap::new()
}

impl Delta {
    pub fn new() -> Self {
        Delta { ops: Vec::new() }
    }

    pub fn insert<S: Into<String>>(mut self, text: S, attributes: Attributes) -> Self {
        self
    }

    pub fn delete(mut self, length: usize) -> Self {
        self
    }

    pub fn retain(mut self, length: usize, attributes: Attributes) -> Self {
        self
    }

    pub fn push(mut self, op: DeltaOperation) -> Self {
        self
    }

    pub fn chop(mut self) {}
}

impl std::ops::Deref for Delta {
    type Target = Vec<DeltaOperation>;

    fn deref(&self) -> &Self::Target {
        &self.ops
    }
}

impl std::ops::DerefMut for Delta {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ops
    }
}

impl From<Vec<DeltaOperation>> for Delta {
    fn from(ops: Vec<DeltaOperation>) -> Delta {
        Delta { ops }
    }
}

impl std::iter::FromIterator<DeltaOperation> for Delta {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = DeltaOperation>,
    {
        let res: Vec<_> = iter.into_iter().collect();
        res.into()
    }
}

type Attributes = HashMap<String, Value>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeltaOperation {
    #[serde(flatten)]
    pub kind: OpKind,
    #[serde(default, skip_serializing_if = "empty")]
    pub attributes: Attributes,
}

/// test weather the attributes are empty and if it therfore can be skipped
fn empty(value: &Attributes) -> bool {
    value.len() == 0
}

impl DeltaOperation {
    #[inline(always)]
    pub fn insert<V: Into<Value>>(value: V) -> Self {
        DeltaOperation {
            kind: OpKind::Insert(value.into()),
            attributes: HashMap::new(),
        }
    }

    #[inline(always)]
    pub fn retain(value: usize) -> Self {
        DeltaOperation {
            kind: OpKind::Retain(value),
            attributes: HashMap::new(),
        }
    }

    /// Delete a value from the input
    #[inline(always)]
    pub fn delete(value: usize) -> Self {
        DeltaOperation {
            kind: OpKind::Delete(value),
            attributes: HashMap::new(),
        }
    }

    /// set the attribute in a shorthand way
    /// ```
    /// extern crate quill_delta;
    /// use quill_delta::DeltaOperation;
    /// let op = DeltaOperation::insert("Hallo")
    ///     .attr("font", "green")
    ///     .attr("size", 10);
    /// ```
    #[inline(always)]
    pub fn attr<K: Into<String>, V: Into<Value>>(mut self, key: K, value: V) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    /// get the length
    #[inline(always)]
    pub fn len(&self) -> usize {
        match self.kind {
            OpKind::Delete(len) => len,
            OpKind::Retain(len) => len,
            OpKind::Insert(Value::String(ref val)) => val.len(),
            _ => unimplemented!(),
        }
    }
}

#[inline(always)]
pub fn insert<V: Into<Value>>(value: V) -> DeltaOperation {
    DeltaOperation::insert(value)
}

#[inline(always)]
pub fn retain(value: usize) -> DeltaOperation {
    DeltaOperation::retain(value)
}

#[inline(always)]
pub fn delete(value: usize) -> DeltaOperation {
    DeltaOperation::delete(value)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OpKind {
    Insert(Value),
    Retain(usize),
    Delete(usize),
}

#[test]
fn deserialize_delta_operation() {
    let op: DeltaOperation = serde_json::from_str(r#"{ "insert": "Hallo" }"#).unwrap();
    let op: DeltaOperation = serde_json::from_str(r#"{ "retain": 10 }"#).unwrap();
    let op: DeltaOperation = serde_json::from_str(r#"{ "delete": 10 }"#).unwrap();
}

#[test]
fn serilize_delta_operation() {
    assert_eq!(
        serde_json::to_string(&insert("Hallo")).unwrap(),
        r#"{"insert":"Hallo"}"#
    );

    assert_eq!(
        serde_json::to_string(&delete(100)).unwrap(),
        r#"{"delete":100}"#
    );

    assert_eq!(
        serde_json::to_string(&retain(100)).unwrap(),
        r#"{"retain":100}"#
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
