fn main() {
    println!("Hello, world!");
    /*
    Function Generics
    */
    let numbers = vec![1, 2, 3, 4, 5];
    
    let x = max(&numbers);
    println!("{:?}", x);

    /*
    Struct Generics 
     */

    let point1 = Point{ x: 10, y: 3.2, z: 0 };
    let point2 = Point{ x: -21.2, y: 13.2, z: 3};
    let point3 = Point{ x: -1.5, y: 10, z: 3};

    println!("x ->{}", point2.x());
    println!("new point -> {:?}", point3.mixup(point1));

    let p1 = point2;
    let p2= Point {x : "Hello", y: 'c', z : 0};
    let p3 = p1.mixup(p2);
    println!("mix ups -> {:?}", p3);
}


// fn max(vector : &Vec<i32>) -> i32{
//     let mut max  = 0;
//     for i in vector {
//         if max < *i {
//             max = *i;
//         }
//     }
//     max
// }

fn max<T: PartialOrd + Copy>(vector : &Vec<T>) -> T{
    let mut max  = &vector[0];
    for i in vector {
        if *max < *i {
            max = i;
        }
    }
    *max
}

#[derive(Debug)]
struct Point<T,U>{
    x: T,
    y: U,
    z: u64,
}

impl <V,W> Point<V, W>{
    fn x(&self) -> &V{
        &self.x
    }

    fn mixup<X,Y>(self, other: Point<X,Y>) -> Point<V, Y>{
        Point{
            x : self.x,
            y :other.y,
            z : other.z
        }
    }
}

impl <X> Point<X, f64>{
    fn y(&self) -> &f64{
        &self.y
    }
}

