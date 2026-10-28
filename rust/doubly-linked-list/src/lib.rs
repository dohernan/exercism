use std::rc::Rc;
use std::cell::RefCell;

// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;

pub struct Node<T>
{
    data: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    previous: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T>
{
    pub fn new(data:T) -> Self{
        Self{data, next:None, previous:None}
    }
        
}
pub struct LinkedList<T>
{
    list: Option<Rc<RefCell<Node<T>>>>,
    cursor : Cursor<T>,
    front : Cursor<T>,
    back: Cursor<T>,
    //iter: Iter< T>
}

pub struct Cursor< T>{
    element: Option<Rc<RefCell<Node<T>>>>,
}

//pub struct Iter< T>(std::marker::PhantomData<&T>);

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self{list:None, 
            cursor: Cursor{element:None}, 
            front: Cursor{element:None}, 
            back: Cursor{element:None}}//, iter: Iter(std::marker::PhantomData)} 
    }

    pub fn push_back(&mut self, data : T)
    {
        let mut new_node = Node::new(data);
        let back_node_rc = self.back.element.as_ref().unwrap();
        new_node.previous = Some(Rc::clone(back_node_rc));
        let next = Rc::new(RefCell::new(new_node));
        back_node_rc.borrow_mut().next = Some(Rc::clone(&next));
        self.back.element = Some(Rc::clone(&next));
    }

    pub fn push_front(&mut self, data : T)
    {
        let mut new_node = Node::new(data);
        let back_node_rc = self.back.element.as_ref().unwrap();
        new_node.next = Some(Rc::clone(back_node_rc));
        let next = Rc::new(RefCell::new(new_node));
        back_node_rc.borrow_mut().previous = Some(Rc::clone(&next));
        self.front.element = Some(Rc::clone(&next));
    }

    pub fn pop_back(&mut self) -> Option<T>
    {
        let back_node = self.back.element.take()?;
        let previous_back_node_rc = back_node.borrow_mut().previous.take();
        previous_back_node_rc.as_ref().unwrap().borrow_mut().next = None;
        self.back.element = previous_back_node_rc;
        Some(Rc::try_unwrap(back_node).ok().unwrap().into_inner().data)
    }

    pub fn pop_front(&mut self) -> Option<T>
    {
        let front_node = self.front.element.take()?;
        let next_front_node_rc = front_node.borrow_mut().next.take();
        next_front_node_rc.as_ref().unwrap().borrow_mut().previous = None;
        self.front.element = next_front_node_rc;
        Some(Rc::try_unwrap(front_node).ok().unwrap().into_inner().data)
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for LinkedList)
    pub fn is_empty(&self) -> bool {
        self.list.is_none()
    }

    pub fn len(&self) -> usize {
        if self.front.element.is_none()
        {
            return 0;
        }
        let mut cursor = Rc::clone(&self.front.element.as_ref().unwrap());
        let mut len: u32 = 1;
        while cursor.borrow().next.is_some()
        {
            let auto =Rc::clone(cursor.borrow().next.as_ref().unwrap());
            cursor = auto;
            len+=1;
        }
        len
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<T> {
         let element = Rc::clone(&self.front.element.as_ref().unwrap());
        Cursor{element:Some(element)}
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<T> {
        let element = Rc::clone(&self.back.element.as_ref().unwrap());
        Cursor{element:Some(element)}
    }

    // /// Return an iterator that moves from front to back
    // pub fn iter(&self) -> Iter<T> {
    //     todo!()
    // }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        todo!()
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    pub fn next(&mut self) -> Option<&mut T> {
        todo!()
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        todo!()
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        todo!()
    }

    pub fn insert_after(&mut self, _element: T) {
        todo!()
    }

    pub fn insert_before(&mut self, _element: T) {
        todo!()
    }
}

// impl< T> Iterator for Iter< T> {
//     type Item = &'a T;

//     fn next(&mut self) -> Option<&'a T> {
//         todo!()
//     }
// }
