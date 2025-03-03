// use libc::{
//     pthread_attr_t, pthread_create, pthread_t, sched_param, PTHREAD_EXPLICIT_SCHED, SCHED_FIFO,
// };
// use std::ptr;
// use std::thread;
// use std::time::Duration;

// const NUM_THREADS: usize = 5;

// // test cases
// const PRIORITIES: [i32; NUM_THREADS] = [10, 20, 30, 40, 50];
// // const PRIORITIES: [i32; 5] = [50, 40, 30, 20, 10];
// // const PRIORITIES: [i32; 5] = [30, 30, 30, 30, 30];
// // const PRIORITIES: [i32; 5] = [60, 15, 100, 30, 300];

// extern "C" fn runner(param: *mut libc::c_void) -> *mut libc::c_void {
//     let thread_id = param as usize;
//     println!(
//         "Thread {} started with priority {}",
//         thread_id, PRIORITIES[thread_id]
//     );

//     thread::sleep(Duration::from_secs(1));

//     println!("Thread {} finished", thread_id);
//     ptr::null_mut()
// }

// pub fn run_thread() {
//     let mut tid: [pthread_t; NUM_THREADS] = unsafe { std::mem::zeroed() };
//     let mut attr: pthread_attr_t = unsafe { std::mem::zeroed() };

//     unsafe { libc::pthread_attr_init(&mut attr) };

//     unsafe { libc::pthread_attr_setinheritsched(&mut attr, PTHREAD_EXPLICIT_SCHED) };

//     for i in 0..NUM_THREADS {
//         let param = sched_param {
//             sched_priority: PRIORITIES[i],
//         };

//         unsafe {
//             libc::pthread_attr_setschedpolicy(&mut attr, SCHED_FIFO);
//             libc::pthread_attr_setschedparam(&mut attr, &param);
//         }

//         let res = unsafe { pthread_create(&mut tid[i], &attr, runner, i as *mut libc::c_void) };

//         if res != 0 {
//             eprintln!("Failed to create thread {}", i);
//         } else {
//             println!("Thread {} created successfully", i);
//         }
//     }

//     for i in 0..NUM_THREADS {
//         unsafe {
//             let param = sched_param {
//                 sched_priority: PRIORITIES[i],
//             };
//             libc::pthread_setschedparam(tid[i], SCHED_FIFO, &param);
//             libc::pthread_join(tid[i], ptr::null_mut());
//         }
//     }

//     println!("All threads finished");
// }
