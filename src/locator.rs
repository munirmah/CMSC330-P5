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

        while idx > 0 && self[idx] < self[(idx - 1) / 2] {
            if self[idx] < self[(idx - 1) / 2] {
                self.swap(idx, (idx - 1) / 2);
                idx = (idx - 1) / 2;
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
        let mut result = None;
        if self.len() == 0 {
            return result;
        } else {
            result = Some(self.swap_remove(0));
            let mut idx = 0;
            let mut sidx;

            while self.get(2 * idx + 1) != None {
                sidx = 2 * idx + 1;
                if self.get(2 * idx + 2) != None && (self[2 * idx + 2] < self[2 * idx + 1]) {
                    sidx = 2 * idx + 2;
                }
                if self[idx] > self[sidx] {
                    self.swap(idx, sidx);
                }
                idx = sidx;
            }
        }
        return result;
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
    let (p1n1, p1n2) = p1;
    let (p2n1, p2n2) = p2;

    let x = p1n1 - p2n1;
    let y = p1n2 - p2n2;
    return x.abs() + y.abs();
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
    let mut alst = Vec::new();

    for ally in allies.iter() {
        let (aName, apos) = ally;
        for enemy in enemies.iter() {
            let (eName, epos) = enemy;
            let targ = Node {
                priority: distance(*apos, *epos),
                data: eName,
            };
            alst.enqueue(targ);
        }
    }
    return ("1", 1, 1);
}
