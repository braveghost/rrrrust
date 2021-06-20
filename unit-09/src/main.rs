fn main() {
    {
        println!("======================泛型-函数======================");

        // fn largest<T>(list: &[T]) -> T {  // 类型名称放在<>中
        //     let mut l = list[0];
        //     l
        // }
        // fn minimum<X>(list: &[X]) -> X {
        //     let mut l = list[0];
        //     l
        // }
    }

    {
        println!("======================泛型-结构体======================");

        struct Point<T> {
            // 单泛型
            x: T,
            y: T,
        }
        let p1 = Point { x: 1, y: 2 };
        // let p2 = Point{x:1, y:2.0};  // T只能是相同类型
        println!("{}", p1.x);
        println!("{}", p1.y);


        struct PointPlus<T, U> {
            // 多泛型
            x: T,
            y: U,
        }

        impl<T, U> PointPlus<T, U> {
            fn get_x(&self) -> &T {
                &self.x
            }
            fn mixup<V, W> (self, other: PointPlus<V, W>) -> PointPlus<T, W>{
                PointPlus{
                    x : self.x,
                    y: other.y
                }
            }
        }
        impl<U> PointPlus<U, U> { // p3没有这个方法， p4有 因为普通类型是泛型 泛型不是别的泛型
            fn get_y(&self) -> &U {
                &self.y
            }

        }
        impl PointPlus<f32,f32> {
            fn get_x_y_float(&self) -> (f32,f32) {  // 只有PointPlus<f32,f32>才会有该方法
                ( 1.0 ,2.0)
            }
        }
        let p3 = PointPlus { x: 1, y: 3.1 };

        println!("{}", p3.x);
        println!("{}", p3.y);
        println!("{}", p3.get_x());
        let p4 = PointPlus{x: 1.0 as f32, y : 2.0 as f32};
        println!("{:?}", p4.get_x_y_float());
    }


    {
        println!("======================特征 trait======================");

    }
}
