use mlua::prelude::*;

#[derive(PartialEq)]
pub(crate) struct LuaVector {
    inner: Vec<LuaValue>,
}

impl LuaVector {
    const MAX_LEN: usize = LuaInteger::MAX as usize;

    pub(crate) fn new(size: LuaInteger, filler: LuaValue) -> Result<Self, String> {
        if size < 0 {
            return Err(format!("Invalid Vector len {}.", size));
        }
        Ok(Self {
            inner: vec![filler; size as usize],
        })
    }

    pub(crate) fn of(values: Vec<LuaValue>) -> Self {
        Self { inner: values }
    }

    pub(crate) fn get(&self, index: LuaInteger) -> Option<LuaValue> {
        self.zero_based_index(index).map(|idx| self.inner[idx].clone())
    }

    pub(crate) fn len(&self) -> LuaInteger {
        self.inner.len() as LuaInteger
    }

    pub(crate) fn push(&mut self, value: LuaValue) -> bool {
        if self.inner.len() < Self::MAX_LEN {
            self.inner.push(value);
            return true;
        }
        false
    }

    pub(crate) fn pop(&mut self) -> Option<LuaValue> {
        self.inner.pop()
    }

    pub(crate) fn set(&mut self, index: LuaInteger, new_value: LuaValue) -> bool {
        match self.zero_based_index(index) {
            Some(idx) => {
                self.inner[idx] = new_value;
                true
            }
            None => false,
        }
    }

    pub(crate) fn is_same(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }

    fn zero_based_index(&self, one_based_index: LuaInteger) -> Option<usize> {
        let len = self.inner.len() as LuaInteger;

        match one_based_index {
            0 => None,
            idx if idx > len => None,
            idx if idx < -len => None,
            idx if idx > 0 => Some((idx - 1) as usize),
            idx => Some((idx + len + 1) as usize),
        }
    }
}
