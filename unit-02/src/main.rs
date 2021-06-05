fn my_function_1(n: i32) {
    println!("my_function_1.println={}", n)
}

fn my_function_2() -> i32 {
    let x = 19;  // 没有返回值的是语句
    let _ = { x + 1 }; // 有返回值的是表达式 x+1就是表达式
    let c = { 5 }; // 有返回值的是表达式 x+1就是表达式

    let yy = {   // 下划线开头或直接下划线可以忽略不使用的值
        let t = 10;
        t + 1
    }; // t+1 有返回值所以是表达式
    if yy == 10 {
        return yy;
    }
    c      // return c      // 单独一个c即可, 可以省略return, 不能有分号，有分号就成了语句没有返回值
}

// 数字加下划线可读、下划线只是可读而已、可以不加
const MAX_POINTS: u32 = 100_00_0;
const MIN_POINTS: u32 = 10;

fn main() {
    {
        println!("=================常量==================");

        println!("MIN_POINTS {}", MIN_POINTS);
        println!("MAX_POINTS {}", MAX_POINTS);
    }

    {
        println!("=================变量==================");
        let v1 = 1;
        let mut v2 = 3;
        println!("v2.1 is {}", v2);

        // v1 = 3; v1 不可变 不带let关键字无法赋值
        let v1 = v1 * 2; // 变量隐藏赋值
        v2 = 5;
        println!("v1.1 is {}", v1);
        let v1 = "改变v1类型"; // 变量隐藏赋值 可以改变类型
        println!("v1.2 is {}", v1);
        println!("v2.2 is {}", v2);
    }


    {
        println!("=================标量类型==================");
        let x = 100_u32; // 数字指定类型
        println!("x is u32 type {}", x);
        let d1 = 11.11;
        println!("d1 is {}", d1);
        let d2 = 11.1 / 3.0;
        // +-*/% 加减乘除余
        println!("11.1/3.0={}", d2); // todo 精度问题
        let b = true;
        println!("b is bool {}", b);
        let char = '👌'; //char类型是一个unicode标量值
        println!("字符是单引号的不是字符串，字符包括各种语言或emoji表情 {}", char);
    }
    {
        println!("=================标量类型==================");
        // 元组（元素类型可以不一样，不能改变长度） 按下标取值直接加数字
        let tup: (i32, i64) = (5, 50);
        println!("tup-1={}", tup.0);
        println!("tup-2={}", tup.1);
        let (t1, t2) = tup;
        println!("t1={} t2={}", t1, t2);


        // 数组（元素类型一样，不能改变长度） 按下标取值必须有中括号
        let li = [1, 2];
        println!("list-0={}", li[0]);
        println!("list-1={}", li[1]);
        let [l1, l2 ] = li;
        println!("l1={} l2={}", l1, l2);
    }

    {
        println!("=================函数=================");
        my_function_1(10);
        println!("my_function_2.c={}", my_function_2());
    }

    {
        println!("=================控制流=================条件语句");
        // todo 普通 if 语句同 go

        let bb = true;
        // 分支的返回值类型必须一样
        let n = if bb { 5 } else { 6 };
        println!("n return={}", n)
    }
    {
        println!("=================控制流=================循环语句");
        let mut counter = 1;
        loop {
            counter += 1;
            if counter == 10 {
                break;
            }
        }
        let _x = loop {
            counter += 1;
            if counter == 20 {
                break counter;  // break 带返回值 只能在loop中
            }
        };
        println!("loop={}", counter);


        //     while 和 python差不多
        while counter == 20 {
            counter += 1;
        }
        println!("while={}", counter);

        //     for 和 python差不多,但是必须要是iter
        let a = [20, 40];
        for i in a.iter() {
            println!("for={}", i);
        }
    }
}
