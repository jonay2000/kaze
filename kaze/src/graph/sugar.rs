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

impl<'a> If<'a, &'a Signal<'a>> {
    pub fn else_<S: Into<&'a Signal<'a>>>(self, when_false: S) -> &'a Signal<'a> {
        let when_false = when_false.into();
        self.cond.mux(self.when_true, when_false)
    }
}

// TODO: Come up with a nice way to generate these definitions with macros
impl<'a> If<'a, (&'a Signal<'a>,)> {
    pub fn else_<S: Into<&'a Signal<'a>>>(self, when_false: (S,)) -> (&'a Signal<'a>,) {
        (self.cond.mux(self.when_true.0, when_false.0.into()),)
    }
}

impl<'a> If<'a, (&'a Signal<'a>, &'a Signal<'a>)> {
    pub fn else_<
        S1: Into<&'a Signal<'a>>,
        S2: Into<&'a Signal<'a>>,
    >(self, when_false: (S1, S2)) -> (&'a Signal<'a>, &'a Signal<'a>) {
        (
            self.cond.mux(self.when_true.0, when_false.0.into()),
            self.cond.mux(self.when_true.1, when_false.1.into()),
        )
    }
}

impl<'a> If<'a, (&'a Signal<'a>, &'a Signal<'a>, &'a Signal<'a>)> {
    pub fn else_<
        S1: Into<&'a Signal<'a>>,
        S2: Into<&'a Signal<'a>>,
        S3: Into<&'a Signal<'a>>,
    >(
        self,
        when_false: (S1, S2, S3),
    ) -> (&'a Signal<'a>, &'a Signal<'a>, &'a Signal<'a>) {
        (
            self.cond.mux(self.when_true.0, when_false.0.into()),
            self.cond.mux(self.when_true.1, when_false.1.into()),
            self.cond.mux(self.when_true.2, when_false.2.into()),
        )
    }
}

impl<'a>
    If<
        'a,
        (
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
        ),
    >
{
    pub fn else_<
        S1: Into<&'a Signal<'a>>,
        S2: Into<&'a Signal<'a>>,
        S3: Into<&'a Signal<'a>>,
        S4: Into<&'a Signal<'a>>,
    >(
        self,
        when_false: (
            S1,
            S2,
            S3,
            S4,
        ),
    ) -> (&'a Signal<'a>, &'a Signal<'a>, &'a Signal<'a>, &'a Signal<'a>) {
        (
            self.cond.mux(self.when_true.0, when_false.0.into()),
            self.cond.mux(self.when_true.1, when_false.1.into()),
            self.cond.mux(self.when_true.2, when_false.2.into()),
            self.cond.mux(self.when_true.3, when_false.3.into()),
        )
    }
}

impl<'a>
    If<
        'a,
        (
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
        ),
    >
{
    pub fn else_<
        S1: Into<&'a Signal<'a>>,
        S2: Into<&'a Signal<'a>>,
        S3: Into<&'a Signal<'a>>,
        S4: Into<&'a Signal<'a>>,
        S5: Into<&'a Signal<'a>>,
    >(
        self,
        when_false: (
            S1,
            S2,
            S3,
            S4,
            S5,
        ),
    ) -> (
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
    ) {
        (
            self.cond.mux(self.when_true.0, when_false.0.into()),
            self.cond.mux(self.when_true.1, when_false.1.into()),
            self.cond.mux(self.when_true.2, when_false.2.into()),
            self.cond.mux(self.when_true.3, when_false.3.into()),
            self.cond.mux(self.when_true.4, when_false.4.into()),
        )
    }
}

impl<'a>
    If<
        'a,
        (
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
        ),
    >
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
        when_false: (
            S1,
            S2,
            S3,
            S4,
            S5,
            S6,
        ),
    ) -> (
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
    ) {
        (
            self.cond.mux(self.when_true.0, when_false.0.into()),
            self.cond.mux(self.when_true.1, when_false.1.into()),
            self.cond.mux(self.when_true.2, when_false.2.into()),
            self.cond.mux(self.when_true.3, when_false.3.into()),
            self.cond.mux(self.when_true.4, when_false.4.into()),
            self.cond.mux(self.when_true.5, when_false.5.into()),
        )
    }
}

impl<'a>
    If<
        'a,
        (
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
        ),
    >
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
        when_false: (
            S1,
            S2,
            S3,
            S4,
            S5,
            S6,
            S7,
        ),
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
            self.cond.mux(self.when_true.0, when_false.0.into()),
            self.cond.mux(self.when_true.1, when_false.1.into()),
            self.cond.mux(self.when_true.2, when_false.2.into()),
            self.cond.mux(self.when_true.3, when_false.3.into()),
            self.cond.mux(self.when_true.4, when_false.4.into()),
            self.cond.mux(self.when_true.5, when_false.5.into()),
            self.cond.mux(self.when_true.6, when_false.6.into()),
        )
    }
}

impl<'a>
    If<
        'a,
        (
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
        ),
    >
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
        when_false: (
            S1,
            S2,
            S3,
            S4,
            S5,
            S6,
            S7,
            S8,
        ),
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
            self.cond.mux(self.when_true.0, when_false.0.into()),
            self.cond.mux(self.when_true.1, when_false.1.into()),
            self.cond.mux(self.when_true.2, when_false.2.into()),
            self.cond.mux(self.when_true.3, when_false.3.into()),
            self.cond.mux(self.when_true.4, when_false.4.into()),
            self.cond.mux(self.when_true.5, when_false.5.into()),
            self.cond.mux(self.when_true.6, when_false.6.into()),
            self.cond.mux(self.when_true.7, when_false.7.into()),
        )
    }
}

impl<'a>
    If<
        'a,
        (
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
        ),
    >
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
        when_false: (
            S1,
            S2,
            S3,
            S4,
            S5,
            S6,
            S7,
            S8,
            S9,
        ),
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
            self.cond.mux(self.when_true.0, when_false.0.into()),
            self.cond.mux(self.when_true.1, when_false.1.into()),
            self.cond.mux(self.when_true.2, when_false.2.into()),
            self.cond.mux(self.when_true.3, when_false.3.into()),
            self.cond.mux(self.when_true.4, when_false.4.into()),
            self.cond.mux(self.when_true.5, when_false.5.into()),
            self.cond.mux(self.when_true.6, when_false.6.into()),
            self.cond.mux(self.when_true.7, when_false.7.into()),
            self.cond.mux(self.when_true.8, when_false.8.into()),
        )
    }
}

impl<'a>
    If<
        'a,
        (
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
        ),
    >
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
        when_false: (
            S1,
            S2,
            S3,
            S4,
            S5,
            S6,
            S7,
            S8,
            S9,
            S10,
        ),
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
            self.cond.mux(self.when_true.0, when_false.0.into()),
            self.cond.mux(self.when_true.1, when_false.1.into()),
            self.cond.mux(self.when_true.2, when_false.2.into()),
            self.cond.mux(self.when_true.3, when_false.3.into()),
            self.cond.mux(self.when_true.4, when_false.4.into()),
            self.cond.mux(self.when_true.5, when_false.5.into()),
            self.cond.mux(self.when_true.6, when_false.6.into()),
            self.cond.mux(self.when_true.7, when_false.7.into()),
            self.cond.mux(self.when_true.8, when_false.8.into()),
            self.cond.mux(self.when_true.9, when_false.9.into()),
        )
    }
}

impl<'a>
    If<
        'a,
        (
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
        ),
    >
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
        when_false: (
            S1,
            S2,
            S3,
            S4,
            S5,
            S6,
            S7,
            S8,
            S9,
            S10,
            S11,
        ),
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
            self.cond.mux(self.when_true.0, when_false.0.into()),
            self.cond.mux(self.when_true.1, when_false.1.into()),
            self.cond.mux(self.when_true.2, when_false.2.into()),
            self.cond.mux(self.when_true.3, when_false.3.into()),
            self.cond.mux(self.when_true.4, when_false.4.into()),
            self.cond.mux(self.when_true.5, when_false.5.into()),
            self.cond.mux(self.when_true.6, when_false.6.into()),
            self.cond.mux(self.when_true.7, when_false.7.into()),
            self.cond.mux(self.when_true.8, when_false.8.into()),
            self.cond.mux(self.when_true.9, when_false.9.into()),
            self.cond.mux(self.when_true.10, when_false.10.into()),
        )
    }
}

impl<'a>
    If<
        'a,
        (
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
        ),
    >
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
        when_false: (
            S1,
            S2,
            S3,
            S4,
            S5,
            S6,
            S7,
            S8,
            S9,
            S10,
            S11,
            S12,
        ),
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
            self.cond.mux(self.when_true.0, when_false.0.into()),
            self.cond.mux(self.when_true.1, when_false.1.into()),
            self.cond.mux(self.when_true.2, when_false.2.into()),
            self.cond.mux(self.when_true.3, when_false.3.into()),
            self.cond.mux(self.when_true.4, when_false.4.into()),
            self.cond.mux(self.when_true.5, when_false.5.into()),
            self.cond.mux(self.when_true.6, when_false.6.into()),
            self.cond.mux(self.when_true.7, when_false.7.into()),
            self.cond.mux(self.when_true.8, when_false.8.into()),
            self.cond.mux(self.when_true.9, when_false.9.into()),
            self.cond.mux(self.when_true.10, when_false.10.into()),
            self.cond.mux(self.when_true.11, when_false.11.into()),
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

impl<'a> ElseIf<'a, &'a Signal<'a>> {
    pub fn else_<S: Into<&'a Signal<'a>>>(self, when_false: S) -> &'a Signal<'a> {
        let when_false = when_false.into();
        let ret = self.cond.mux(self.when_true, when_false);
        match self.parent {
            ElseIfParent::If(parent) => parent.else_(ret),
            ElseIfParent::ElseIf(parent) => parent.else_(ret),
        }
    }
}

// TODO: Come up with a nice way to generate these definitions with macros
impl<'a> ElseIf<'a, (&'a Signal<'a>,)> {
    pub fn else_<S: Into<&'a Signal<'a>>>(self, when_false: (S,)) -> (&'a Signal<'a>,) {
        let ret = (self.cond.mux(self.when_true.0, when_false.0.into()),);
        match self.parent {
            ElseIfParent::If(parent) => parent.else_(ret),
            ElseIfParent::ElseIf(parent) => parent.else_(ret),
        }
    }
}

impl<'a> ElseIf<'a, (&'a Signal<'a>, &'a Signal<'a>)> {
    pub fn else_<S1: Into<&'a Signal<'a>>, S2: Into<&'a Signal<'a>>>(self, when_false: (S1, S2)) -> (&'a Signal<'a>, &'a Signal<'a>) {
        let ret = (
            self.cond.mux(self.when_true.0, when_false.0.into()),
            self.cond.mux(self.when_true.1, when_false.1.into()),
        );
        match self.parent {
            ElseIfParent::If(parent) => parent.else_(ret),
            ElseIfParent::ElseIf(parent) => parent.else_(ret),
        }
    }
}

impl<'a> ElseIf<'a, (&'a Signal<'a>, &'a Signal<'a>, &'a Signal<'a>)> {
    pub fn else_<
        S1: Into<&'a Signal<'a>>,
        S2: Into<&'a Signal<'a>>,
        S3: Into<&'a Signal<'a>>,
    >(
        self,
        when_false: (S1, S2, S3),
    ) -> (&'a Signal<'a>, &'a Signal<'a>, &'a Signal<'a>) {
        let ret = (
            self.cond.mux(self.when_true.0, when_false.0.into()),
            self.cond.mux(self.when_true.1, when_false.1.into()),
            self.cond.mux(self.when_true.2, when_false.2.into()),
        );
        match self.parent {
            ElseIfParent::If(parent) => parent.else_(ret),
            ElseIfParent::ElseIf(parent) => parent.else_(ret),
        }
    }
}

impl<'a>
    ElseIf<
        'a,
        (
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
        ),
    >
{
    pub fn else_<
        S1: Into<&'a Signal<'a>>,
        S2: Into<&'a Signal<'a>>,
        S3: Into<&'a Signal<'a>>,
        S4: Into<&'a Signal<'a>>,
    >(
        self,
        when_false: (
            S1,
            S2,
            S3,
            S4,
        ),
    ) -> (&'a Signal<'a>, &'a Signal<'a>, &'a Signal<'a>, &'a Signal<'a>) {
        let ret = (
            self.cond.mux(self.when_true.0, when_false.0.into()),
            self.cond.mux(self.when_true.1, when_false.1.into()),
            self.cond.mux(self.when_true.2, when_false.2.into()),
            self.cond.mux(self.when_true.3, when_false.3.into()),
        );
        match self.parent {
            ElseIfParent::If(parent) => parent.else_(ret),
            ElseIfParent::ElseIf(parent) => parent.else_(ret),
        }
    }
}

impl<'a>
    ElseIf<
        'a,
        (
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
        ),
    >
{
    pub fn else_<
        S1: Into<&'a Signal<'a>>,
        S2: Into<&'a Signal<'a>>,
        S3: Into<&'a Signal<'a>>,
        S4: Into<&'a Signal<'a>>,
        S5: Into<&'a Signal<'a>>,
    >(
        self,
        when_false: (
            S1,
            S2,
            S3,
            S4,
            S5,
        ),
    ) -> (
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
    ) {
        let ret = (
            self.cond.mux(self.when_true.0, when_false.0.into()),
            self.cond.mux(self.when_true.1, when_false.1.into()),
            self.cond.mux(self.when_true.2, when_false.2.into()),
            self.cond.mux(self.when_true.3, when_false.3.into()),
            self.cond.mux(self.when_true.4, when_false.4.into()),
        );
        match self.parent {
            ElseIfParent::If(parent) => parent.else_(ret),
            ElseIfParent::ElseIf(parent) => parent.else_(ret),
        }
    }
}

impl<'a>
    ElseIf<
        'a,
        (
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
        ),
    >
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
        when_false: (
            S1,
            S2,
            S3,
            S4,
            S5,
            S6,
        ),
    ) -> (
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
        &'a Signal<'a>,
    ) {
        let ret = (
            self.cond.mux(self.when_true.0, when_false.0.into()),
            self.cond.mux(self.when_true.1, when_false.1.into()),
            self.cond.mux(self.when_true.2, when_false.2.into()),
            self.cond.mux(self.when_true.3, when_false.3.into()),
            self.cond.mux(self.when_true.4, when_false.4.into()),
            self.cond.mux(self.when_true.5, when_false.5.into()),
        );
        match self.parent {
            ElseIfParent::If(parent) => parent.else_(ret),
            ElseIfParent::ElseIf(parent) => parent.else_(ret),
        }
    }
}

impl<'a>
    ElseIf<
        'a,
        (
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
        ),
    >
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
        when_false: (
            S1,
            S2,
            S3,
            S4,
            S5,
            S6,
            S7,
        ),
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
            self.cond.mux(self.when_true.0, when_false.0.into()),
            self.cond.mux(self.when_true.1, when_false.1.into()),
            self.cond.mux(self.when_true.2, when_false.2.into()),
            self.cond.mux(self.when_true.3, when_false.3.into()),
            self.cond.mux(self.when_true.4, when_false.4.into()),
            self.cond.mux(self.when_true.5, when_false.5.into()),
            self.cond.mux(self.when_true.6, when_false.6.into()),
        );
        match self.parent {
            ElseIfParent::If(parent) => parent.else_(ret),
            ElseIfParent::ElseIf(parent) => parent.else_(ret),
        }
    }
}

impl<'a>
    ElseIf<
        'a,
        (
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
        ),
    >
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
        when_false: (
            S1,
            S2,
            S3,
            S4,
            S5,
            S6,
            S7,
            S8,
        ),
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
            self.cond.mux(self.when_true.0, when_false.0.into()),
            self.cond.mux(self.when_true.1, when_false.1.into()),
            self.cond.mux(self.when_true.2, when_false.2.into()),
            self.cond.mux(self.when_true.3, when_false.3.into()),
            self.cond.mux(self.when_true.4, when_false.4.into()),
            self.cond.mux(self.when_true.5, when_false.5.into()),
            self.cond.mux(self.when_true.6, when_false.6.into()),
            self.cond.mux(self.when_true.7, when_false.7.into()),
        );
        match self.parent {
            ElseIfParent::If(parent) => parent.else_(ret),
            ElseIfParent::ElseIf(parent) => parent.else_(ret),
        }
    }
}

impl<'a>
    ElseIf<
        'a,
        (
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
            &'a Signal<'a>,
        ),
    >
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
        when_false: (
            S1,
            S2,
            S3,
            S4,
            S5,
            S6,
            S7,
            S8,
            S9,
        ),
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
            self.cond.mux(self.when_true.0, when_false.0.into()),
            self.cond.mux(self.when_true.1, when_false.1.into()),
            self.cond.mux(self.when_true.2, when_false.2.into()),
            self.cond.mux(self.when_true.3, when_false.3.into()),
            self.cond.mux(self.when_true.4, when_false.4.into()),
            self.cond.mux(self.when_true.5, when_false.5.into()),
            self.cond.mux(self.when_true.6, when_false.6.into()),
            self.cond.mux(self.when_true.7, when_false.7.into()),
            self.cond.mux(self.when_true.8, when_false.8.into()),
        );
        match self.parent {
            ElseIfParent::If(parent) => parent.else_(ret),
            ElseIfParent::ElseIf(parent) => parent.else_(ret),
        }
    }
}

impl<'a>
    ElseIf<
        'a,
        (
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
        ),
    >
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
        when_false: (
            S1,
            S2,
            S3,
            S4,
            S5,
            S6,
            S7,
            S8,
            S9,
            S10,
        ),
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
            self.cond.mux(self.when_true.0, when_false.0.into()),
            self.cond.mux(self.when_true.1, when_false.1.into()),
            self.cond.mux(self.when_true.2, when_false.2.into()),
            self.cond.mux(self.when_true.3, when_false.3.into()),
            self.cond.mux(self.when_true.4, when_false.4.into()),
            self.cond.mux(self.when_true.5, when_false.5.into()),
            self.cond.mux(self.when_true.6, when_false.6.into()),
            self.cond.mux(self.when_true.7, when_false.7.into()),
            self.cond.mux(self.when_true.8, when_false.8.into()),
            self.cond.mux(self.when_true.9, when_false.9.into()),
        );
        match self.parent {
            ElseIfParent::If(parent) => parent.else_(ret),
            ElseIfParent::ElseIf(parent) => parent.else_(ret),
        }
    }
}

impl<'a>
    ElseIf<
        'a,
        (
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
        ),
    >
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
        when_false: (
            S1,
            S2,
            S3,
            S4,
            S5,
            S6,
            S7,
            S8,
            S9,
            S10,
            S11,
        ),
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
            self.cond.mux(self.when_true.0, when_false.0.into()),
            self.cond.mux(self.when_true.1, when_false.1.into()),
            self.cond.mux(self.when_true.2, when_false.2.into()),
            self.cond.mux(self.when_true.3, when_false.3.into()),
            self.cond.mux(self.when_true.4, when_false.4.into()),
            self.cond.mux(self.when_true.5, when_false.5.into()),
            self.cond.mux(self.when_true.6, when_false.6.into()),
            self.cond.mux(self.when_true.7, when_false.7.into()),
            self.cond.mux(self.when_true.8, when_false.8.into()),
            self.cond.mux(self.when_true.9, when_false.9.into()),
            self.cond.mux(self.when_true.10, when_false.10.into()),
        );
        match self.parent {
            ElseIfParent::If(parent) => parent.else_(ret),
            ElseIfParent::ElseIf(parent) => parent.else_(ret),
        }
    }
}

impl<'a>
    ElseIf<
        'a,
        (
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
        ),
    >
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
        when_false: (
            S1,
            S2,
            S3,
            S4,
            S5,
            S6,
            S7,
            S8,
            S9,
            S10,
            S11,
            S12,
        ),
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
            self.cond.mux(self.when_true.0, when_false.0.into()),
            self.cond.mux(self.when_true.1, when_false.1.into()),
            self.cond.mux(self.when_true.2, when_false.2.into()),
            self.cond.mux(self.when_true.3, when_false.3.into()),
            self.cond.mux(self.when_true.4, when_false.4.into()),
            self.cond.mux(self.when_true.5, when_false.5.into()),
            self.cond.mux(self.when_true.6, when_false.6.into()),
            self.cond.mux(self.when_true.7, when_false.7.into()),
            self.cond.mux(self.when_true.8, when_false.8.into()),
            self.cond.mux(self.when_true.9, when_false.9.into()),
            self.cond.mux(self.when_true.10, when_false.10.into()),
            self.cond.mux(self.when_true.11, when_false.11.into()),
        );
        match self.parent {
            ElseIfParent::If(parent) => parent.else_(ret),
            ElseIfParent::ElseIf(parent) => parent.else_(ret),
        }
    }
}
