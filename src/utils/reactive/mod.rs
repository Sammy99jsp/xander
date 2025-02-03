use std::{
    collections::VecDeque,
    fmt::Debug,
    mem::{self, transmute},
    ops::Deref,
    rc::{Rc, Weak},
    sync::{Mutex, MutexGuard},
};

use serde::Deserialize;

/// Something that is dependent on some external condition to be kept "alive".
pub trait Ephemeral {
    fn is_alive(&self) -> bool;
}

impl<E: Ephemeral> Ephemeral for Rc<E> {
    fn is_alive(&self) -> bool {
        self.as_ref().is_alive()
    }
}

#[derive(Default, Debug)]
pub enum Lifespan<C: ?Sized = ()> {
    Of(Weak<C>),

    #[default]
    Indefinite,
}

impl<C: ?Sized> Clone for Lifespan<C> {
    fn clone(&self) -> Self {
        match self {
            Self::Of(arg0) => Self::Of(arg0.clone()),
            Self::Indefinite => Self::Indefinite,
        }
    }
}

impl Lifespan {
    pub fn of<T>(source: &Rc<T>) -> Self {
        const { assert!(mem::size_of::<Weak<T>>() == mem::size_of::<Weak<()>>()) }
        Lifespan::Of(unsafe { transmute::<Weak<T>, Weak<()>>(Rc::downgrade(source)) })
    }


}

impl<C: ?Sized> Lifespan<C> {
    pub fn of_this(source: &Rc<C>) -> Self {
        Lifespan::Of(Rc::downgrade(source))
    }

    pub fn as_weak(&self) -> Option<Weak<C>> {
        match self {
            Lifespan::Of(weak) => Some(weak.clone()),
            Lifespan::Indefinite => None,
        }
    }
}

impl<C: ?Sized> Ephemeral for Lifespan<C> {
    fn is_alive(&self) -> bool {
        match self {
            Lifespan::Of(weak) => weak.strong_count() > 0,
            Lifespan::Indefinite => true,
        }
    }
}

pub trait EphemeralContainer<E: Ephemeral> {
    type Return<'a>: IntoIterator<Item = &'a E>
    where
        E: 'a;

    fn cleanup(&mut self);
}

/// A slot holding an [Ephemeral] value.
///
/// The value is always checked via [Ephemeral::is_alive] before
/// any reads on this slot, if any is present.
pub struct RSlot<E: Ephemeral>(Option<E>);

impl<P> Debug for RSlot<P>
where
    P: Ephemeral + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_filled() {
            f.debug_tuple("Filled")
                .field(self.0.as_ref().unwrap())
                .finish()
        } else {
            write!(f, "Empty")
        }
    }
}

impl<P: Ephemeral> Default for RSlot<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P: Ephemeral> RSlot<P> {
    /// Creates a new (empty) slot.
    pub const fn new() -> Self {
        Self(None)
    }

    pub fn is_filled(&self) -> bool {
        match self.0 {
            Some(ref r) => r.is_alive(),
            None => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        !self.is_filled()
    }

    pub fn get(&self) -> Option<&P> {
        self.0.as_ref().and_then(|r| r.is_alive().then_some(r))
    }

    pub fn get_mut(&mut self) -> Option<&mut P> {
        self.0.as_mut().and_then(|r| r.is_alive().then_some(r))
    }

    /// Akin to [Option::replace]
    ///
    /// Replaces the current value in the slot,
    /// returning [Some] if there was anything already in the slot,
    /// or [None] otherwise.
    pub const fn replace(&mut self, new: P) -> Option<P> {
        self.0.replace(new)
    }

    /// Akin to [Option::take]
    ///
    /// Takes the value out of this slot (if there was any)
    pub const fn take(&mut self) -> Option<P> {
        self.0.take()
    }
}

impl<E: Ephemeral> EphemeralContainer<E> for RSlot<E> {
    type Return<'a>
        = Option<&'a E>
    where
        E: 'a;

    fn cleanup(&mut self) {
        match self.0 {
            Some(ref r) if !r.is_alive() => {
                self.0.take();
            }
            _ => (),
        }
    }
}

type Storage<T> = VecDeque<T>;

/// Reactive List.
///
/// This is equivalent to [Storage], but has the invariant
/// that all items will always be alive (see [Ephemeral::is_alive]).
#[derive(Debug, Deserialize)]
#[serde(from = "Vec<P>")]
pub struct RList<P: Ephemeral> {
    inner: Mutex<Storage<P>>,
}

impl<P: Ephemeral> From<Vec<P>> for RList<P> {
    fn from(value: Vec<P>) -> Self {
        Self {
            inner: Mutex::new(From::from(value)),
        }
    }
}

impl<P: Ephemeral> Default for RList<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P: Ephemeral> RList<P> {
    pub const fn new() -> Self {
        Self {
            inner: Mutex::new(Storage::new()),
        }
    }

    /// Purge any items that are no longer true,
    /// then return a [MutexGuard] of the backing storage.
    fn purge(&self) -> MutexGuard<'_, Storage<P>> {
        let mut list = self.inner.lock().expect("Non-Poisoned Mutex");

        for to_remove in list
            .iter()
            .enumerate()
            .filter_map(|(i, el)| el.is_alive().then_some(i))
            .enumerate()
            .map(|(removes_before, i)| i - removes_before)
            .collect::<Vec<_>>()
        {
            list.remove(to_remove);
        }

        list
    }

    pub fn push_back(&mut self, item: P) {
        let mut list = self.purge();
        list.push_back(item);
    }

    pub fn push_front(&mut self, item: P) {
        let mut list = self.purge();
        list.push_front(item);
    }

    /// Return an immutable reference to the list itself.
    pub fn list(&self) -> RListRef<'_, P> {
        RListRef(self.purge())
    }

    /// Return a mutable reference to the list itself.
    pub fn list_mut(&mut self) -> &mut Storage<P> {
        drop(self.purge());
        self.inner.get_mut().expect("not to be poisoned!")
    }
}

/// Wrapper type to ensure that the user
/// does not modify the underlying [Storage]
/// through this type.
pub struct RListRef<'a, P>(MutexGuard<'a, Storage<P>>);

impl<'a, P> Deref for RListRef<'a, P> {
    type Target = Storage<P>;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
