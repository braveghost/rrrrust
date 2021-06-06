
fn main() {
    {
        println!("=================test1==================枚举");
        let v4 = IpAddrKind::V4;
        println!("{:?}", v4);
        println!("{:?}", IpAddrKind::V6);

        let v6ip = IpAddr::V6(String::from("::1"));
        let v4ip = IpAddr::V4(127, 0, 0, 1);
        println!("{:?}", v4ip);
        println!("{:?}", v6ip);
        let msg = Message::Size { length: 1, size: 2 };
        let _ = Message::Type;
        let _ = Message::To(String::from("miller"));
        msg.call()
    }

    {
        println!("=================test2==================Option");
        let some_number = Some(5);
        let some_string = Some(String::from("some string"));
        let null: Option<i32> = None;
        println!("{:?}", some_number);
        println!("{:?}", some_string);
        println!("{:?}", null);
        // let x : i16 = 5;
        // let y :Option<i16> = Some(6);
        // let sum = x+y; // 不能相加，这是不同的类型，todo 补充官方文档
        // println!("Sum = {}", sum)
    }
    {
        println!("=================test3==================match");
        let v4 = IpAddrKind::V4;

        match v4 {
            IpAddrKind::V4 => {
                println!("haha v4")
            }
            IpAddrKind::V6 => println!("haha v6")
        }
        let i = 20;
        match i == 2 {
            true => println!("true"),
            false => println!("false"),
        }

        let c1 = Coin::Quarter(UsState::Alabama);
        let c2 = Coin::Quarter(UsState::Alaska);
        println!("c1 = {}", value_in_cents(c1));
        println!("c2 = {}", value_in_cents(c2));
        println!("Penny = {}", value_in_cents(Coin::Penny));
        println!("Penny = {}", value_in_cents(Coin::Nickel));
        println!("Penny = {}", value_in_cents(Coin::Dime));


        let y :Option<i16> = Some(6);
        let q=  match y {
            None=>None,
            Some(i)=>Some(i+10), // i 绑定了y包含的6
        };
        println!("Sum = {:?}", q);

        let x = 10;
        let c =  match x {
            1=>false,
            10=>true,
            _ =>false,  // 除了1和10其他的情况
        };
        println!("c = {:?}", c);
        let ff =  match x {
            1=>println!("not 1"),
            10=>println!("not 10"),
            _ =>(),  // 返回(), 必要的非穷尽格式
            _ => {},  // 返回()
        };
        println!("ff = {:?}", ff);

        if let q = Some(16){   // 可以替代match和_的写法
            println!("q={:?}", q)
        }else if let   q = Some(1)  {
            println!("q={:?}", q)

        }else {
            println!("q={:?}", q)

        }

    }
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Type,
    To(String),
    Size { length: i32, size: i32 },  // 匿名结构体
}

impl Message {
    fn call(&self) {}
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(c: Coin) -> u32 {
    match c {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            match state {
                UsState::Alaska => {
                    println!("Alaska");
                    25
                }
                UsState::Alabama => {
                    println!("Alabama");
                    25
                }
            }
        }
    }
}
