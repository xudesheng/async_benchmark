# async_benchmark
Performance benchmark to use async/await in file access.



After I read through great training courses created by https://twitter.com/snoyberg, I want to use it in my recent project, in which I have to deal with a lot of simple CSV file.( [Lesson 8](https://www.snoyman.com/blog/2019/12/rust-crash-course-08-down-dirty-future) and [Lesson 9](https://www.snoyman.com/blog/2019/12/rust-crash-course-09-tokio-0-2)).



To run test on your own machine, you can use following command:

`./target/release/asynctest <tmp_folder> <how many files> <total rows>`. For example:

```bash
./target/release/asynctest ./sample 10 2000000
```

It means: asynctest will create 10 CSV files understand folder ./sample and total lines summurized from all CSV files in total would be 2M.

Asynctest will invoke 10 different binary excutable files and test performance.

In following table, all results are milliseconds. 

|        | Async/Sync, Files/Total Lines | 1/1,000,000 | 5/1,000,000 | 10/10,000,000 | 25/20,000,000 | 50/100,000,000 |
| ------ | ----------------------------- | ----------- | ----------- | ------------- | ------------- | -------------- |
| test00 | tokio, multi tasks            | 731         | 282         | 3116          | 6074          | 32384          |
| test01 | tokio, multi tasks, Arc/Mutex | 701         | 291         | 2737          | 6108          | 31247          |
| test02 | tokio, LocalSet,              | 709         | 504         | 5035          | 9922          | 51788          |
| test03 | tokio, LocalSet, Rc/RefCell   | 702         | 505         | 4953          | 10436         | 52066          |
| test04 | sync, single thread           | 333         | 327         | 3384          | 6759          | 33958          |
| test05 | sync, multi threads           | 446         | 210         | 2047          | 4109          | 24026          |
| test06 | sync, Thread pool             | 417         | 243         | 2214          | 4136          | 20633          |
| test07 | sync, Rayon                   | 406         | 237         | 2075          | 4135          | 20213          |
| test08 | async-std, single task        | 756         | 828         | 8109          | 15963         | 79896          |
| test09 | async-std, multi tasks        | 826         | 289         | 2718          | 4775          | 22827          |

What surprised me:

1. Rayon performs consistently the best in all cases except single file case.
2. Async-std executor performs better than tokio in most cases.



What confused me:

1. Why does Arc/Mutex perform better in release build sometimes? (In debug build, it's consistently less performt.)



Comment and suggestions are welcome and appreciated!

