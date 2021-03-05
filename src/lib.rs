use std::collections::VecDeque;

#[derive(Eq, PartialEq, Debug)]
pub enum Element<T> {
    Added,
    NotAdded(T),
}

pub trait TryPush<T> {
    /// Attempts to push an element to the collection. If that action would
    /// re-allocate or require shifting elements then the `elem` is returned
    /// in `Element::NotAdded()`. Otherwise, the return value is `Element::Added`.
    fn try_push(&mut self, elem: T) -> Element<T>;

    /// Attempts to push an element front to the collection. If that action would
    /// re-allocate or require shifting elements then the `elem` is returned
    /// in `Element::NotAdded()`. Otherwise, the return value is `Element::Added`.
    fn try_push_front(&mut self, elem: T) -> Element<T>;
}

impl<T> TryPush<T> for Vec<T> {
    fn try_push(&mut self, elem: T) -> Element<T> {
        if self.capacity() == self.len() {
            Element::NotAdded(elem)
        } else {
            self.push(elem);
            Element::Added
        }
    }

    fn try_push_front(&mut self, elem: T) -> Element<T> {
        if self.capacity() == self.len() || self.len() > 0 {
            Element::NotAdded(elem)
        } else {
            self.push(elem);
            Element::Added
        }
    }
}

impl<T> TryPush<T> for VecDeque<T> {
    fn try_push(&mut self, elem: T) -> Element<T> {
        if dbg!{dbg!{self.capacity()} == dbg!{self.len()}} {
            Element::NotAdded(elem)
        } else {
            self.push_back(elem);
            Element::Added
        }
    }

    fn try_push_front(&mut self, elem: T) -> Element<T> {
        if self.capacity() == self.len() {
            Element::NotAdded(elem)
        } else {
            self.push_front(elem);
            Element::Added
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_not_add_to_empty_vec() {
        assert_eq!(Vec::with_capacity(0).try_push(1), Element::NotAdded(1));
    }

    #[test]
    fn should_add_to_capable_vec() {
        assert_eq!(Vec::with_capacity(10).try_push(1), Element::Added);
    }

    #[test]
    fn should_not_add_front_to_empty_vec() {
        assert_eq!(Vec::with_capacity(0).try_push_front(1), Element::NotAdded(1));
    }

    #[test]
    fn should_not_add_front_to_non_empty_vec() {
        assert_eq!(vec![0, 1].try_push_front(1), Element::NotAdded(1));
    }

    #[test]
    fn should_add_front_to_capable_vec() {
        assert_eq!(Vec::with_capacity(10).try_push_front(1), Element::Added);
    }

    #[test]
    fn should_not_add_to_empty_vec_deque() {
        let mut vd = VecDeque::with_capacity(0);
        vd.push_back(1);
        assert_eq!(vd.try_push(1), Element::NotAdded(1));
    }

    #[test]
    fn should_add_to_capable_vec_deque() {
        assert_eq!(VecDeque::with_capacity(10).try_push(1), Element::Added);
    }

    #[test]
    fn should_not_add_front_to_empty_vec_deque() {
        let mut vd = VecDeque::with_capacity(0);
        vd.push_back(1);
        assert_eq!(vd.try_push_front(1), Element::NotAdded(1));
    }

    #[test]
    fn should_add_front_to_capable_vec_deque() {
        assert_eq!(VecDeque::with_capacity(10).try_push_front(1), Element::Added);
    }
}
