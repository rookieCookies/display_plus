use repeated_display::RepeatDisplay;

pub mod repeated_display;

pub trait DisplayPlus {
    /// 
    /// Display `self` amount times
    ///
    fn repeated(self, amount: usize) -> RepeatDisplay<Self> where Self: Sized {
        RepeatDisplay { val: self, amount }
    }
}


impl<T> DisplayPlus for T {}
