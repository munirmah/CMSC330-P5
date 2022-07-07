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
        let mut idx = self.len() as i32;
        self.push(ele);
        let mut curr;
        let mut parent;

        curr = self.get(idx as usize).unwrap();
        parent = self.get(((idx - 1) / 2) as usize).unwrap();

        while parent.partial_cmp(&curr) == Some(Ordering::Greater) {
            self.swap(idx as usize, ((idx - 1) / 2) as usize);

            idx = (idx - 1) / 2;

            curr = self.get(idx as usize).unwrap();
            parent = self.get(((idx - 1) / 2) as usize).unwrap();
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
        /*         if self.len() == 0 {
            return None;
        } else {
            let result = self.swap_remove(0);

            let mut idx = 0;
            let mut curr;
            let mut child1;
            let mut child2;

            curr = self.get(idx).unwrap();
            child1 = self.get(2 * idx + 1).unwrap();
            child2 = self.get(2 * idx + 2).unwrap();

            while curr > child1 || curr > child2 {
                if curr > child1 && curr < child2 {
                    /* swap curr and child1 */
                    self.swap(2 * idx + 1, idx);
                    idx = 2 * idx + 1;

                    curr = self.get(idx).unwrap();
                    child1 = self.get(2 * idx + 1).unwrap();
                    child2 = self.get(2 * idx + 2).unwrap();
                } else if curr < child1 && curr > child2 {
                    /* swap curr and child2 */
                    self.swap(2 * idx + 2, idx);
                    idx = 2 * idx + 2;

                    curr = self.get(idx).unwrap();
                    child1 = self.get(2 * idx + 1).unwrap();
                    child2 = self.get(2 * idx + 2).unwrap();
                } else {
                    if curr < child1 && curr < child2 {
                        if child1 < child2 {
                            /* Swap curr with child1 */
                            self.swap(2 * idx + 1, idx);
                            idx = 2 * idx + 1;

                            curr = self.get(idx).unwrap();
                            child1 = self.get(2 * idx + 1).unwrap();
                            child2 = self.get(2 * idx + 2).unwrap();
                        } else {
                            /* Swap curr with child2 */
                            self.swap(2 * idx + 2, idx);
                            idx = 2 * idx + 2;

                            curr = self.get(idx).unwrap();
                            child1 = self.get(2 * idx + 1).unwrap();
                            child2 = self.get(2 * idx + 2).unwrap();
                        }
                    } else {
                        break;
                    }
                }
            }
            return Some(result);
        } */
        if self.len() == 0 {
            return None;
        } else {
            let result = self.swap_remove(0);

            let mut idx = 0;
            let mut curr;
            let mut child1;
            let mut child2;

            curr = self.get(idx).unwrap();
            child1 = self.get(2 * idx + 1).unwrap();
            child2 = self.get(2 * idx + 2).unwrap();
            if self.len() == 2 {
                if self[0] > self[1] {
                    self.swap(idx, idx + 1);
                }
            }
            while self.get(2 * idx + 1) != None
                && self.get(2 * idx + 2) != None
                && (curr > self.get(2 * idx + 1).unwrap() || curr > self.get(2 * idx + 2).unwrap())
            {
                if curr > self.get(2 * idx + 1).unwrap() && curr < self.get(2 * idx + 2).unwrap() {
                    /* swap curr and child1 */
                    self.swap(2 * idx + 1, idx);
                    idx = 2 * idx + 1;
                    if idx > self.len() {
                        break;
                    };
                    curr = self.get(idx).unwrap();
                //                   child1 = self.get(2 * idx + 1).unwrap();
                //                  child2 = self.get(2 * idx + 2).unwrap();
                } else if curr < self.get(2 * idx + 1).unwrap()
                    && curr > self.get(2 * idx + 2).unwrap()
                {
                    /* swap curr and child2 */
                    self.swap(2 * idx + 2, idx);
                    idx = 2 * idx + 2;
                    if idx > self.len() {
                        break;
                    };

                    curr = self.get(idx).unwrap();
                //                  child1 = self.get(2 * idx + 1).unwrap();
                //                    child2 = self.get(2 * idx + 2).unwrap();
                } else {
                    println!("Hello");
                    if curr > self.get(2 * idx + 1).unwrap()
                        && curr > self.get(2 * idx + 2).unwrap()
                    {
                        if self.get(2 * idx + 1).unwrap() < self.get(2 * idx + 2).unwrap() {
                            /* Swap curr with child1 */
                            self.swap(2 * idx + 1, idx);
                            idx = 2 * idx + 1;
                            if idx > self.len() {
                                break;
                            };

                            curr = self.get(idx).unwrap();
                        //                   child1 = self.get(2 * idx + 1).unwrap();
                        //                  child2 = self.get(2 * idx + 2).unwrap();
                        } else {
                            /* Swap curr with child2 */
                            self.swap(2 * idx + 2, idx);
                            idx = 2 * idx + 2;
                            if idx > self.len() {
                                break;
                            };

                            curr = self.get(idx).unwrap();
                            //                  if self.get(2 * idx + 1) != None{
                            //                    child1 = self.get(2 * idx + 1).unwrap();
                            //                      }
                            //                        if self.get(2* idx + 2) != None{
                            //                          child2 = self.get(2 * idx + 2).unwrap();
                            //                  }
                        }
                    } else {
                        break;
                    }
                }
            }
            return Some(result);
        }
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
    unimplemented!()
}
