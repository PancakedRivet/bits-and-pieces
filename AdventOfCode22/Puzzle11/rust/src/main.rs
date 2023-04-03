use std::collections::VecDeque;

// https://adventofcode.com/2022/day/11

type WorryScore = u64;

struct Monkey {
    items: VecDeque<WorryScore>,
    operation: Box<dyn Fn(WorryScore) -> WorryScore>,
    test_divisor: WorryScore,
    test_true: usize,
    test_false: usize,
    inspection_count: WorryScore,
}

impl Monkey {
    fn less_worried(worry: WorryScore) -> WorryScore {
         worry / 3
    }
    fn operate_worry(self: &Self, worry: WorryScore) -> WorryScore {
        (self.operation)(worry)
    }
    fn test_worry(self: &Self, worry: WorryScore) -> bool {
        (worry % self.test_divisor) == 0
    }
    fn increment_count(self: &mut Self) {
        self.inspection_count += 1;
    }
}

fn main() {
    const ROUNDS_PART1: u8 = 20;
    let mut monkeys: Vec<Monkey> = monkeys_input();

    println!("PART 1:");
    println!("START");
    for i in 0..monkeys.len() {
        println!("Monkey{}: {:?}: Inspections: {}", i, monkeys[i].items, monkeys[i].inspection_count);
    }

    for _ in 1..=ROUNDS_PART1 {

        // println!("Round {} Start", i);

        for j in 0..monkeys.len() {

            // println!("Monkey {} Start", j);

            let item_count = monkeys[j].items.len();
            for _ in 0..item_count {
                take_turn(&mut monkeys, j, 0);
            }
            
            // println!("{:?}", monkeys[0].items);
            // println!("{:?}", monkeys[1].items);
            // println!("{:?}", monkeys[2].items);
            // println!("{:?}", monkeys[3].items);
            // println!("Monkey {} End", j);
        }

        // println!("Round {} End", i);

    }

    println!("END");
    for i in 0..monkeys.len() {
        println!("Monkey{}: {:?}: Inspections: {}", i, monkeys[i].items, monkeys[i].inspection_count);
    }

    // Part 2: https://adventofcode.com/2022/day/11#part2

    const ROUNDS_PART2: u32 = 10000;

    let mut monkeys: Vec<Monkey> = monkeys_input();

    // for part 2: modulo arithmetic means that we can multiply all of the divisor values together and use that as a modulo factor to keep worry scores manageable
    let divisor_product = monkeys.iter().map(|m| m.test_divisor).product::<WorryScore>();

    println!("PART 2:");
    println!("START");
    for i in 0..monkeys.len() {
        println!("Monkey{}: {:?}: Inspections: {}", i, monkeys[i].items, monkeys[i].inspection_count);
    }

    for _ in 1..=ROUNDS_PART2 {

        // println!("Round {} Start", i);

        for j in 0..monkeys.len() {

            // println!("Monkey {} Start", j);

            let item_count = monkeys[j].items.len();
            for _ in 0..item_count {
                take_turn(&mut monkeys, j, divisor_product);
            }
            
            // println!("{:?}", monkeys[0].items);
            // println!("{:?}", monkeys[1].items);
            // println!("{:?}", monkeys[2].items);
            // println!("{:?}", monkeys[3].items);
            // println!("Monkey {} End", j);
        }

        // println!("Round {} End", i);

    }

    println!("END");
    for i in 0..monkeys.len() {
        println!("Monkey{}: {:?}: Inspections: {}", i, monkeys[i].items, monkeys[i].inspection_count);
    }

}

fn take_turn(monkeys: &mut Vec<Monkey>, monkey_index: usize, divisor_product: WorryScore) {
    let monkey = &mut monkeys[monkey_index];
    let mut item_worry = monkey.items.pop_front().unwrap();
    monkey.increment_count();
    // inspection
    item_worry = monkey.operate_worry(item_worry);
    // part 1: relief it's not damaged
    // item_worry = Monkey::less_worried(item_worry);
    // part 2:finding another way to keep worry low
    item_worry %= divisor_product;
    // test if it's divisble
    let item_test = monkey.test_worry(item_worry).clone();
    // choose the monkey to throw item to
    let to_monnkey_index = match item_test {
        true => monkey.test_true,
        false => monkey.test_false,
    };
    // throw the item
    let to_monkey = &mut monkeys[to_monnkey_index];
    to_monkey.items.push_back(item_worry);
}

fn monkeys_input() -> Vec<Monkey> {
    let monkey0: Monkey = Monkey {
        items: VecDeque::from([65, 78]),
        operation: Box::new(|x| x * 3),
        test_divisor: 5,
        test_true: 2,
        test_false: 3,
        inspection_count: 0
    };
    let monkey1: Monkey = Monkey {
        items: VecDeque::from([54, 78, 86, 79, 73, 64, 85, 88]),
        operation: Box::new(|x| x + 8),
        test_divisor: 11,
        test_true: 4,
        test_false: 7,
        inspection_count: 0
    };
    let monkey2: Monkey = Monkey {
        items: VecDeque::from([69, 97, 77, 88, 87]),
        operation: Box::new(|x| x + 2),
        test_divisor: 2,
        test_true: 5,
        test_false: 3,
        inspection_count: 0
    };
    let monkey3: Monkey = Monkey {
        items: VecDeque::from([99]),
        operation: Box::new(|x| x + 4),
        test_divisor: 13,
        test_true: 1,
        test_false: 5,
        inspection_count: 0
    };
    let monkey4: Monkey = Monkey {
        items: VecDeque::from([60, 57, 52]),
        operation: Box::new(|x| x * 19),
        test_divisor: 7,
        test_true: 7,
        test_false: 6,
        inspection_count: 0
    };
    let monkey5: Monkey = Monkey {
        items: VecDeque::from([91, 82, 85, 73, 84, 53]),
        operation: Box::new(|x| x + 5),
        test_divisor: 3,
        test_true: 4,
        test_false: 1,
        inspection_count: 0
    };
    let monkey6: Monkey = Monkey {
        items: VecDeque::from([88, 74, 68, 56]),
        operation: Box::new(|x| x * x),
        test_divisor: 17,
        test_true: 0,
        test_false: 2,
        inspection_count: 0
    };
    let monkey7: Monkey = Monkey {
        items: VecDeque::from([54, 82, 72, 71, 53, 99, 67]),
        operation: Box::new(|x| x + 1),
        test_divisor: 19,
        test_true: 6,
        test_false: 0,
        inspection_count: 0
    };
    let monkeys: Vec<Monkey> = vec![monkey0,monkey1,monkey2,monkey3,monkey4,monkey5,monkey6,monkey7];

    monkeys
}

fn monkeys_input2() -> Vec<Monkey> {
    let monkey0: Monkey = Monkey {
        items: VecDeque::from([79, 98]),
        operation: Box::new(|x| x * 19),
        test_divisor: 23,
        test_true: 2,
        test_false: 3,
        inspection_count: 0
    };
    let monkey1: Monkey = Monkey {
        items: VecDeque::from([54, 65, 75, 74]),
        operation: Box::new(|x| x + 6),
        test_divisor: 19,
        test_true: 2,
        test_false: 0,
        inspection_count: 0
    };
    let monkey2: Monkey = Monkey {
        items: VecDeque::from([79, 60, 97]),
        operation: Box::new(|x| x * x),
        test_divisor: 13,
        test_true: 1,
        test_false: 3,
        inspection_count: 0
    };
    let monkey3: Monkey = Monkey {
        items: VecDeque::from([74]),
        operation: Box::new(|x| x + 3),
        test_divisor: 17,
        test_true: 0,
        test_false: 1,
        inspection_count: 0
    };
    let monkeys: Vec<Monkey> = vec![monkey0,monkey1,monkey2,monkey3];

    monkeys
}