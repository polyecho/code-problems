// https://www.algoexpert.io/questions/move-element-to-end

struct TestCase<'a, 'b> {
    array: &'a [i32],
    to_move: i32,
    answer: &'b [i32],
}

const TEST_CASES: [TestCase; 11] = [
    TestCase {
        array: &[2, 1, 2, 2, 2, 3, 4, 2],
        to_move: 2,
        answer: &[4, 1, 3, 2, 2, 2, 2, 2],
    },
    TestCase {
        array: &[],
        to_move: 3,
        answer: &[],
    },
    TestCase {
        array: &[1, 2, 4, 5, 6],
        to_move: 3,
        answer: &[1, 2, 4, 5, 6],
    },
    TestCase {
        array: &[3, 3, 3, 3, 3],
        to_move: 3,
        answer: &[3, 3, 3, 3, 3],
    },
    TestCase {
        array: &[3, 1, 2, 4, 5],
        to_move: 3,
        answer: &[5, 1, 2, 4, 3],
    },
    TestCase {
        array: &[1, 2, 4, 5, 3],
        to_move: 3,
        answer: &[1, 2, 4, 5, 3],
    },
    TestCase {
        array: &[1, 2, 3, 4, 5],
        to_move: 3,
        answer: &[1, 2, 5, 4, 3],
    },
    TestCase {
        array: &[2, 4, 2, 5, 6, 2, 2],
        to_move: 2,
        answer: &[6, 4, 5, 2, 2, 2, 2],
    },
    TestCase {
        array: &[5, 5, 5, 5, 5, 5, 1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12],
        to_move: 5,
        answer: &[12, 11, 10, 9, 8, 7, 1, 2, 3, 4, 6, 5, 5, 5, 5, 5, 5],
    },
    TestCase {
        array: &[1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 5, 5, 5, 5, 5, 5],
        to_move: 5,
        answer: &[1, 2, 3, 4, 6, 7, 8, 9, 10, 11, 12, 5, 5, 5, 5, 5, 5],
    },
    TestCase {
        array: &[5, 1, 2, 5, 5, 3, 4, 6, 7, 5, 8, 9, 10, 11, 5, 5, 12],
        to_move: 5,
        answer: &[12, 1, 2, 11, 10, 3, 4, 6, 7, 9, 8, 5, 5, 5, 5, 5, 5],
    },
];

fn main() {
    for item in TEST_CASES {
        let mut array = item.array.to_vec();
        let result = move_element_to_end(&mut array, item.to_move);
        assert_eq!(result, item.answer);
    }

    println!("Yay, your code passed all the test cases!");
}

fn move_element_to_end(array: &mut Vec<i32>, to_move: i32) -> &mut Vec<i32> {
    if array.is_empty() {
        return array;
    }

    let mut i = 0;
    let mut j = array.len() - 1;

    while i < j {
        while i < j && array[j] == to_move {
            j -= 1
        }

        if array[i] == to_move {
            let (element_i, element_j) = (array[i], array[j]);
            array[i] = element_j;
            array[j] = element_i;
        }

        i += 1
    }

    array
}
