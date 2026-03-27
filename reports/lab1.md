# report

- author: Qiumix

## summary

这次我实现了一个叫trace的syscall，刚开始以为和xv6的syscall的lab是一样的，研究后发现实则大不相同，不过rCore的要更简单很多。
直接又加了一个counters数组在TaskManagerInner里，然后每个进程syscall开头会进行自增。
然后遇到了奇怪的问题，每次运行make docker后，所有代码文件的所有权就不是我的了，再改要chown，有点小麻烦，不过个人不是很懂docker。

## 简答作业

### 1

这是sbi版本(md的内嵌代码用的maple mono的斜体显示，字符画怎么有点别扭，碎碎念)

```sh
[rustsbi] RustSBI version 0.3.0-alpha.4, adapting to RISC-V SBI v1.0.0
.______       __    __      _______.___________.  _______..______   __
|   _  \     |  |  |  |    /       |           | /       ||   _  \ |  |
|  |_)  |    |  |  |  |   |   (----`---|  |----`|   (----`|  |_)  ||  |
|      /     |  |  |  |    \   \       |  |      \   \    |   _  < |  |
|  |\  \----.|  `--'  |.----)   |      |  |  .----)   |   |  |_)  ||  |
| _| `._____| \______/ |_______/       |__|  |_______/    |______/ |__|
[rustsbi] Implementation     : RustSBI-QEMU Version 0.2.0-alpha.2
[rustsbi] Platform Name      : riscv-virtio,qemu
```

```sh
[kernel] Loading app_0 # 第0个用例: (0x0 as *mut u8).write_volatile(0)写入空指针触发段错误
[kernel] PageFault in application, kernel killed it.
[kernel] Loading app_1 # 第1个用例: supervisor ret, 用户态不能用喵
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] Loading app_2 # 第2个用例: sstatus属于S模式，用户态不能访问
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] Loading app_3 # hello world正常
Hello, world from user mode program!
```

### 2

#### 2.1

> 这时候sp存的还是内核的栈指针
> 从时钟中断到内核态恢复到用户态，从syscall到内核态恢复到用户态

#### 2.2

```asm
csrr t0, sstatus
csrr t1, sepc
sd t0, 32*8(sp)
sd t1, 33*8(sp)
```

↑\_\_alltraps中

```asm
ld t0, 32*8(sp)
ld t1, 33*8(sp)
csrw sstatus, t0
csrw sepc, t1
```

↑\_\_restore中是相反过程，恢复了sstatus和sepc，
前者放进入内核态前特权的状态，后者是一个特殊pc，trap时会把当前pc存入。

xv6里的解释

> machine exception program counter, holds the
> instruction address to which a return from
> exception will go.

贴一个xv6的头文件

```c
#define SSTATUS_SPP (1L << 8)  // Previous mode, 1=Supervisor, 0=User
#define SSTATUS_SPIE (1L << 5) // Supervisor Previous Interrupt Enable
#define SSTATUS_UPIE (1L << 4) // User Previous Interrupt Enable
#define SSTATUS_SIE (1L << 1)  // Supervisor Interrupt Enable
#define SSTATUS_UIE (1L << 0)  // User Interrupt Enable
```

sscratch是一个特殊寄存器，因为其他的寄存器被占用了无法使用，可以用来放栈指针

### 3

- x2是sp，要基于sp+内存偏移值来找其他寄存器，所以不能动
- x4是tp，thread pointer，留给多线程用的

### 4

在这之前sp里存内核态栈指针，sscratch存用户态
在这之后sp里存用户态栈指针，sscratch存内核态

### 5

sret
这时候sepc会覆盖pc，跳回用户态之前执行的位置

### 6

在这之前sp里存用户态栈指针，sscratch存内核态
在这之后sp里存内核态栈指针，sscratch存用户态

### 7

ecall触发syscall

# 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

   > 无

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

   > xv6的kernel/riscv.h，回顾rv相关的汇编用，只在简答作业参考

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
