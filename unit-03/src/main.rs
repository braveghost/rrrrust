fn main() {
    // 数字、元组（标量元素）、数组（标量元素）存储在栈上，可变的值都在堆上
    {
        println!("=================栈数据==================");
        let x = 5;
        let y = x;
        println!("x={}", x); // 栈上数据
        println!("y={}", y);  // 栈上数据重新赋值不会变量失效
    }

    {
        println!("=================字符串可变==================");
        let mut s = String::from("hello");
        s.push_str("world");
        println!("{}", s)
    }
    {
        println!("=================堆数据==================");
        let s1 = String::from("hello");
        let s2 = s1;
        // println!("s1={}",s1); // s1 失效了,指针、长度和容量元数据移动到了s2变量，指针、长度和容量存储在栈内，类似于浅拷贝概念，但是原来的变量会失效
        println!("s2={}", s2);
        //    拷贝特性：标量元素数字、布尔、字符char、元组（内部元素都是标量元素）
    }
    {
        //  一个花括号就是一个作用域，所以s1和s2不会和上面的冲突
        println!("=================拷贝==================");
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1={}", s1); // s1指针、长度和容量元数据以及堆上的值都做拷贝，类似于深拷贝概念
        println!("s2={}", s2);
    }

    {
        println!("=================函数作用域==================");
        //     变量传给函数会触发移动或复制，就像赋值语句一样
        level_3_fn_1();
        println!("=================函数、变量所有权==================");
        level_3_fn_2();
        println!("=================引用==================");
        level_3_fn_3();
    }
    {
        println!("=================切片==================");
        level_3_fn_4()

    }
}

fn level_3_fn_1() {
    //  释放
    let _s = String::from("hello");
    level_3_fn_1_1(_s);
    // println!("level_3_fn_1.s={}", s); // s不可用, level_3_fn_1_1(s)执行触发移动，s已失效 因为s为堆数据触发了移动

    let n = 5;
    level_3_fn_1_2(n);
    println!("level_3_fn_1.n={}", n); // n 可用, 因为n是栈内数据直接拷贝的

    // 变量脱离作用域后将释放
    // todo
    // _s进栈、n进栈、n出栈触发drop函数失效、_s出栈失效， 因为_s已触发移动，所以不会有啥事在这发生,_s将在level_3_fn_1_1执行结束后在level_3_fn_1_1内部失效
}

fn level_3_fn_1_1(s: String) {
    println!("level_3_fn_1_1={}", s)
}

fn level_3_fn_1_2(n: i32) {
    println!("level_3_fn_1_2={}", n)
//     n drop，因为是独立的
}

fn level_3_fn_2() {
    let s1 = level_3_fn_2_1();
    let s2 = String::from("hahaha");
    let s3 = level_3_fn_2_2(s2);
    println!("level_3_fn_2.s1={}", s1);
    // println!("level_3_fn_2.s2={}", s2); // s2触发移动已失效
    println!("level_3_fn_2.s3={}", s3)
}

fn level_3_fn_2_1() -> String {
    String::from("hello")
}

fn level_3_fn_2_2(s: String) -> String {
    s
}

fn level_3_fn_3() {
    let s = String::from("hello");  //   不可变引用
     // s.add(&(String::from("hello"))); //不能修改

    let len = level_3_fn_3_1(&s); // 引用不会交付所有权，借用所有权，但是只有使用权没有修改权
    println!("level_3_fn_3.s={}", s);
    println!("level_3_fn_3.len={}", len);
    let mut s2 = String::from("hello");
    let len2 = level_3_fn_3_2(&mut s2);   // 可变引用
    let len3 = level_3_fn_3_2(&mut s2);   // 可变引用
    println!("level_3_fn_3.s2={}", s2);
    println!("level_3_fn_3.len2={}", len2);
    println!("level_3_fn_3.len3={}", len3);
    {
        //  一个作用域只能有一个可变引用（普通引用不受限制，因为是只读）
        let s2_i1 = &mut s2;
        // let s2_i2 = &mut s2;
        println!("s2_i1={}", s2_i1);
        // let s2_i2 = &mut s2;
        // println!("s2_i2={}",s2_i2);
    }
    {
        //  放在这木得问题
        let s2_i2 = &mut s2;
        println!("s2_i2={}", s2_i2);
    }
}

// usize、isize是随系统位数选择的数字类型
fn level_3_fn_3_1(s: &String) -> usize {
    //  s 本身是不可变的类型，引用并且没有所有权 不可以修改s
    println!("level_3_fn_3_1.s={}", s);
    // s.push_str("world");  //不能修改
    s.len() // level_3_fn_3_1不持有s的所有权所以s离开作用域不会释放

}

fn level_3_fn_3_2(s: &mut String) -> usize {
    //  s mut可变所以这里可以改
    println!("level_3_fn_3_2.s={}", s);
    s.push_str("world");
    s.len() // level_3_fn_3_1不持有s的所有权所以s离开作用域不会释放
}

//  todo 编译不过，不能返回一个引用，因为脱离作用域s2本身就失效了了
// fn level_3_fn_3_3() -> &mut String {
//     let mut s2 = String::from("hello");
//     &mut s2
// }

fn level_3_fn_4() {
    let    s = String::from("hello world");
    // let s1 = "hhhh"; // 字符串的字面量就是切片

    let hello1 = &s[0..5];
    let hello2 = &s[..5];
    let world1 = &s[6..10];
    let world2 = &s[6..];

    println!("level_3_fn_4.hello1={}",hello1);  // 字符串切片必须是有效的utf-8边界
    println!("level_3_fn_4.hello2={}",hello2);
    println!("level_3_fn_4.world1={}",world1);
    println!("level_3_fn_4.world2={}",world2);
    // s.clear();  // 清空后下面打开编译时候会报错,清空之前产生了不可变的切片"借用"，所以不允许再清空，可以执行，但是下面的代码无法再调用
    // println!("level_3_fn_4.hello1.1={}",hello1);
    // println!("level_3_fn_4.hello2.1={}",hello2);
    // println!("level_3_fn_4.world1.1={}",world1);
    // println!("level_3_fn_4.world2.1={}",world2);
    println!("level_3_fn_4={}",s);  // 字符串切片必须是有效的utf-8边界

    //  元组不能切片， 数组可以
    let mut li = [1,2];
    let lili = &li[..];
    // li[1] = 4;   // li已被借用 不能修改
    println!("{}",li[1]);
    println!("{}",lili[1]);

}

