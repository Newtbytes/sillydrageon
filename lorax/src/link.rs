use crate::pool::{Pool, Ptr};

pub trait Linked {
    fn next(&self) -> &mut Option<Ptr>;
    fn prev(&self) -> &mut Option<Ptr>;
}

impl<T: Linked> Pool<T> {
    fn get_next(&self, obj: &Ptr) -> &mut Option<Ptr> {
        self.get(obj).next()
    }

    fn get_prev(&self, obj: &Ptr) -> &mut Option<Ptr> {
        self.get(obj).prev()
    }

    fn insert_ahead(&mut self, head: &Ptr, ahead: &Ptr) {
        if let Some(next) = self.get_next(head) {
            *self.get(next).prev() = Some(*ahead);
            *self.get(ahead).next() = Some(*next);
        }

        *self.get(head).next() = Some(*ahead);
        *self.get(ahead).prev() = Some(*head);
    }
}
