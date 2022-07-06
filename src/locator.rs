use std::cmp::Ordering;
use std::collections::HashMap;

pub trait PriorityQueue<T: PartialOrd> {
    fn enqueue(&mut self, ele: T) -> ();
    fn dequeue(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
}

/**
    An optional definition of a Node struct you may find useful
**/
struct Node<T> {
    priority: i32,
    data: T,
}

/**
    These traits are implemented for Nodes to make them comparable
**/
impl<T> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Node<T>) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        self.priority == other.priority
    }
}

/**
    You must implement the above trait for the vector type
**/
impl<T: PartialOrd> PriorityQueue<T> for Vec<T> {
    /**
        This functions pushes a given element onto the queue and
        reorders the queue such that the min heap property holds.
        See the project specifications for more details on how this
        works.
    **/
    fn enqueue(&mut self, ele: T) -> () {
        let mut idx = self.len();
        self.push(ele);
        let mut curr;
        let mut parent;

        match self.get(idx) {
            None => (),
            Some(x) => curr = x,
        }
        match self.get((idx - 1) / 2) {
            None => (),
            Some(x) => parent = x,
        }
        while parent.partial_cmp(&curr) == Some(Ordering::Greater) {
            self.insert((idx - 1) / 2, *curr);
            self.insert(idx, parent);
            idx = (idx - 1) / 2;
            match self.get(idx) {
                None => (),
                Some(x) => curr = x,
            }
            match self.get((idx - 1) / 2) {
                None => (),
                Some(x) => parent = x,
            }
        }
    }

    /**
        This function removes the root element from the queue and
        reorders the queue such that it maintains the min heap
        property.  See the project specifications for more details.
        You should return the deleted element in the form of an option.
        Return None if the queue was initially empty, Some(T) otherwise.
    **/
    fn dequeue(&mut self) -> Option<T> {
        let tmp;
        match self.pop() {
            None => (),
            Some(x) => tmp = x,
        }
        let res = Some(self.remove(0));
        self.insert(0, tmp);

        let mut idx = 0;
        let mut curr;
        let mut child1;
        let mut child2;
        match self.get(idx) {
            None => (),
            Some(x) => curr = x,
        }
        match self.get(2 * idx + 1) {
            None => (),
            Some(x) => child1 = x,
        }
        match self.get(2 * idx + 2) {
            None => (),
            Some(x) => child2 = x,
        }
        while curr.partial_cmp(child1) == Some(Ordering::Greater)
            && curr.partial_cmp(child2) == Some(Ordering::Greater)
        {
            if curr.partial_cmp(child1) == Some(Ordering::Greater)
                && curr.partial_cmp(child2) == Some(Ordering::Less)
            {
                /* swap curr and child1 */

                self.insert(2 * idx + 1, *curr);
                self.insert(idx, *child1);
                idx = 2 * idx + 1;

            } else if curr.partial_cmp(child1) == Some(Ordering::Less)
                && curr.partial_cmp(child2) == Some(Ordering::Greater)
            {
                /* swap curr and child2 */
                self.insert(2 * idx + 2, *curr);
                self.insert(idx, *child2);
                idx = 2 * idx + 2;
            } else {
                break; /* Not sure what to do here */
            }
        }
        return res;
    }

    /**
        This function returns the element that would be removed
        if dequeue were called on the queue.  There should be no
        mutations to the queue.  Return the element in the form
        of an option.  Return None if the queue is empty, Some(T)
        otherwise.
    **/
    fn peek(&self) -> Option<&T> {
        return self.get(0);
    }
}

/**
    You must implement this function that computes the orthogonal
    distance between two coordinates.  Remember, orthogonal distance
    is not like Euclidean distance.  See the specifications for more
    details.
**/
pub fn distance(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    let (p1n1,p1n2) = p1;
    let (p2n1,p2n2) = p2;
    
    let x = p1n1 - p2n1;
    let y = p1n2 - p2n2;
    return x.abs() + y.abs()
}

/**
    You must implement this function that determines which enemy Stark
    should battle and their coordinates.  You are given two hashmaps for
    allies and enemies.  Each maps a name to their current coordinates.
    You can assume that the allies hashmap will always have a name
    called "Stark" included.  Return the name and coordinates of the enemy
    Stark will battle in the form of a 3-tuple.  See the specifications
    for more details on how to choose which enemy.
**/
pub fn target_locator<'a>(
    allies: &'a HashMap<&String, (i32, i32)>,
    enemies: &'a HashMap<&String, (i32, i32)>,
) -> (&'a str, i32, i32) {
    unimplemented!()
}
