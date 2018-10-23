extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use serde_json::Value;
use std::collections::HashMap;
use std::usize;

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
// https://github.com/quilljs/delta#insert-operation
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

    pub fn insert<S: Into<Value>>(self, value: S, attributes: Attributes) -> Self {
        self.push(DeltaOperation::insert(value).attrs(attributes))
    }

    pub fn delete(self, length: usize) -> Self {
        self.push(DeltaOperation::delete(length))
    }

    pub fn retain(self, length: usize, attributes: Attributes) -> Self {
        self.push(DeltaOperation::insert(length).attrs(attributes))
    }

    pub fn push(mut self, op: DeltaOperation) -> Self {
        self.ops.push(op);
        self
    }

    /// Returns a new Delta representing the concatenation of this and another document Delta's operations.
    pub fn concat(&self, other: &Delta) -> Delta {
        let mut res = self.clone();
        if other.ops.len() > 0 {
            res = res.push(other.ops[0].clone());
            res.ops.extend(other.ops[1..].iter().cloned());
        }
        res
    }

    /// Returns a Delta representing the difference between two documents.
    /// Optionally, accepts a suggested index where change took place, often representing a cursor position before change.
    ///
    /// ```
    /// # extern crate quill_delta;
    /// # use quill_delta::*;
    ///
    /// let a: Delta = vec![insert("Hallo")].into();
    /// let b: Delta = vec![insert("Hallo!")].into();
    ///
    /// let diff = a.diff(&b, None);
    /// // { ops: [{ retain: 5 }, { insert: '!' }] }
    /// ```
    pub fn diff(&self, other: &Delta, index: Option<usize>) -> Result<Delta, DiffError> {
        let strings = [self.to_string()?, other.to_string()?];
        let mut delta = Delta::new();

        let diffResult = diff(strings[0].as_str(), strings[1].as_str(), index);

        Ok(delta)
    }

    /// Generate a string with all insert concatenated and non visible thing represented by `NULL_CHARACTER`.
    fn to_string(&self) -> Result<String, DiffError> {
        let mut res = String::new();
        for op in self.iter() {
            match op {
                &DeltaOperation {
                    kind: OpKind::Insert(Value::String(ref val)),
                    ..
                } => {
                    res.push_str(&val[..]);
                }
                &DeltaOperation {
                    kind: OpKind::Insert(_),
                    ..
                } => {
                    res.push(NULL_CHARACTER);
                }
                _ => return Err(DiffError::NotADocument),
            }
        }

        Ok(res)
    }

    /// Returns copy of delta with subset of operations.
    /// `start` - Start index of subset, default to 0
    /// `end` - End index of subset, defalts to rest of operations
    pub fn slice(&self, start: usize, end: usize) -> Delta {
        let mut ops = vec![];

        unimplemented!("all the handling code should go there");

        ops.into()
    }

    /// Returns a Delta that is equivalent to applying the operations of own Delta, followed by another Delta.
    /// `other` - Delta to compose
    pub fn compose(&self, other: &Delta) -> Delta {
        unimplemented!()
    }

    /// Transform given Delta against own operations.
    /// `other` - Delta to transform
    /// `priority` - Boolean used to break ties. If `true`, then `this` takes priority over `other`, that is, its
    /// actions are considered to happened "first".
    pub fn transform(&self, other: &Delta, priority: bool) -> Delta {
        unimplemented!()
    }

    /// Transform an index against the delta. Useful for representing cursor/selection positions.
    /// `index` - index to transform
    pub fn transform_position(&self, index: usize, priority: bool) -> usize {
        unimplemented!()
    }
}

//TODO: implement index operator for ranges.

/// placeholder char to embed in diff()
pub const NULL_CHARACTER: char = '\0';

pub enum DiffError {
    NotADocument,
}

fn diff(a: &str, b: &str, index: Option<usize>) {
    unimplemented!(
        "diff algorithm should be implemented, use something similar to fast-diff for node."
    )
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

    /// set multiple attributes at once
    pub fn attrs<V: Into<HashMap<String, Value>>>(mut self, values: V) -> Self {
        self.attributes = values.into();
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

pub struct DeltaIterator<'a> {
    ops: &'a [DeltaOperation],
    index: usize,
    offset: usize,
}
// https://github.com/quilljs/delta/blob/master/lib/op.js
impl<'a> DeltaIterator<'a> {
    pub fn new(ops: &'a [DeltaOperation]) -> Self {
        DeltaIterator {
            ops,
            index: 0,
            offset: 0,
        }
    }

    pub fn has_next(&self) -> bool {
        self.peek_len() < usize::MAX
    }

    pub fn next(&mut self, length: usize) {
        /*
        if (!length) length = Infinity;
        var nextOp = this.ops[this.index];
        if (nextOp) {
          var offset = this.offset;
          var opLength = lib.length(nextOp)
          if (length >= opLength - offset) {
            length = opLength - offset;
            this.index += 1;
            this.offset = 0;
          } else {
            this.offset += length;
          }
          if (typeof nextOp['delete'] === 'number') {
            return { 'delete': length };
          } else {
            var retOp = {};
            if (nextOp.attributes) {
              retOp.attributes = nextOp.attributes;
            }
            if (typeof nextOp.retain === 'number') {
              retOp.retain = length;
            } else if (typeof nextOp.insert === 'string') {
              retOp.insert = nextOp.insert.substr(offset, length);
            } else {
              // offset should === 0, length should === 1
              retOp.insert = nextOp.insert;
            }
            return retOp;
          }
        } else {
          return { retain: Infinity };
        }
              */
    }

    pub fn peek(&mut self) -> &'a DeltaOperation {
        &self.ops[self.index]
    }

    pub fn peek_len(&self) -> usize {
        if let Some(op) = self.ops.get(self.index) {
            op.len() - self.offset
        } else {
            usize::MAX
        }
    }

    pub fn peek_type(&mut self) -> Tp {
        match self.ops.get(self.index) {
            Some(DeltaOperation {
                kind: OpKind::Insert(_),
                ..
            }) => Tp::Insert,
            Some(DeltaOperation {
                kind: OpKind::Delete(_),
                ..
            }) => Tp::Delete,
            _ => Tp::Retain,
        }
    }

    pub fn reset(&mut self) {
        /*
           if (!this.hasNext()) {
          return [];
        } else if (this.offset === 0) {
          return this.ops.slice(this.index);
        } else {
          var offset = this.offset;
          var index = this.index;
          var next = this.next();
          var rest = this.ops.slice(this.index);
          this.offset = offset;
          this.index = index;
          return [next].concat(rest);
        }
              */
    }
}

pub enum Tp {
    Delete,
    Retain,
    Insert,
}

#[test]
fn deserialize_delta_operation() {
    let _: DeltaOperation = serde_json::from_str(r#"{ "insert": "Hallo" }"#).unwrap();
    let _: DeltaOperation = serde_json::from_str(r#"{ "retain": 10 }"#).unwrap();
    let _: DeltaOperation = serde_json::from_str(r#"{ "delete": 10 }"#).unwrap();
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
