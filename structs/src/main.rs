/*
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

struct Point {
    x: isize,
    y: isize,
    z: isize
}

struct Color (u8, u8, u8);

// r: red
// g: green
// b: blue
fn identify_color(r: u8, g: u8, b: u8) -> String {
    if r == 0 && g == 0 && b == 0 {
        return String::from("black");
    }

    if r == 255 && g == 255 && b == 255 {
        return String::from("white");
    }

    return String::from("unknown");
}

fn build_user(email: String, username: String) -> User {
    return User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}

fn main() {
    let user = build_user(String::from("loa@koa.co"), String::from("lokoa"));
    let new_user = User { username: String::from("goa"), ..user };

    let color = Color(255, 155, 255);
    let color_name = identify_color(color.0, color.1, color.2);

    println!("{color_name}");

    println!("{:?}", new_user);
    println!("{}", user.active);
}
*/

/////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn perimeter(&self) -> u32 {
        return 2 * self.width + self.height;
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        return self.width > rect.width && self.height > rect.width;
    }

    fn double_size(&self) -> Rectangle {
        return Rectangle {
            width: self.width,
            height: self.height
        }
    }

    fn square(size: u32) -> Rectangle {
        return Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // println!("let rect = {:#?}", rect);
    // println!("Area := {}", rect.area());
    // println!("Perimeter := {}", rect.perimeter());
}
