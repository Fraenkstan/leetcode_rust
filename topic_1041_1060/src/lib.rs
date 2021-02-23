
mod topic_1052;

#[cfg(test)]
mod tests {
    use crate::topic_1052::max_satisfied;

    #[test]
    fn solution_1052() {
        let customers = vec![1,0,1,2,1,1,7,5];
        let grumpy =  vec![0,1,0,1,0,1,0,1];
        customers.iter().zip(grumpy.iter()).for_each(|(customer, grumpy)|
            println!("{}, {}", customer, grumpy));
        println!("{}", max_satisfied(customers, grumpy, 3));
    }
}
