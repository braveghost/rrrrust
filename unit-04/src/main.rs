fn main() {
    {
        println!("=================test1==================结构体");
        // let user1 = User{username: String::from("miller"),email:String::from("abc@xxx.com"),active:true}; // 不加mnt是不可变的，值一旦固定就不能修改
        // user1.active = false
        let user1 = new_user(String::from("bob"), String::from("email@ccc.com"));
        let mut user2 = User { username: String::from("miller"), email: String::from("abc@xxx.com"), active: true, age: 10 };
        user2.active = false;


        let user3 = User {
            username: String::from("nini"),
            ..user2
        };

        let user4 = User {
            username: String::from("nini"),
            ..user1
        };
        // println!("user2 username = {}", user2.username);// 报错，失效
        // println!("user2 email = {}", user2.email); // 报错，失效
        println!("user3 username = {}", user3.username);
        println!("user3 email = {}", user3.email);
        println!("user4 username = {}", user4.username);
        println!("user4 email = {}", user4.email);
        let user5 = user4;
        println!("user5 username = {}", user5.username);
        println!("user5 email = {}", user5.email);
        // println!("user4 username = {}", user4.username); // user4的所有权已经转移给user5
        // println!("user4 email = {}", user4.email);
        let mut user6 = User {
            username: String::from("nini"),
            email: String::from("test"),
            active: false,
            age: 10,
        };
        user6.active = true;
        let user7 = user6;
        println!("user7 username = {}", user7.username);
        println!("user7 email = {}", user7.email);
        println!("user7 active = {}", user7.active);
        // user6.active = true; // user6的所有权已经转移给user7
        // println!("user7 active = {}", user7.active);
    }

    {
        println!("=================test2==================元组结构体");
        let ss = Color(1, 2, 3);  // 不可改变
        let mut pp = ss;
        pp.1 = 3;
        let _ = Point(1, 2, 3);  // 不可改变

        level_4_fn_2((1, 3, 54)); // 不能接受Color类型和Point类型
    }
    {
        println!("=================test3==================注解派生");
        let user1 = new_user(String::from("bob"), String::from("email@ccc.com"));
        // println!("{}", user1) // 不能打印，因为没有实现对应Display接口特性
        println!("user1.username = {}", user1.username);
        // println!("{:?}", user1) // 不能打印，因为没有实现对应debug接口特性
        let ss = Color(1, 2, 3);
        println!("{:?}", ss); // 实现了debug特性
    }
    {
        println!("=================test4==================方法和静态绑定方法（关联函数python的staticmethod）");
        let mut user1 = new_user(String::from("bob"), String::from("email@ccc.com"));
        let  user2 = new_user(String::from("nini"), String::from("email@ccc.com"));
        // println!("{}",       user1.get_name());
        println!("{}",       user1.add_age(10));
        user1.set_name(String::from("miller"));
        println!("{}",  user1.username     );
        User::set_name(&mut user1,String::from("miller chai"));
        println!("{}",  user1.username     );

        User::married(user1,user2 ); // user1 和 user2 移动了作用域，都失效了

    }

}

struct User {
    username: String,
    email: String,
    active: bool,
    age: i32,
}

fn new_user(name: String, email: String) -> User {
    User { username: name, email: email, active: true, age: 10 }
}

//  这俩不是一个类型
#[derive(Debug)]
struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

fn level_4_fn_2(_: (i32, i32, i32)) {}


//  impl关联函数
impl User {
    // fn get_name(self) -> String { // 执行这个函数后会转移所有权
    //     self.username
    // }

}
impl User {
    fn add_age(&self, age: i32) -> i32{ // 这里self只是调用
        self.age + age
    }
    fn set_name(&mut self, name: String){ // 可变引用的所有权
        self.username = name
    }
    fn married(_: User,_: User){ // 静态方法

    }
}
