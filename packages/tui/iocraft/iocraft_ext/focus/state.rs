use std::{
    collections::VecDeque,
    hash::{Hash, Hasher},
    sync::{Arc, Mutex},
};

use super::{FocusId, FocusScope};
use crate::iocraft_ext::focus::key::FocusKey;

#[derive(Clone, Debug, Default)]
pub struct FocusState {
    version: usize,
    inner: Arc<Mutex<Inner>>,
}

impl PartialEq for FocusState {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

impl Eq for FocusState {}

impl Hash for FocusState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.inner).hash(state);
    }
}

impl FocusState {
    pub fn new(scope: FocusScope) -> Self {
        Self {
            version: 0,
            inner: Arc::new(Mutex::new(Inner::new(scope))),
        }
    }

    pub fn register(&mut self, scope: FocusScope) -> FocusId {
        let mut guard = self.inner.lock().expect("mutex lock failed: register");
        self.version += 1;
        guard.register(scope)
    }

    pub fn scope(&self) -> FocusScope {
        let guard = self.inner.lock().expect("mutex lock failed: scope");
        guard.scope
    }

    pub fn set_scope(&mut self, scope: FocusScope) {
        let mut guard = self.inner.lock().expect("mutex lock failed: set_scope");
        self.version += 1;
        guard.set_scope(scope)
    }

    pub fn is_focused(&self, id: FocusId) -> bool {
        let guard = self.inner.lock().expect("mutex lock failed: is_focused");
        guard.focused == Some(id)
    }

    pub fn focus_next(&mut self) {
        let mut guard = self.inner.lock().expect("mutex lock failed: focus_next");
        self.version += 1;
        guard.rotate(1)
    }

    pub fn focus_previous(&mut self) {
        let mut guard = self
            .inner
            .lock()
            .expect("mutex lock failed: focus_previous");
        self.version += 1;
        guard.rotate(-1)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
struct Inner {
    focused: Option<FocusId>,
    scope: FocusScope,
    order: VecDeque<FocusId>,
}

impl Inner {
    fn new(scope: FocusScope) -> Self {
        Self {
            focused: None,
            scope,
            order: VecDeque::new(),
        }
    }

    fn register(&mut self, scope: FocusScope) -> FocusId {
        let key = FocusKey(self.order.len());
        let id = FocusId { key, scope };
        self.order.push_back(id);

        if self.focused.is_none() {
            self.focused = Some(id);
        }

        id
    }

    fn set_scope(&mut self, scope: FocusScope) {
        self.scope = scope;
        self.focused = self.order.iter().copied().find(|e| e.scope == scope);
    }

    fn rotate(&mut self, dir: isize) {
        let items: Vec<_> = self
            .order
            .iter()
            .copied()
            .filter(|e| e.scope == self.scope)
            .collect();

        if items.is_empty() {
            return;
        }

        let idx = items
            .iter()
            .position(|e| Some(e) == self.focused.as_ref())
            .unwrap_or(0);

        let next = ((idx as isize + dir).rem_euclid(items.len() as isize)) as usize;
        self.focused = Some(items[next]);
    }
}
