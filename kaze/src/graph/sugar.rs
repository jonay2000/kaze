use super::signal::*;

/// **UNSTABLE:** Provides a convenient way to write conditional combinational logic.
///
/// # Panics
///
/// Since this construct wraps the returned values with [`Signal::mux`], any panic conditions from that method apply to the generated code as well.
///
/// # Examples
///
/// ```
/// use kaze::*;
///
/// let p = Context::new();
///
/// let m = p.module("m", "MyModule");
/// let i = m.input("i", 1);
/// let invert = m.input("invert", 1);
/// let o = if_(invert, {
///     !i
/// }).else_({
///     i
/// });
/// m.output("o", o);
/// ```
// TODO: Can we constrain T more than this to make sure it's only a supported type?
pub fn if_<'a, S: Into<&'a Signal<'a>>, T>(cond: S, when_true: T) -> If<'a, T> {
    let cond = cond.into();
    If::new(cond, when_true)
}

#[doc(hidden)]
pub struct If<'a, T> {
    cond: &'a Signal<'a>,
    when_true: T,
}

impl<'a, T> If<'a, T> {
    fn new(cond: &'a Signal<'a>, when_true: T) -> If<'a, T> {
        If { cond, when_true }
    }

    pub fn else_if<S: Into<&'a Signal<'a>>>(self, cond: S, when_true: T) -> ElseIf<'a, T> {
        let cond = cond.into();
        ElseIf {
            parent: ElseIfParent::If(self),
            cond,
            when_true,
        }
    }
}

// TODO: Use more S's or are R's OK here? I don't like it either way..
impl<'a, R: Into<&'a Signal<'a>>> If<'a, R> {
    pub fn else_<S: Into<&'a Signal<'a>>>(self, when_false: S) -> &'a Signal<'a> {
        self.cond.mux(self.when_true, when_false)
    }
}

// TODO: Come up with a nice way to generate these definitions with macros
impl<'a, R: Into<&'a Signal<'a>>> If<'a, (R,)> {
    pub fn else_<S: Into<&'a Signal<'a>>>(self, when_false: (S,)) -> (&'a Signal<'a>,) {
        (self.cond.mux(self.when_true.0, when_false.0),)
    }
}

impl<'a, R1: Into<&'a Signal<'a>>, R2: Into<&'a Signal<'a>>> If<'a, (R1, R2)> {
    pub fn else_<S1: Into<&'a Signal<'a>>, S2: Into<&'a Signal<'a>>>(
        self,
        when_false: (S1, S2),
    ) -> (&'a Signal<'a>, &'a Signal<'a>) {
        (
            self.cond.mux(self.when_true.0, when_false.0),
            self.cond.mux(self.when_true.1, when_false.1),
        )
    }
}

impl<'a, R1: Into<&'a Signal<'a>>, R2: Into<&'a Signal<'a>>, R3: Into<&'a Signal<'a>>>
    If<'a, (R1, R2, R3)>
{
    pub fn else_<S1: Into<&'a Signal<'a>>, S2: Into<&'a Signal<'a>>, S3: Into<&'a Signal<'a>>>(
        self,
        when_false: (S1, S2, S3),
    ) -> (&'a Signal<'a>, &'a Signal<'a>, &'a Signal<'a>) {
        (
            self.cond.mux(self.when_true.0, when_false.0),
            self.cond.mux(self.when_true.1, when_false.1),
            self.cond.mux(self.when_true.2, when_false.2),
        )
    }
}

impl<
        'a,
        R1: Into<&'a Signal<'a>>,
        R2: Into<&'a Signal<'a>>,
        R3: Into<&'a Signal<'a>>,
        R4: Into<&'a Signal<'a>>,
    > If<'a, (R1, R2, R3, R4)>
{
    pub fn else_<
        S1: Into<&'a Signal<'a>>,
        S2: Into<&'a Signal<'a>>,
        S3: Into<&'a Signal<'a>>,
        S4: Into<&'a Signal<'a>>,
    >(
        self,
        when_false: (S1, S2, S3, S4),
    ) -> (
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
    ) {
        (
            self.cond.mux(self.when_true.0, when_false.0),
            self.cond.mux(self.when_true.1, when_false.1),
            self.cond.mux(self.when_true.2, when_false.2),
            self.cond.mux(self.when_true.3, when_false.3),
        )
    }
}

impl<
        'a,
        R1: Into<&'a Signal<'a>>,
        R2: Into<&'a Signal<'a>>,
        R3: Into<&'a Signal<'a>>,
        R4: Into<&'a Signal<'a>>,
        R5: Into<&'a Signal<'a>>,
    > If<'a, (R1, R2, R3, R4, R5)>
{
    pub fn else_<
        S1: Into<&'a Signal<'a>>,
        S2: Into<&'a Signal<'a>>,
        S3: Into<&'a Signal<'a>>,
        S4: Into<&'a Signal<'a>>,
        S5: Into<&'a Signal<'a>>,
    >(
        self,
        when_false: (S1, S2, S3, S4, S5),
    ) -> (
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
    ) {
        (
            self.cond.mux(self.when_true.0, when_false.0),
            self.cond.mux(self.when_true.1, when_false.1),
            self.cond.mux(self.when_true.2, when_false.2),
            self.cond.mux(self.when_true.3, when_false.3),
            self.cond.mux(self.when_true.4, when_false.4),
        )
    }
}

impl<
        'a,
        R1: Into<&'a Signal<'a>>,
        R2: Into<&'a Signal<'a>>,
        R3: Into<&'a Signal<'a>>,
        R4: Into<&'a Signal<'a>>,
        R5: Into<&'a Signal<'a>>,
        R6: Into<&'a Signal<'a>>,
    > If<'a, (R1, R2, R3, R4, R5, R6)>
{
    pub fn else_<
        S1: Into<&'a Signal<'a>>,
        S2: Into<&'a Signal<'a>>,
        S3: Into<&'a Signal<'a>>,
        S4: Into<&'a Signal<'a>>,
        S5: Into<&'a Signal<'a>>,
        S6: Into<&'a Signal<'a>>,
    >(
        self,
        when_false: (S1, S2, S3, S4, S5, S6),
    ) -> (
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
    ) {
        (
            self.cond.mux(self.when_true.0, when_false.0),
            self.cond.mux(self.when_true.1, when_false.1),
            self.cond.mux(self.when_true.2, when_false.2),
            self.cond.mux(self.when_true.3, when_false.3),
            self.cond.mux(self.when_true.4, when_false.4),
            self.cond.mux(self.when_true.5, when_false.5),
        )
    }
}

impl<
        'a,
        R1: Into<&'a Signal<'a>>,
        R2: Into<&'a Signal<'a>>,
        R3: Into<&'a Signal<'a>>,
        R4: Into<&'a Signal<'a>>,
        R5: Into<&'a Signal<'a>>,
        R6: Into<&'a Signal<'a>>,
        R7: Into<&'a Signal<'a>>,
    > If<'a, (R1, R2, R3, R4, R5, R6, R7)>
{
    pub fn else_<
        S1: Into<&'a Signal<'a>>,
        S2: Into<&'a Signal<'a>>,
        S3: Into<&'a Signal<'a>>,
        S4: Into<&'a Signal<'a>>,
        S5: Into<&'a Signal<'a>>,
        S6: Into<&'a Signal<'a>>,
        S7: Into<&'a Signal<'a>>,
    >(
        self,
        when_false: (S1, S2, S3, S4, S5, S6, S7),
    ) -> (
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
    ) {
        (
            self.cond.mux(self.when_true.0, when_false.0),
            self.cond.mux(self.when_true.1, when_false.1),
            self.cond.mux(self.when_true.2, when_false.2),
            self.cond.mux(self.when_true.3, when_false.3),
            self.cond.mux(self.when_true.4, when_false.4),
            self.cond.mux(self.when_true.5, when_false.5),
            self.cond.mux(self.when_true.6, when_false.6),
        )
    }
}

impl<
        'a,
        R1: Into<&'a Signal<'a>>,
        R2: Into<&'a Signal<'a>>,
        R3: Into<&'a Signal<'a>>,
        R4: Into<&'a Signal<'a>>,
        R5: Into<&'a Signal<'a>>,
        R6: Into<&'a Signal<'a>>,
        R7: Into<&'a Signal<'a>>,
        R8: Into<&'a Signal<'a>>,
    > If<'a, (R1, R2, R3, R4, R5, R6, R7, R8)>
{
    pub fn else_<
        S1: Into<&'a Signal<'a>>,
        S2: Into<&'a Signal<'a>>,
        S3: Into<&'a Signal<'a>>,
        S4: Into<&'a Signal<'a>>,
        S5: Into<&'a Signal<'a>>,
        S6: Into<&'a Signal<'a>>,
        S7: Into<&'a Signal<'a>>,
        S8: Into<&'a Signal<'a>>,
    >(
        self,
        when_false: (S1, S2, S3, S4, S5, S6, S7, S8),
    ) -> (
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
    ) {
        (
            self.cond.mux(self.when_true.0, when_false.0),
            self.cond.mux(self.when_true.1, when_false.1),
            self.cond.mux(self.when_true.2, when_false.2),
            self.cond.mux(self.when_true.3, when_false.3),
            self.cond.mux(self.when_true.4, when_false.4),
            self.cond.mux(self.when_true.5, when_false.5),
            self.cond.mux(self.when_true.6, when_false.6),
            self.cond.mux(self.when_true.7, when_false.7),
        )
    }
}

impl<
        'a,
        R1: Into<&'a Signal<'a>>,
        R2: Into<&'a Signal<'a>>,
        R3: Into<&'a Signal<'a>>,
        R4: Into<&'a Signal<'a>>,
        R5: Into<&'a Signal<'a>>,
        R6: Into<&'a Signal<'a>>,
        R7: Into<&'a Signal<'a>>,
        R8: Into<&'a Signal<'a>>,
        R9: Into<&'a Signal<'a>>,
    > If<'a, (R1, R2, R3, R4, R5, R6, R7, R8, R9)>
{
    pub fn else_<
        S1: Into<&'a Signal<'a>>,
        S2: Into<&'a Signal<'a>>,
        S3: Into<&'a Signal<'a>>,
        S4: Into<&'a Signal<'a>>,
        S5: Into<&'a Signal<'a>>,
        S6: Into<&'a Signal<'a>>,
        S7: Into<&'a Signal<'a>>,
        S8: Into<&'a Signal<'a>>,
        S9: Into<&'a Signal<'a>>,
    >(
        self,
        when_false: (S1, S2, S3, S4, S5, S6, S7, S8, S9),
    ) -> (
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
    ) {
        (
            self.cond.mux(self.when_true.0, when_false.0),
            self.cond.mux(self.when_true.1, when_false.1),
            self.cond.mux(self.when_true.2, when_false.2),
            self.cond.mux(self.when_true.3, when_false.3),
            self.cond.mux(self.when_true.4, when_false.4),
            self.cond.mux(self.when_true.5, when_false.5),
            self.cond.mux(self.when_true.6, when_false.6),
            self.cond.mux(self.when_true.7, when_false.7),
            self.cond.mux(self.when_true.8, when_false.8),
        )
    }
}

impl<
        'a,
        R1: Into<&'a Signal<'a>>,
        R2: Into<&'a Signal<'a>>,
        R3: Into<&'a Signal<'a>>,
        R4: Into<&'a Signal<'a>>,
        R5: Into<&'a Signal<'a>>,
        R6: Into<&'a Signal<'a>>,
        R7: Into<&'a Signal<'a>>,
        R8: Into<&'a Signal<'a>>,
        R9: Into<&'a Signal<'a>>,
        R10: Into<&'a Signal<'a>>,
    > If<'a, (R1, R2, R3, R4, R5, R6, R7, R8, R9, R10)>
{
    pub fn else_<
        S1: Into<&'a Signal<'a>>,
        S2: Into<&'a Signal<'a>>,
        S3: Into<&'a Signal<'a>>,
        S4: Into<&'a Signal<'a>>,
        S5: Into<&'a Signal<'a>>,
        S6: Into<&'a Signal<'a>>,
        S7: Into<&'a Signal<'a>>,
        S8: Into<&'a Signal<'a>>,
        S9: Into<&'a Signal<'a>>,
        S10: Into<&'a Signal<'a>>,
    >(
        self,
        when_false: (S1, S2, S3, S4, S5, S6, S7, S8, S9, S10),
    ) -> (
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
    ) {
        (
            self.cond.mux(self.when_true.0, when_false.0),
            self.cond.mux(self.when_true.1, when_false.1),
            self.cond.mux(self.when_true.2, when_false.2),
            self.cond.mux(self.when_true.3, when_false.3),
            self.cond.mux(self.when_true.4, when_false.4),
            self.cond.mux(self.when_true.5, when_false.5),
            self.cond.mux(self.when_true.6, when_false.6),
            self.cond.mux(self.when_true.7, when_false.7),
            self.cond.mux(self.when_true.8, when_false.8),
            self.cond.mux(self.when_true.9, when_false.9),
        )
    }
}

impl<
        'a,
        R1: Into<&'a Signal<'a>>,
        R2: Into<&'a Signal<'a>>,
        R3: Into<&'a Signal<'a>>,
        R4: Into<&'a Signal<'a>>,
        R5: Into<&'a Signal<'a>>,
        R6: Into<&'a Signal<'a>>,
        R7: Into<&'a Signal<'a>>,
        R8: Into<&'a Signal<'a>>,
        R9: Into<&'a Signal<'a>>,
        R10: Into<&'a Signal<'a>>,
        R11: Into<&'a Signal<'a>>,
    > If<'a, (R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11)>
{
    pub fn else_<
        S1: Into<&'a Signal<'a>>,
        S2: Into<&'a Signal<'a>>,
        S3: Into<&'a Signal<'a>>,
        S4: Into<&'a Signal<'a>>,
        S5: Into<&'a Signal<'a>>,
        S6: Into<&'a Signal<'a>>,
        S7: Into<&'a Signal<'a>>,
        S8: Into<&'a Signal<'a>>,
        S9: Into<&'a Signal<'a>>,
        S10: Into<&'a Signal<'a>>,
        S11: Into<&'a Signal<'a>>,
    >(
        self,
        when_false: (S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11),
    ) -> (
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
    ) {
        (
            self.cond.mux(self.when_true.0, when_false.0),
            self.cond.mux(self.when_true.1, when_false.1),
            self.cond.mux(self.when_true.2, when_false.2),
            self.cond.mux(self.when_true.3, when_false.3),
            self.cond.mux(self.when_true.4, when_false.4),
            self.cond.mux(self.when_true.5, when_false.5),
            self.cond.mux(self.when_true.6, when_false.6),
            self.cond.mux(self.when_true.7, when_false.7),
            self.cond.mux(self.when_true.8, when_false.8),
            self.cond.mux(self.when_true.9, when_false.9),
            self.cond.mux(self.when_true.10, when_false.10),
        )
    }
}

impl<
        'a,
        R1: Into<&'a Signal<'a>>,
        R2: Into<&'a Signal<'a>>,
        R3: Into<&'a Signal<'a>>,
        R4: Into<&'a Signal<'a>>,
        R5: Into<&'a Signal<'a>>,
        R6: Into<&'a Signal<'a>>,
        R7: Into<&'a Signal<'a>>,
        R8: Into<&'a Signal<'a>>,
        R9: Into<&'a Signal<'a>>,
        R10: Into<&'a Signal<'a>>,
        R11: Into<&'a Signal<'a>>,
        R12: Into<&'a Signal<'a>>,
    > If<'a, (R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12)>
{
    pub fn else_<
        S1: Into<&'a Signal<'a>>,
        S2: Into<&'a Signal<'a>>,
        S3: Into<&'a Signal<'a>>,
        S4: Into<&'a Signal<'a>>,
        S5: Into<&'a Signal<'a>>,
        S6: Into<&'a Signal<'a>>,
        S7: Into<&'a Signal<'a>>,
        S8: Into<&'a Signal<'a>>,
        S9: Into<&'a Signal<'a>>,
        S10: Into<&'a Signal<'a>>,
        S11: Into<&'a Signal<'a>>,
        S12: Into<&'a Signal<'a>>,
    >(
        self,
        when_false: (S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12),
    ) -> (
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
    ) {
        (
            self.cond.mux(self.when_true.0, when_false.0),
            self.cond.mux(self.when_true.1, when_false.1),
            self.cond.mux(self.when_true.2, when_false.2),
            self.cond.mux(self.when_true.3, when_false.3),
            self.cond.mux(self.when_true.4, when_false.4),
            self.cond.mux(self.when_true.5, when_false.5),
            self.cond.mux(self.when_true.6, when_false.6),
            self.cond.mux(self.when_true.7, when_false.7),
            self.cond.mux(self.when_true.8, when_false.8),
            self.cond.mux(self.when_true.9, when_false.9),
            self.cond.mux(self.when_true.10, when_false.10),
            self.cond.mux(self.when_true.11, when_false.11),
        )
    }
}

enum ElseIfParent<'a, T> {
    If(If<'a, T>),
    ElseIf(Box<ElseIf<'a, T>>),
}

#[doc(hidden)]
pub struct ElseIf<'a, T> {
    parent: ElseIfParent<'a, T>,
    cond: &'a Signal<'a>,
    when_true: T,
}

impl<'a, T> ElseIf<'a, T> {
    pub fn else_if<S: Into<&'a Signal<'a>>>(self, cond: S, when_true: T) -> ElseIf<'a, T> {
        let cond = cond.into();
        ElseIf {
            parent: ElseIfParent::ElseIf(Box::new(self)),
            cond,
            when_true,
        }
    }
}

impl<'a, R: Into<&'a Signal<'a>>> ElseIf<'a, R> {
    pub fn else_<S: Into<&'a Signal<'a>>>(self, when_false: S) -> &'a Signal<'a> {
        let ret = self.cond.mux(self.when_true, when_false);
        match self.parent {
            ElseIfParent::If(parent) => parent.else_(ret),
            ElseIfParent::ElseIf(parent) => parent.else_(ret),
        }
    }
}

// TODO: Come up with a nice way to generate these definitions with macros
impl<'a, R: Into<&'a Signal<'a>>> ElseIf<'a, (R,)> {
    pub fn else_<S: Into<&'a Signal<'a>>>(self, when_false: (S,)) -> (&'a Signal<'a>,) {
        let ret = (self.cond.mux(self.when_true.0, when_false.0),);
        match self.parent {
            ElseIfParent::If(parent) => parent.else_(ret),
            ElseIfParent::ElseIf(parent) => parent.else_(ret),
        }
    }
}

impl<'a, R1: Into<&'a Signal<'a>>, R2: Into<&'a Signal<'a>>> ElseIf<'a, (R1, R2)> {
    pub fn else_<S1: Into<&'a Signal<'a>>, S2: Into<&'a Signal<'a>>>(
        self,
        when_false: (S1, S2),
    ) -> (&'a Signal<'a>, &'a Signal<'a>) {
        let ret = (
            self.cond.mux(self.when_true.0, when_false.0),
            self.cond.mux(self.when_true.1, when_false.1),
        );
        match self.parent {
            ElseIfParent::If(parent) => parent.else_(ret),
            ElseIfParent::ElseIf(parent) => parent.else_(ret),
        }
    }
}

impl<'a, R1: Into<&'a Signal<'a>>, R2: Into<&'a Signal<'a>>, R3: Into<&'a Signal<'a>>>
    ElseIf<'a, (R1, R2, R3)>
{
    pub fn else_<S1: Into<&'a Signal<'a>>, S2: Into<&'a Signal<'a>>, S3: Into<&'a Signal<'a>>>(
        self,
        when_false: (S1, S2, S3),
    ) -> (&'a Signal<'a>, &'a Signal<'a>, &'a Signal<'a>) {
        let ret = (
            self.cond.mux(self.when_true.0, when_false.0),
            self.cond.mux(self.when_true.1, when_false.1),
            self.cond.mux(self.when_true.2, when_false.2),
        );
        match self.parent {
            ElseIfParent::If(parent) => parent.else_(ret),
            ElseIfParent::ElseIf(parent) => parent.else_(ret),
        }
    }
}

impl<
        'a,
        R1: Into<&'a Signal<'a>>,
        R2: Into<&'a Signal<'a>>,
        R3: Into<&'a Signal<'a>>,
        R4: Into<&'a Signal<'a>>,
    > ElseIf<'a, (R1, R2, R3, R4)>
{
    pub fn else_<
        S1: Into<&'a Signal<'a>>,
        S2: Into<&'a Signal<'a>>,
        S3: Into<&'a Signal<'a>>,
        S4: Into<&'a Signal<'a>>,
    >(
        self,
        when_false: (S1, S2, S3, S4),
    ) -> (
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
    ) {
        let ret = (
            self.cond.mux(self.when_true.0, when_false.0),
            self.cond.mux(self.when_true.1, when_false.1),
            self.cond.mux(self.when_true.2, when_false.2),
            self.cond.mux(self.when_true.3, when_false.3),
        );
        match self.parent {
            ElseIfParent::If(parent) => parent.else_(ret),
            ElseIfParent::ElseIf(parent) => parent.else_(ret),
        }
    }
}

impl<
        'a,
        R1: Into<&'a Signal<'a>>,
        R2: Into<&'a Signal<'a>>,
        R3: Into<&'a Signal<'a>>,
        R4: Into<&'a Signal<'a>>,
        R5: Into<&'a Signal<'a>>,
    > ElseIf<'a, (R1, R2, R3, R4, R5)>
{
    pub fn else_<
        S1: Into<&'a Signal<'a>>,
        S2: Into<&'a Signal<'a>>,
        S3: Into<&'a Signal<'a>>,
        S4: Into<&'a Signal<'a>>,
        S5: Into<&'a Signal<'a>>,
    >(
        self,
        when_false: (S1, S2, S3, S4, S5),
    ) -> (
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
    ) {
        let ret = (
            self.cond.mux(self.when_true.0, when_false.0),
            self.cond.mux(self.when_true.1, when_false.1),
            self.cond.mux(self.when_true.2, when_false.2),
            self.cond.mux(self.when_true.3, when_false.3),
            self.cond.mux(self.when_true.4, when_false.4),
        );
        match self.parent {
            ElseIfParent::If(parent) => parent.else_(ret),
            ElseIfParent::ElseIf(parent) => parent.else_(ret),
        }
    }
}

impl<
        'a,
        R1: Into<&'a Signal<'a>>,
        R2: Into<&'a Signal<'a>>,
        R3: Into<&'a Signal<'a>>,
        R4: Into<&'a Signal<'a>>,
        R5: Into<&'a Signal<'a>>,
        R6: Into<&'a Signal<'a>>,
    > ElseIf<'a, (R1, R2, R3, R4, R5, R6)>
{
    pub fn else_<
        S1: Into<&'a Signal<'a>>,
        S2: Into<&'a Signal<'a>>,
        S3: Into<&'a Signal<'a>>,
        S4: Into<&'a Signal<'a>>,
        S5: Into<&'a Signal<'a>>,
        S6: Into<&'a Signal<'a>>,
    >(
        self,
        when_false: (S1, S2, S3, S4, S5, S6),
    ) -> (
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
    ) {
        let ret = (
            self.cond.mux(self.when_true.0, when_false.0),
            self.cond.mux(self.when_true.1, when_false.1),
            self.cond.mux(self.when_true.2, when_false.2),
            self.cond.mux(self.when_true.3, when_false.3),
            self.cond.mux(self.when_true.4, when_false.4),
            self.cond.mux(self.when_true.5, when_false.5),
        );
        match self.parent {
            ElseIfParent::If(parent) => parent.else_(ret),
            ElseIfParent::ElseIf(parent) => parent.else_(ret),
        }
    }
}

impl<
        'a,
        R1: Into<&'a Signal<'a>>,
        R2: Into<&'a Signal<'a>>,
        R3: Into<&'a Signal<'a>>,
        R4: Into<&'a Signal<'a>>,
        R5: Into<&'a Signal<'a>>,
        R6: Into<&'a Signal<'a>>,
        R7: Into<&'a Signal<'a>>,
    > ElseIf<'a, (R1, R2, R3, R4, R5, R6, R7)>
{
    pub fn else_<
        S1: Into<&'a Signal<'a>>,
        S2: Into<&'a Signal<'a>>,
        S3: Into<&'a Signal<'a>>,
        S4: Into<&'a Signal<'a>>,
        S5: Into<&'a Signal<'a>>,
        S6: Into<&'a Signal<'a>>,
        S7: Into<&'a Signal<'a>>,
    >(
        self,
        when_false: (S1, S2, S3, S4, S5, S6, S7),
    ) -> (
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
    ) {
        let ret = (
            self.cond.mux(self.when_true.0, when_false.0),
            self.cond.mux(self.when_true.1, when_false.1),
            self.cond.mux(self.when_true.2, when_false.2),
            self.cond.mux(self.when_true.3, when_false.3),
            self.cond.mux(self.when_true.4, when_false.4),
            self.cond.mux(self.when_true.5, when_false.5),
            self.cond.mux(self.when_true.6, when_false.6),
        );
        match self.parent {
            ElseIfParent::If(parent) => parent.else_(ret),
            ElseIfParent::ElseIf(parent) => parent.else_(ret),
        }
    }
}

impl<
        'a,
        R1: Into<&'a Signal<'a>>,
        R2: Into<&'a Signal<'a>>,
        R3: Into<&'a Signal<'a>>,
        R4: Into<&'a Signal<'a>>,
        R5: Into<&'a Signal<'a>>,
        R6: Into<&'a Signal<'a>>,
        R7: Into<&'a Signal<'a>>,
        R8: Into<&'a Signal<'a>>,
    > ElseIf<'a, (R1, R2, R3, R4, R5, R6, R7, R8)>
{
    pub fn else_<
        S1: Into<&'a Signal<'a>>,
        S2: Into<&'a Signal<'a>>,
        S3: Into<&'a Signal<'a>>,
        S4: Into<&'a Signal<'a>>,
        S5: Into<&'a Signal<'a>>,
        S6: Into<&'a Signal<'a>>,
        S7: Into<&'a Signal<'a>>,
        S8: Into<&'a Signal<'a>>,
    >(
        self,
        when_false: (S1, S2, S3, S4, S5, S6, S7, S8),
    ) -> (
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
    ) {
        let ret = (
            self.cond.mux(self.when_true.0, when_false.0),
            self.cond.mux(self.when_true.1, when_false.1),
            self.cond.mux(self.when_true.2, when_false.2),
            self.cond.mux(self.when_true.3, when_false.3),
            self.cond.mux(self.when_true.4, when_false.4),
            self.cond.mux(self.when_true.5, when_false.5),
            self.cond.mux(self.when_true.6, when_false.6),
            self.cond.mux(self.when_true.7, when_false.7),
        );
        match self.parent {
            ElseIfParent::If(parent) => parent.else_(ret),
            ElseIfParent::ElseIf(parent) => parent.else_(ret),
        }
    }
}

impl<
        'a,
        R1: Into<&'a Signal<'a>>,
        R2: Into<&'a Signal<'a>>,
        R3: Into<&'a Signal<'a>>,
        R4: Into<&'a Signal<'a>>,
        R5: Into<&'a Signal<'a>>,
        R6: Into<&'a Signal<'a>>,
        R7: Into<&'a Signal<'a>>,
        R8: Into<&'a Signal<'a>>,
        R9: Into<&'a Signal<'a>>,
    > ElseIf<'a, (R1, R2, R3, R4, R5, R6, R7, R8, R9)>
{
    pub fn else_<
        S1: Into<&'a Signal<'a>>,
        S2: Into<&'a Signal<'a>>,
        S3: Into<&'a Signal<'a>>,
        S4: Into<&'a Signal<'a>>,
        S5: Into<&'a Signal<'a>>,
        S6: Into<&'a Signal<'a>>,
        S7: Into<&'a Signal<'a>>,
        S8: Into<&'a Signal<'a>>,
        S9: Into<&'a Signal<'a>>,
    >(
        self,
        when_false: (S1, S2, S3, S4, S5, S6, S7, S8, S9),
    ) -> (
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
    ) {
        let ret = (
            self.cond.mux(self.when_true.0, when_false.0),
            self.cond.mux(self.when_true.1, when_false.1),
            self.cond.mux(self.when_true.2, when_false.2),
            self.cond.mux(self.when_true.3, when_false.3),
            self.cond.mux(self.when_true.4, when_false.4),
            self.cond.mux(self.when_true.5, when_false.5),
            self.cond.mux(self.when_true.6, when_false.6),
            self.cond.mux(self.when_true.7, when_false.7),
            self.cond.mux(self.when_true.8, when_false.8),
        );
        match self.parent {
            ElseIfParent::If(parent) => parent.else_(ret),
            ElseIfParent::ElseIf(parent) => parent.else_(ret),
        }
    }
}

impl<
        'a,
        R1: Into<&'a Signal<'a>>,
        R2: Into<&'a Signal<'a>>,
        R3: Into<&'a Signal<'a>>,
        R4: Into<&'a Signal<'a>>,
        R5: Into<&'a Signal<'a>>,
        R6: Into<&'a Signal<'a>>,
        R7: Into<&'a Signal<'a>>,
        R8: Into<&'a Signal<'a>>,
        R9: Into<&'a Signal<'a>>,
        R10: Into<&'a Signal<'a>>,
    > ElseIf<'a, (R1, R2, R3, R4, R5, R6, R7, R8, R9, R10)>
{
    pub fn else_<
        S1: Into<&'a Signal<'a>>,
        S2: Into<&'a Signal<'a>>,
        S3: Into<&'a Signal<'a>>,
        S4: Into<&'a Signal<'a>>,
        S5: Into<&'a Signal<'a>>,
        S6: Into<&'a Signal<'a>>,
        S7: Into<&'a Signal<'a>>,
        S8: Into<&'a Signal<'a>>,
        S9: Into<&'a Signal<'a>>,
        S10: Into<&'a Signal<'a>>,
    >(
        self,
        when_false: (S1, S2, S3, S4, S5, S6, S7, S8, S9, S10),
    ) -> (
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
    ) {
        let ret = (
            self.cond.mux(self.when_true.0, when_false.0),
            self.cond.mux(self.when_true.1, when_false.1),
            self.cond.mux(self.when_true.2, when_false.2),
            self.cond.mux(self.when_true.3, when_false.3),
            self.cond.mux(self.when_true.4, when_false.4),
            self.cond.mux(self.when_true.5, when_false.5),
            self.cond.mux(self.when_true.6, when_false.6),
            self.cond.mux(self.when_true.7, when_false.7),
            self.cond.mux(self.when_true.8, when_false.8),
            self.cond.mux(self.when_true.9, when_false.9),
        );
        match self.parent {
            ElseIfParent::If(parent) => parent.else_(ret),
            ElseIfParent::ElseIf(parent) => parent.else_(ret),
        }
    }
}

impl<
        'a,
        R1: Into<&'a Signal<'a>>,
        R2: Into<&'a Signal<'a>>,
        R3: Into<&'a Signal<'a>>,
        R4: Into<&'a Signal<'a>>,
        R5: Into<&'a Signal<'a>>,
        R6: Into<&'a Signal<'a>>,
        R7: Into<&'a Signal<'a>>,
        R8: Into<&'a Signal<'a>>,
        R9: Into<&'a Signal<'a>>,
        R10: Into<&'a Signal<'a>>,
        R11: Into<&'a Signal<'a>>,
    > ElseIf<'a, (R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11)>
{
    pub fn else_<
        S1: Into<&'a Signal<'a>>,
        S2: Into<&'a Signal<'a>>,
        S3: Into<&'a Signal<'a>>,
        S4: Into<&'a Signal<'a>>,
        S5: Into<&'a Signal<'a>>,
        S6: Into<&'a Signal<'a>>,
        S7: Into<&'a Signal<'a>>,
        S8: Into<&'a Signal<'a>>,
        S9: Into<&'a Signal<'a>>,
        S10: Into<&'a Signal<'a>>,
        S11: Into<&'a Signal<'a>>,
    >(
        self,
        when_false: (S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11),
    ) -> (
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
    ) {
        let ret = (
            self.cond.mux(self.when_true.0, when_false.0),
            self.cond.mux(self.when_true.1, when_false.1),
            self.cond.mux(self.when_true.2, when_false.2),
            self.cond.mux(self.when_true.3, when_false.3),
            self.cond.mux(self.when_true.4, when_false.4),
            self.cond.mux(self.when_true.5, when_false.5),
            self.cond.mux(self.when_true.6, when_false.6),
            self.cond.mux(self.when_true.7, when_false.7),
            self.cond.mux(self.when_true.8, when_false.8),
            self.cond.mux(self.when_true.9, when_false.9),
            self.cond.mux(self.when_true.10, when_false.10),
        );
        match self.parent {
            ElseIfParent::If(parent) => parent.else_(ret),
            ElseIfParent::ElseIf(parent) => parent.else_(ret),
        }
    }
}

impl<
        'a,
        R1: Into<&'a Signal<'a>>,
        R2: Into<&'a Signal<'a>>,
        R3: Into<&'a Signal<'a>>,
        R4: Into<&'a Signal<'a>>,
        R5: Into<&'a Signal<'a>>,
        R6: Into<&'a Signal<'a>>,
        R7: Into<&'a Signal<'a>>,
        R8: Into<&'a Signal<'a>>,
        R9: Into<&'a Signal<'a>>,
        R10: Into<&'a Signal<'a>>,
        R11: Into<&'a Signal<'a>>,
        R12: Into<&'a Signal<'a>>,
    > ElseIf<'a, (R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12)>
{
    pub fn else_<
        S1: Into<&'a Signal<'a>>,
        S2: Into<&'a Signal<'a>>,
        S3: Into<&'a Signal<'a>>,
        S4: Into<&'a Signal<'a>>,
        S5: Into<&'a Signal<'a>>,
        S6: Into<&'a Signal<'a>>,
        S7: Into<&'a Signal<'a>>,
        S8: Into<&'a Signal<'a>>,
        S9: Into<&'a Signal<'a>>,
        S10: Into<&'a Signal<'a>>,
        S11: Into<&'a Signal<'a>>,
        S12: Into<&'a Signal<'a>>,
    >(
        self,
        when_false: (S1, S2, S3, S4, S5, S6, S7, S8, S9, S10, S11, S12),
    ) -> (
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
    ) {
        let ret = (
            self.cond.mux(self.when_true.0, when_false.0),
            self.cond.mux(self.when_true.1, when_false.1),
            self.cond.mux(self.when_true.2, when_false.2),
            self.cond.mux(self.when_true.3, when_false.3),
            self.cond.mux(self.when_true.4, when_false.4),
            self.cond.mux(self.when_true.5, when_false.5),
            self.cond.mux(self.when_true.6, when_false.6),
            self.cond.mux(self.when_true.7, when_false.7),
            self.cond.mux(self.when_true.8, when_false.8),
            self.cond.mux(self.when_true.9, when_false.9),
            self.cond.mux(self.when_true.10, when_false.10),
            self.cond.mux(self.when_true.11, when_false.11),
        );
        match self.parent {
            ElseIfParent::If(parent) => parent.else_(ret),
            ElseIfParent::ElseIf(parent) => parent.else_(ret),
        }
    }
}
