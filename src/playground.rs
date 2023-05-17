use std::thread::JoinHandle;
use std::{thread, time};

// use futures::executor::block_on;
use futures::join;

use async_std;
use futures::future::join_all;
use std::vec::Vec;

fn dosomething_synchronous(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);

    thread::sleep(two_seconds);

    return 2;
}

pub fn evaluate_dosomething_synchronous() {
    let now = time::Instant::now();

    let one = dosomething_synchronous(1);
    let two = dosomething_synchronous(2);
    let three = dosomething_synchronous(3);

    println!("time elapsed {:?}", now.elapsed());
    println!("result {}", one + two + three);
}

pub fn evaluate_dosomething_asynchronous() {
    let now = time::Instant::now();

    let thread_one: JoinHandle<i8> = thread::spawn(|| dosomething_synchronous(1));
    let thread_two: JoinHandle<i8> = thread::spawn(|| dosomething_synchronous(2));
    let thread_three: JoinHandle<i8> = thread::spawn(|| dosomething_synchronous(3));

    let result_one = thread_one.join();
    let result_two = thread_two.join();
    let result_three = thread_three.join();

    println!("time elapsed {:?}", now.elapsed());

    println!(
        "result {}",
        result_one.unwrap() + result_two.unwrap() + result_three.unwrap()
    );
}

async fn dosomething_futures(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);

    thread::sleep(two_seconds);

    return 2;
}

pub async fn evaluate_dosomething_futures() {
    let now = time::Instant::now();

    // let future_one = dosomething_futures(1).await;
    // let future_two = dosomething_futures(2).await;

    let future_one = dosomething_futures(1);
    let future_two = dosomething_futures(2);

    let result = join!(future_one, future_two);

    let two_seconds = time::Duration::new(2, 0);

    thread::sleep(two_seconds);

    //  our future does not execute until we apply an executor using the block_on function.
    // let outcome = block_on(future_one);

    println!("time elapsed {:?}", now.elapsed());
    // println!("Here is the outcome: {}", outcome);
    // println!("Here is the outcome: {}", future_one);
    // println!("Here is the outcome: {}", future_two);
    println!("Here is the outcome: {:?}", result);

    return;
}

pub async fn evaluate_dosomething_futures_with_async_task() -> i8 {
    let now = time::Instant::now();
    // 1
    let mut futures_vec = Vec::new();
    let future_four = dosomething_futures(4);
    let future_five = dosomething_futures(5);

    // 2
    futures_vec.push(future_four);
    futures_vec.push(future_five);

    // 3 Loop through the vector, spinning off tasks for each future in the vector.
    let handles = futures_vec
        .into_iter()
        .map(async_std::task::spawn)
        .collect::<Vec<_>>();

    // 4
    let result = join_all(handles).await;

    let val = result.into_iter().sum::<i8>();

    // let two_seconds = time::Duration::new(2, 0);

    // thread::sleep(two_seconds);

    println!("time elapsed {:?}", now.elapsed());

    val
}
