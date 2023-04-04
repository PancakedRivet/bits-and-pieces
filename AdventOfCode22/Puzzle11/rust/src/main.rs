use std::collections::VecDeque;

// https://adventofcode.com/2022/day/11

// Decalring a type alias to changing the worry score data type easier
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
    println!("PART 1:");
    let rounds: u32 = 20;
    let monkeys: Vec<Monkey> = monkeys_input();
    let worry_closure = Box::new(|x| x / 3);
    do_rounds(rounds, monkeys, worry_closure);

    // Part 2: https://adventofcode.com/2022/day/11#part2

    println!("\nPART 2:");
    let rounds: u32 = 10000;
    let monkeys: Vec<Monkey> = monkeys_input();

    // for part 2: modulo arithmetic means that we can multiply all of the divisor values together and use that as a modulo factor to keep worry scores manageable
    let divisor_product: WorryScore = monkeys.iter().map(|m| m.test_divisor).product::<WorryScore>();
    let worry_closure = Box::new(move |x| x % divisor_product);

    do_rounds(rounds, monkeys, worry_closure);
}

fn do_rounds(rounds: u32, mut monkeys: Vec<Monkey>, worry_closure: Box<dyn Fn(WorryScore) -> WorryScore>) { //worry_closure: &Box<dyn Fn(WorryScore) -> WorryScore>

    for _ in 1..=rounds { // number of rounds
        for j in 0..monkeys.len() { // number of monkeys
            let item_count = monkeys[j].items.len();
            for _ in 0..item_count { // number of items held by a given monkey
                take_turn(&mut monkeys, j, &worry_closure);
            }
        }
    }

    // println!("END");
    // for i in 0..monkeys.len() {
    //     println!("Monkey{}: {:?}: Inspections: {}", i, monkeys[i].items, monkeys[i].inspection_count);
    // }

    // Calculate monkey business
    let mut monkey_business_vec: Vec<WorryScore> = Vec::new();
    for monkey in monkeys {
        monkey_business_vec.push(monkey.inspection_count)
    }
    monkey_business_vec.sort_by(|a, b| b.cmp(a));
    let top_monkeys = &monkey_business_vec[0..2];
    let monkey_business = top_monkeys.iter().map(|m| m).product::<WorryScore>();
    println!("Monnkey Business = {}", monkey_business);

}

fn take_turn(monkeys: &mut Vec<Monkey>, monkey_index: usize, worry_closure: &Box<dyn Fn(WorryScore) -> WorryScore>) {
    let monkey = &mut monkeys[monkey_index];
    let mut item_worry = monkey.items.pop_front().unwrap();
    monkey.increment_count();
    // inspection
    item_worry = monkey.operate_worry(item_worry);
    
    // part 1: relief it's not damaged
    // part 2: finding another way to keep worry low
    item_worry = (worry_closure)(item_worry);
    
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

#[allow(dead_code)]
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