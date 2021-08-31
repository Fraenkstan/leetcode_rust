
mod topic_05_01;
mod topic_05_02;
mod topic_05_03;
mod topic_05_04;
mod topic_05_06;
mod topic_05_07;

#[cfg(test)]
mod tests {
    use crate::topic_05_01::insert_bits;
    use crate::topic_05_02::print_bin;
    use crate::topic_05_03::reverse_bits;
    use crate::topic_05_04::find_closed_numbers;
    use crate::topic_05_06::convert_integer;
    use crate::topic_05_07::exchange_bits;

    #[test]
    fn solution_05_01() {
        println!("{}", insert_bits(1024, 19, 2, 6));
        println!("{}", insert_bits(0, 31, 0, 4));
    }

    #[test]
    fn solution_05_02() {
        println!("{}", print_bin(0.625));
        println!("{}", print_bin(0.1));
    }

    #[test]
    fn solution_05_03() {
        println!("{}", reverse_bits(1775));
        println!("{}", reverse_bits(7));
    }

    #[test]
    fn solution_05_04() {
        println!("{:?}", find_closed_numbers(2));
        println!("{:?}", find_closed_numbers(1));
        println!("{:?}", find_closed_numbers(34));
    }

    #[test]
    fn solution_05_06() {
        println!("{}", convert_integer(29, 15));
        println!("{}", convert_integer(1, 2));
    }

    #[test]
    fn solution_05_07() {
        println!("{}", exchange_bits(2));
        println!("{}", exchange_bits(3));
    }
}
