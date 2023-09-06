use std::ops::{DerefMut, Deref};

pub struct RepeatDisplay<T> {
    pub val: T,
    pub amount: usize,
}


impl<T: core::fmt::Display> core::fmt::Display for RepeatDisplay<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.amount {
            self.val.fmt(f)?;
        };
        Ok(())
    }
}


impl<T: core::fmt::Debug> core::fmt::Debug for RepeatDisplay<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.amount {
            self.val.fmt(f)?;
        };
        Ok(())
    }
}


impl<T> Deref for RepeatDisplay<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}


impl<T> DerefMut for RepeatDisplay<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.val
    }
}
