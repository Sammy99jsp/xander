pub mod parts;

use std::{
    fmt::Debug,
    ops::Deref,
    sync::{RwLock, Weak},
};

pub use parts::Anonymous;
pub trait ProxyPart<Ctx, Value>: Debug + Sync + Send {
    fn compute(&mut self, ctx: &Ctx, prev: &mut Value, dispatch: Dispatch<'_>);
}

#[derive(Clone)]
enum ProxyInner<Ctx, Value> {
    Const(Value),

    /// Computed off something else.
    Derived(for<'a> fn(&'a Ctx) -> Value),
}

impl<Ctx, Value> ProxyInner<Ctx, Value> {
    fn debug(&self, f: &mut std::fmt::Formatter<'_>, ctx: &Ctx) -> std::fmt::Result
    where
        Value: Debug,
    {
        match self {
            ProxyInner::Const(val) => val.fmt(f),
            ProxyInner::Derived(get) => get(ctx).fmt(f),
        }
    }

    fn get(&self, ctx: &Ctx) -> Value
    where
        Value: Clone,
    {
        match self {
            ProxyInner::Const(c) => c.clone(),
            ProxyInner::Derived(d) => d(ctx),
        }
    }
}

pub enum PostAction {
    Destroy(usize),
}

pub struct Dispatch<'a>(&'a mut Vec<PostAction>, usize);

impl<'a> Dispatch<'a> {
    ///
    /// Destroy this [ProxyPart].
    ///
    pub fn destroy(&mut self) {
        self.0.push(PostAction::Destroy(self.1));
    }
}

pub struct Proxy<Ctx, Value> {
    ctx: Weak<Ctx>,
    value: ProxyInner<Ctx, Value>,
    overrides: RwLock<Vec<Box<dyn ProxyPart<Ctx, Value>>>>,
}

impl<Ctx, Value: Debug> std::fmt::Debug for Proxy<Ctx, Value> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Proxy")
            .field_with(|f| {
                let ctx = self.ctx.upgrade().unwrap();
                self.value.debug(f, ctx.as_ref())
            })
            .finish()?;

        let overrides = self.overrides.read().expect("Not poisoned.");
        if !overrides.is_empty() {
            if !f.alternate() {
                write!(f, " → ")?;
            }

            for (idx, part) in overrides.iter().enumerate() {
                if f.alternate() {
                    write!(f, "\n ⤷ ")?;
                }

                part.fmt(f)?;

                if !f.alternate() && idx != (overrides.len() - 1) {
                    write!(f, " → ")?;
                }
            }
        }

        Ok(())
    }
}

impl<Ctx, Value> Proxy<Ctx, Value> {
    pub const fn new(initial: Value, ctx: Weak<Ctx>) -> Self {
        Self {
            ctx,
            value: ProxyInner::Const(initial),
            overrides: RwLock::new(Vec::new()),
        }
    }

    pub fn derived(derivation: for<'a> fn(&'a Ctx) -> Value, ctx: Weak<Ctx>) -> Self {
        Self {
            ctx,
            value: ProxyInner::Derived(derivation),
            overrides: RwLock::new(vec![]),
        }
    }

    pub fn insert<P>(&self, part: P)
    where
        P: ProxyPart<Ctx, Value> + 'static,
    {
        self.overrides
            .write()
            .expect("Not poisoned.")
            .push(Box::new(part))
    }

    pub fn get(&self) -> Value
    where
        Value: Clone,
    {
        // Because of ownership rules, it's okay! Hopefully...
        let ctx = self.ctx.upgrade().unwrap();
        let ctx = ctx.deref();

        let mut value = self.value.get(ctx);

        let mut dispatcher = vec![];

        for (idx, part) in self
            .overrides
            .write()
            .expect("Not poisoned.")
            .iter_mut()
            .enumerate()
        {
            part.compute(ctx, &mut value, Dispatch(&mut dispatcher, idx));
        }

        value
    }
}

pub trait ToProxy<Ctx>: Sized {
    fn proxy(self, ctx: Weak<Ctx>) -> Proxy<Ctx, Self> {
        Proxy::new(self, ctx)
    }
}

impl<Ctx, Val> ToProxy<Ctx> for Val {
    fn proxy(self, ctx: Weak<Ctx>) -> Proxy<Ctx, Self> {
        Proxy::new(self, ctx)
    }
}
