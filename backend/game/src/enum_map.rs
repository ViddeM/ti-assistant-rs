use std::{
    collections::HashMap,
    fmt::{self, Debug},
    hash::Hash,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize, Serializer};
use ts_rs::{Dummy, TS};

/// A wrapper around [HashMap] that generates correct [TS] bindings for enum keys.
///
/// When using a typescript string union (i.e. a rust enum) as a map key (`K`), the key type needs
/// to be specified as [key in K]. Don't ask me why.
/// ```typescript
/// // type of HashMap<K, V>:
/// { [key: K]: V }
///
/// // type of EnumMap<K, V>:
/// { [key in K]: V }
/// ```
#[derive(Clone)]
pub struct EnumMap<K, V> {
    inner: HashMap<K, V>,
}

impl<K: Eq + Hash, V: Eq> Eq for EnumMap<K, V> {}
impl<K: Eq + Hash, V: PartialEq> PartialEq for EnumMap<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<K, V> Debug for EnumMap<K, V>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<K, V> Serialize for EnumMap<K, V>
where
    K: Serialize,
    V: Serialize,
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.inner.serialize(serializer)
    }
}

impl<'a, K, V> Deserialize<'a> for EnumMap<K, V>
where
    K: Deserialize<'a> + Hash + Eq,
    V: Deserialize<'a>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'a>,
    {
        Ok(Self {
            inner: Deserialize::deserialize(deserializer)?,
        })
    }
}

impl<K: TS, V: TS> TS for EnumMap<K, V> {
    type WithoutGenerics = EnumMap<Dummy, Dummy>;

    fn name() -> String {
        format!("{{ [key in {}]: {} }}", K::name(), V::name())
    }

    fn inline() -> String {
        format!("{{ [key in {}]: {} }}", K::name(), V::name())
    }

    fn decl() -> String {
        panic!("{} cannot be declared", Self::name())
    }

    fn decl_concrete() -> String {
        panic!("{} cannot be declared", Self::name())
    }

    fn inline_flattened() -> String {
        panic!("{} cannot be flattened", Self::name())
    }

    fn dependency_types() -> impl ts_rs::typelist::TypeList
    where
        Self: 'static,
    {
        HashMap::<K, V>::dependency_types()
    }

    fn generics() -> impl ts_rs::typelist::TypeList
    where
        Self: 'static,
    {
        HashMap::<K, V>::generics()
    }
}

impl<K, V> Default for EnumMap<K, V> {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl<K: Hash + Eq, V> FromIterator<(K, V)> for EnumMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self {
            inner: FromIterator::from_iter(iter),
        }
    }
}

impl<K, V> EnumMap<K, V> {
    /// Create an empty [EnumMap].
    pub fn new() -> Self {
        Self::default()
    }
}

impl<K, V> Deref for EnumMap<K, V> {
    type Target = HashMap<K, V>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<K, V> DerefMut for EnumMap<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<const N: usize, K: Hash + Eq, V> From<[(K, V); N]> for EnumMap<K, V> {
    fn from(array: [(K, V); N]) -> Self {
        Self {
            inner: From::from(array),
        }
    }
}
