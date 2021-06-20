/// An instance of a [`Module`], created by the [`Module::instance`] method.
///
/// # Examples
///
/// ```
/// use kaze::*;
///
/// let c = Context::new();
///
/// // Inner module (simple pass-through)
/// let inner = c.module("Inner");
/// inner.output("o", inner.input("i", 32));
///
/// // Outer module (wraps a single `Inner` instance)
/// let outer = c.module("Outer");
/// let inner_inst = outer.instance("inner_inst", "Inner");
/// inner_inst.drive_input("i", outer.input("i", 32));
/// outer.output("o", inner_inst.output("o"));
/// ```
pub struct Instance<'a> {
}

impl<'a> Instance<'a> {
    /// Drives the input of this [`Module`] `Instance` specified by `name` with the given [`Signal`].
    ///
    /// # Panics
    ///
    /// Panics if `i` is from a different [`Module`] than `self`, if `name` specifies an input that doesn't exist on this `Instance`'s [`Module`], if this input is already driven on this `Instance`, or if `i`'s bit width differs from that of the input.
    ///
    /// # Examples
    ///
    /// ```
    /// use kaze::*;
    ///
    /// let c = Context::new();
    ///
    /// let inner = c.module("Inner");
    /// inner.output("o", inner.input("i", 32));
    ///
    /// let outer = c.module("Outer");
    /// let inner_inst = outer.instance("inner_inst", "Inner");
    /// // Drive inner_inst's "i" input with a 32-bit literal
    /// inner_inst.drive_input("i", outer.lit(0xfadebabeu32, 32));
    /// ```
    pub fn drive_input<S: Into<String>>(&'a self, name: S, i: &'a Signal<'a>) {
    }
}
