# async_benchmark
Performance benchmark to use async/await in file access.

This is a simple benchmark for: [async-std](https://docs.rs/async-std/1.4.0/async_std/), [tokio](https://docs.rs/tokio/0.2.6/tokio/),[Rayon](https://docs.rs/rayon/1.3.0/rayon/). 



After I read through great training courses created by https://twitter.com/snoyberg, I want to use it in my recent project, in which I have to deal with a lot of simple CSV file.( [Lesson 8](https://www.snoyman.com/blog/2019/12/rust-crash-course-08-down-dirty-future) and [Lesson 9](https://www.snoyman.com/blog/2019/12/rust-crash-course-09-tokio-0-2)).



To run test on your own machine, you can use following command:

`./target/release/asynctest <tmp_folder> <how many files> <total rows>`. For example:

```bash
./target/release/asynctest ./sample 10 2000000
```

It means: asynctest will create 10 CSV files understand folder ./sample and total lines summurized from all CSV files in total would be 2M.

Asynctest will invoke 10 different binary excutable files and test performance.

In below table, all results are milliseconds.  (VM: Azure D4_v3, 4 vCPU, 16G Memory, SSD)

|        | Async/Sync, Files/Total Lines | 1/1,000,000 | 5/1,000,000 | 10/10,000,000 | 25/20,000,000 | 50/100,000,000 |
| ------ | ----------------------------- | ----------- | ----------- | ------------- | ------------- | -------------- |
| test00 | tokio, multi tasks            | 731         | 282         | 3,116         | 6,074         | 32,384         |
| test01 | tokio, multi tasks, Arc/Mutex | 701         | 291         | 2,737         | 6,108         | 31,247         |
| test02 | tokio, LocalSet,              | 709         | 504         | 5,035         | 9,922         | 51,788         |
| test03 | tokio, LocalSet, Rc/RefCell   | 702         | 505         | 4,953         | 10,436        | 52,066         |
| test04 | sync, single thread           | 333         | 327         | 3,384         | 6,759         | 33,958         |
| test05 | sync, multi threads           | 446         | 210         | 2,047         | 4,109         | 24,026         |
| test06 | sync, Thread pool             | 417         | 243         | 2,214         | 4,136         | 20,633         |
| test07 | sync, Rayon                   | 406         | 237         | 2,075         | 4,135         | 20,213         |
| test08 | async-std, single task        | 756         | 828         | 8,109         | 15,963        | 79,896         |
| test09 | async-std, multi tasks        | 826         | 289         | 2,718         | 4,775         | 22,827         |

What surprised me:

1. `Rayon` performs consistently the best in all cases except single file case.
2. `Async-std` executor performs better than `tokio` in most cases.

What confused me:

1. Why does Arc/Mutex perform better in release build sometimes? (In debug build, it's consistently less performt.)



In below table, all results are milliseconds.  (MBP-15, 2017, 2.9 GHz Quad-Core i7, 16 GB memory, SSD)

|        | Async/Sync, Files/Total Lines | 1/1,000,000 | 5/1,000,000 | 10/10,000,000 | 25/20,000,000 | 50/100,000,000 |
| ------ | ----------------------------- | ----------- | ----------- | ------------- | ------------- | -------------- |
| test00 | tokio, multi tasks            | 1,101       | 428         | 3,237         | 6,606         | 49,204         |
| test01 | tokio, multi tasks, Arc/Mutex | 1,168       | 463         | 3,018         | 6,332         | 32,481         |
| test02 | tokio, LocalSet,              | 1,298       | 1,166       | 7,780         | 15,868        | 71,896         |
| test03 | tokio, LocalSet, Rc/RefCell   | 1,055       | 871         | 7,432         | 17,127        | 74,117         |
| test04 | sync, single thread           | 773         | 718         | 6,701         | 13,672        | 62,301         |
| test05 | sync, multi threads           | 793         | 235         | 2,413         | 4,946         | 21,347         |
| test06 | sync, Thread pool             | 747         | 280         | 2,555         | 5,347         | 23,321         |
| test07 | sync, Rayon                   | 840         | 255         | 2,554         | 5,370         | 22,487         |
| test08 | async-std, single task        | 1,126       | 1,168       | 8,799         | 17,265        | 79,682         |
| test09 | async-std, multi tasks        | 1,044       | 411         | 3,114         | 6,156         | 25,363         |





Comment and suggestions are welcome and appreciated!

