extern crate intcode;
use intcode::IntcodeComp;

// Convenience structure for executing tests
struct IntcodeTest <'a>{
    program: &'a Vec<i64>,
    final_state: Option<Vec<i64>>,
    input: Option<Vec<i64>>,
    output: Option<Vec<i64>>,
}

// Helper to execute tests
fn execute_test(to_run: IntcodeTest){
    let mut comp = IntcodeComp::new(to_run.program);

    // Apply inputs and run the core until done
    match &to_run.input {
        Some(vec) => for i in vec { comp.input(*i) },
        None => ()
    }
    comp.run_all();

    // Check final state is what was provided (if final state is provided)
    match &to_run.final_state {
        Some(vec) => assert_eq!(&comp._int_mem(), &vec),
        None => (),//pass
    };

    // Check output data stream is what was expected
    let mut test_out: Vec<i64> = vec![];
    loop {
        match comp.output() {
            Some(val) => test_out.push(val),
            None => break,
        }
    }

    match &to_run.output {
        Some(vec) => assert_eq!(&test_out, vec),
        None => () // Don't check
    };
}

// Run some sample initial states against known final states to verify core instructions
#[test]
fn basic_test_1() {
    execute_test(IntcodeTest {
        program : &vec![1, 0, 0, 0, 99],
        final_state : Some(vec![2, 0, 0, 0, 99]),
        input : None,
        output : None});
}
#[test]
fn basic_test_2() {
    execute_test(IntcodeTest {
        program : &vec![2, 3, 0, 3, 99],
        final_state : Some(vec![2, 3, 0, 6, 99]),
        input : None,
        output : None});
}
#[test]
fn basic_test_3() {
    execute_test(IntcodeTest {
        program : &vec![2,4,4,5,99,0],
        final_state : Some(vec![2,4,4,5,99,9801]),
        input : None,
        output : None});
}
#[test]
fn basic_test_4() {
    execute_test(IntcodeTest {
        program : &vec![1,1,1,4,99,5,6,0,99],
        final_state : Some(vec![30,1,1,4,2,5,6,0,99]),
        input : None,
        output : None});
}

// Verify conditional jump works as expected (jump if equal or not equal)
// These tests also verify that IO works as expected, and also limited addressing mode tests
#[test]
fn check_equal() {
    // this program returns 1 if the input == 8, else 0
    let gt_prog = vec![3,9,8,9,10,9,4,9,99,-1,8];
    execute_test(IntcodeTest {
        program: &gt_prog,
        final_state : None,
        input : Some(vec![8]),
        output : Some(vec![1])});
}
#[test]
fn check_not_equal() {
// this program returns 1 if the input == 8, else 0
    let gt_prog = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
    execute_test(IntcodeTest {
        program: &gt_prog,
        final_state: None,
        input: Some(vec![1]),
        output: Some(vec![0]),
    });
}

// These test zero-detection and related conditional jumps
#[test]
fn check_zero() {
    // This outputs 0 if input is 0, else output is 1
    let zero_prog = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
    execute_test(IntcodeTest {
        program: &zero_prog,
        final_state : None,
        input : Some(vec![0]),
        output : Some(vec![0])});
}
#[test]
fn check_nonzero() {
    // This outputs 0 if input is 0, else output is 1
    let zero_prog = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
    execute_test(IntcodeTest {
        program: &zero_prog,
        final_state : None,
        input : Some(vec![5]),
        output : Some(vec![1])});
}

// Test that instruction 9 (relative base offset adj) works as expected
#[test]
fn self_copy() {
    // This sample program simply replicates itself in the output stream
    execute_test(IntcodeTest { // test instruction 9
        program: &vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99],
        final_state : None,
        input : None,
        output : Some(vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99])});
}
#[test]
fn output_middle_number() {
    // This should output the number in the middle, this also checks that large ints work
    execute_test(IntcodeTest {
        program: &vec![104,1125899906842624,99],
        final_state : None,
        input : None,
        output : Some(vec![1125899906842624])});

}
