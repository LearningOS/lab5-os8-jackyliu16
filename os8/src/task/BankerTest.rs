// 最后感觉这个模块太麻烦了，直接拆除

// const THREAD_NUM: usize = 50;
// const RESOURCE_NUM: usize = 50;
// // NOTE：这个地方逻辑上我的想法在于实现一个动态的数据结构，这个结构process持有，当process打开开关之后，会将某个元素切换成为true，
// // 这个元素会导致我们在进行mutex and semphare 相关操作的时候需要考虑对应的操作结果。
// // 而这边的模块则是负责实现这边的操作, 出于不了解在rust中存在哪些相关的动态生成数组的操作，我尝试通过一种静态的方式生成每一个process的所有资源，并且维持其直到结束
// // 但是存在一个相对复杂的问题在于，我们如何获取我们所需要的数据的内容（我们怎么知道某些task需要多少个lock？来方便完成任务？？？）
// // Reduce the number of parameters passed
// pub struct Point{
//     x: i32,
//     y: i32,
// }

// #[allow(unused_variables, dead_code)]
// pub struct BankerTest{
//     available: [i32; RESOURCE_NUM],                     // the source we have right now
//     allocation: [[i32; THREAD_NUM] ; RESOURCE_NUM],     // allocation of lock right now
//     need: [[i32; THREAD_NUM] ; RESOURCE_NUM],           // the lock of things process need
//     // actually_used: (i32, i32), // RESOURCE, THREAD   // saving the information about how many thread and how many resource actually use
//     np: usize,    // number of process
//     ns: usize,    // number of source
// }

// impl BankerTest {
//     pub fn new() -> Self{
//         BankerTest {
//             available: [0; RESOURCE_NUM],
//             allocation: [[0; THREAD_NUM]; RESOURCE_NUM],
//             need: [[0; THREAD_NUM]; RESOURCE_NUM],
//             np:0,
//             ns:0,
//         }
//     }
//     // 设想是每一次在process中修改一个元素，然后修改完对应元素之后，调用该元素所存在的safety check 函数，就可以实现对于函数运行安全状态的检查
//     // init when we trying to open or close the method

//     // TODO 当前无法确定对应的元素（NP,NS）是否会发生变化
//     pub fn clear(&mut self) {
//         self.available = [0; RESOURCE_NUM];
//         self.allocation = [[0; THREAD_NUM]; RESOURCE_NUM];
//         self.need = [[0; THREAD_NUM]; RESOURCE_NUM];
//         self.np = 0;
//         self.ns = 0;
//     }

//     pub fn safety_check(&self) -> bool {
//         let mut work = self.available.clone();
//         let mut finish = [false; THREAD_NUM];   // for each process

//         // TODO 这个地方其实可以采用更加简单的算法的
//         'outer: for _ in 0..self.np*self.np {
//             for i in 0..=self.np {
//                 // skep this process
//                 if finish[i] {
//                     continue;
//                 }
//                 // check this process
//                 for j in 0..=self.ns {
//                     if self.need[i][j] > work[j] {
//                         continue;
//                     }
//                 }
//                 // if all source has been allocate correctly
//                 for j in 0..=self.ns {
//                     work[j] += self.allocation[i][j];
//                     finish[i] = true;
//                 }
//                 // 此处可以尝试加载新一个元素
//             }
//             for j in 0..=self.np {
//                 if !finish[j] {
//                     break 'outer;
//                 }
//             }
//             return true;
//         }
//         false
//     }

// }