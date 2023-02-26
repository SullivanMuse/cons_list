use std::rc::Rc;

struct ListBox<T> {
    x: T,
    xs: List<T>,
}

impl<T> ListBox<T> {
    fn init(&self) -> List<T>
    where
        T: Clone,
    {
        if let Some(xs) = &self.xs.0 {
            xs.init().cons(self.x.clone())
        } else {
            List::new()
        }
    }

    fn append(&self, other: &List<T>) -> List<T>
    where
        T: Clone,
    {
        if let Some(xs) = &self.xs.0 {
            xs.append(other).cons(self.x.clone())
        } else {
            other.cons(self.x.clone())
        }
    }

    fn last(&self) -> &T {
        self.xs.0.as_ref().map(|x| x.last()).unwrap_or(&self.x)
    }
}

struct List<T>(Option<Rc<ListBox<T>>>);

impl<T> Clone for List<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T> std::fmt::Debug for List<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(xs) = &self.0 {
            write!(f, "{:?} : {:?}", xs.x, xs.xs)
        } else {
            write!(f, "[]")
        }
    }
}

impl<T> List<T> {
    fn new() -> Self {
        Self(None)
    }

    fn cons(&self, x: T) -> Self {
        let xs = self.clone();
        Self(Some(Rc::new(ListBox { x, xs })))
    }

    fn uncons(&self) -> Option<(&T, &Self)> {
        self.0.as_ref().map(|list_box| (&list_box.x, &list_box.xs))
    }

    fn first(&self) -> Option<&T> {
        self.uncons().map(|(x, _)| x)
    }

    fn rest(&self) -> Option<&Self> {
        self.uncons().map(|(_, xs)| xs)
    }

    fn init(&self) -> Option<Self>
    where
        T: Clone,
    {
        self.0.as_ref().map(|xs| xs.init())
    }

    fn last(&self) -> Option<&T> {
        self.0.as_ref().map(|x| x.last())
    }

    fn append(&self, other: &Self) -> Self
    where
        T: Clone,
    {
        self.0.as_ref().map(|xs| xs.append(other)).unwrap_or(other.clone())
    }
}

fn main() {
    let xs = List::new().cons(1).cons(2).cons(3);
    dbg!(xs.first());
    dbg!(xs.rest());
    dbg!(xs.init());
    dbg!(xs.last());
    dbg!(xs.append(&xs));
}
