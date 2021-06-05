


// use std::ops::Add;
// use std::collections::HashMap;

fn main() {
    {
        println!("=================test1==================动态数组");
        let v: Vec<i32> = Vec::new();
        let vv = vec![1, 2, 3];
        println!("{:?}", v);
        println!("{:?}", vv);
        //
        // let mut vx: Vec<i32> = Vec::new();
        //
        // // vx.push(2);   // 不是mut 不能push,添加一个元素
        // // println!("{:?}", vx);
        //
        //  let mut val =  &vx[0];
        //   println!("{:?}", val);
        // let mut val =  vx.get(0);
        //   println!("{:?}", val);
        // vx.push(54);
        // println!("{:?}", vx);
        // let val = &vx[0];
        //
        // vx.push(5);
        // println!("{:?}", val); // 这两行一起会报错，因为插入新的元素原来的数组内存空间不够导致重新分配内存，val的引用就会失效，借用规则会规避
    }
    // {
    //     println!("=================test2==================遍历");
    //     let vv = vec![1, 2, 3];
    //     for i in vv {
    //         println!("{}", i)
    //     }
    //
    //
    //     // for i in &vv { // vv已经被借用并失效
    //     //     println!("{}", i)
    //     // }
    //
    //     let v = vec![1, 2, 3];
    //     for i in &v { // 不可变引用
    //         println!("{}", i)
    //     }
    //
    //     let mut mv = vec![1, 2, 3];
    //     for i in &mut mv { // 可变引用
    //         *i+= 10;
    //         println!("{}", i)
    //     }
    //     println!("{:?}", mv)
    //
    // }
    // {
    //     println!("=================test3==================字符串");
    //     let mut s = String::new();
    //     s.push_str("h");
    //     s.push('e');
    //     println!("{}", s);
    //     let mut s = "hel".to_string();
    //     println!("{}", s);
    //     let mut s2 = "lo".to_string();
    //     let s3 = s+ &s2; // s 失效，s2强制解引用变成了&s2[..]   只能将String和&str想加
    //     println!("{}", s3);
    //
    //     let s4 = "world".to_string();
    //     //  实际+运算符调用的是add
    //     let s5 = s3.add(&s4);
    //     println!("{}", s4);
    //     println!("{}", s5);
    //
    //    let s6  = format!("{}-{}", s4, s5);
    //     println!("{}", s6);
    //
    //     let s7 = "中国".to_string();
    //     println!("{}", s7.len()); // 字符串不能通过所以取值，因为比如中文一个字三字节，索引取值会报错，所以直接编译就不支持
    //     let s8 = &s7[0..3]; // 指定合法的范围切片可以取到
    //     println!("{}", s8);
    //     println!("{:?}", s8);
    // }
    // {
    //     println!("=================test4==================哈希");
    //     let mut h1 = HashMap::new();
    //     h1.insert(String::from("mimi"), 18);
    //     println!("{:?}",h1    );
    //
    // //     拉链
    //     let name = vec![String::from("miller"), String::from("ruoxi")];
    //     let age = vec![18,17];
    //     //  na 必须指定类型, _表示自己计算
    //     let na:HashMap<_,_> =   name.iter().zip(age.iter()).collect();
    //     println!("{:?}",na);
    //
    //     let n = String::from("bob");
    //     let a = 16;
    //     h1.insert(n, a);
    //     println!("{:?}",h1);
    //     // println!("{}",n); // n失效, 持有所有权的类型插入map中会移权
    //     // println!("{}",a);
    //     println!("{:?}",h1.get("bob"));
    //
    //     for (k, v) in &h1{ // h1失效, &h1就不会
    //         println!("{:?}",format!("{}-{}", k, v));
    //     }
    //     let count = h1.entry(String::from("miller")).or_insert(11); // 不存在插入，存在什么也不发生， or_insert会返回一个可变引用值
    //     // println!("{:?}",h1); // 因为有可变引用，可变操作前，不允许借用
    //     *count+=10;
    //     println!("{:?}",h1);
    //
    // }
}