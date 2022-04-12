mod topic_03_01;
mod topic_03_02;
mod topic_03_03;
mod topic_03_04;
mod topic_03_05;
mod topic_03_06;

#[cfg(test)]
mod tests {
    use crate::topic_03_01::TripleInOne;
    use crate::topic_03_02::MinStack;
    use crate::topic_03_03::StackOfPlates;
    use crate::topic_03_04::MyQueue;
    use crate::topic_03_05::SortedStack;
    use crate::topic_03_06::AnimalShelf;

    #[test]
    fn solution_03_01() {
        let mut test1 = TripleInOne::new(1);
        test1.push(0, 1);
        test1.push(0, 2);
        println!("{}", test1.pop(0));
        println!("{}", test1.pop(0));
        println!("{}", test1.pop(0));
        println!("{}", test1.is_empty(0));

        let mut test2 = TripleInOne::new(2);
        test2.push(0, 1);
        test2.push(0, 2);
        test2.push(0, 3);
        println!("{}", test2.pop(0));
        println!("{}", test2.pop(0));
        println!("{}", test2.pop(0));
        println!("{}", test2.peek(0));
    }

    #[test]
    fn solution_03_02() {
        let mut test = MinStack::new();
        test.push(-2);
        test.push(0);
        test.push(-3);
        println!("{}", test.get_min());
        test.pop();
        println!("{}", test.top());
        println!("{}", test.get_min());
    }

    #[test]
    fn solution_03_03() {
        let mut test = StackOfPlates::new(1);
        test.push(1);
        test.push(2);
        println!("{}", test.pop_at(1));
        println!("{}", test.pop());
        println!("{}", test.pop());

        let mut test1 = StackOfPlates::new(2);
        test1.push(1);
        test1.push(2);
        test1.push(3);
        println!("{}", test1.pop_at(0));
        println!("{}", test1.pop_at(0));
        println!("{}", test1.pop_at(0));

        let mut test2 = StackOfPlates::new(0);
        test2.push(5);
        println!("{}", test2.pop());
        println!("{}", test2.pop_at(0));
        test2.push(6);
        println!("{}", test2.pop());
        println!("{}", test2.pop_at(0));
        println!("{}", test2.pop());
        test2.push(8);
    }

    #[test]
    fn solution_03_04() {
        let mut test = MyQueue::new();
        test.push(1);
        test.push(2);
        println!("{}", test.peek());
        println!("{}", test.pop());
        println!("{}", test.empty());
    }

    #[test]
    fn solution_03_05() {
        let mut test = SortedStack::new();
        test.push(1);
        test.push(2);
        println!("{}", test.peek());
        test.pop();
        println!("{}", test.peek());

        let mut test1 = SortedStack::new();
        test1.pop();
        test1.pop();
        test1.push(1);
        test1.pop();
        println!("{}", test1.is_empty());
    }

    #[test]
    fn solution_03_06() {
        let mut test = AnimalShelf::new();
        test.enqueue(vec![0, 0]);
        test.enqueue(vec![1, 0]);
        println!("{:?}", test.dequeue_cat());
        println!("{:?}", test.dequeue_dog());
        println!("{:?}", test.dequeue_any());
    }
}
