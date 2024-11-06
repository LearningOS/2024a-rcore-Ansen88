# 报告 —— chapter5练习

## 报告人 ：安利军



## 简单总结你实现的功能

spawn系统调用：

+ 创建一个新的任务
+ 加载要执行的代码
+ 将该任务的parent字段设置为当前任务
+ 当前的子任务设置为新创建的任务
+ 返回新任务的 pid



stride调度器：

+ 当设置任务的优先级时，重新设置 pass = 1000000 / 优先级；
+ suspend_current_and_run_next() 函数中实现更新当前任务的 stride += pass；
+ 在 fetch() 函数中，通过判断 stride 获取最小 stride 的任务，并返回执行；



## 问答作业

+ p1.stride = 255, p2.stride = 250，在 p2 执行一个时间片后，理论上下一次应该 p1 执行。实际情况是轮到 p1 执行吗？为什么？

​	下一次要执行 p2，因为 p2溢出后会小于255，所以根据 stride 的调度规则，会重新调度 p2。



+ 如果严格按照算法执行，那么 STRIDE_MAX – STRIDE_MIN <= BigStride / 2。为什么？尝试简单说明（不要求严格证明）。

当所有进程的优先级都大于等于 2 时，意味着每个进程的 stride 增量至少为 2。设 BigStride 是所有进程 stride 增量的最大值，即 255（对于8位无符号整型）。如果所有进程的 stride 增量都至少为 2，那么即使某个进程的 stride 达到了最大值 255，它也只会比其他进程的 stride 最多大 127（255/2），因为任何进程的 stride 增加 2 后就会超过 255 并发生溢出，从而减小到一个小于 128 的值。因此，在不考虑溢出的情况下，STRIDE_MAX – STRIDE_MIN 的差值将不会超过 BigStride / 2。



+ 让 BinaryHeap<Stride> 的 pop 方法能返回真正最小的 Stride。补全下列代码中的* `partial_cmp` 函数，假设两个 Stride 永远不会相等。

```rust
use core::cmp::Ordering;

struct Stride(u64);

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let diff = if self.0 > other.0 {
            self.0 - other.0
        } else {
            other.0 - self.0
        };
        
        if diff <= 127 {
            Some(self.0.cmp(&other.0))
        } else {
            Some(other.0.cmp(&self.0))
        }
    }
    }
}

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}
```



## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我没有与其他人就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容。

   

2. 此外，我参考了网上的一些资料任何资料，还在代码中对应的位置以注释形式记录了具体的参考来源及内容。

   参考了通义千问的答案。

   

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计
