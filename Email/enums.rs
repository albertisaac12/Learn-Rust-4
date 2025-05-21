enum IpAddr {
    v4,
    v6
}

struct home {
    userId:u16,
    ip_type: IpAddr 
}

fn main() {
    let four = IpAddr::v4;
    let six = IpAddr::v6;

    let home1 = home {
        userId:1,
        ip_type: IpAddr::v4
    };

    let home2 = home::getHomeV6(16);

    match home2.ip_type {
        IpAddr::v4 => println!("is v4"),
        IpAddr::v6 => println!("is v6")
    }

    let mc = Some(test2::d(1,1.1,1));
    let mc = Some(test2::c{x:1,y:1});

    match mc {
        Some(test2::d(a,b,c)) => {
            println!("{:#?}",a);
        },
        // Some(t) => {
        //     println!("{:#?}",t);
        // }
        Some(test2::c{x:a,y:b}) => {
            println!("{} {}",a,b);

        },
        None => println!("meow"),
        _=> println!("idk")
    }
    /* //////////////////////////////////////////////////////////////
                            IF-LET & IF-LET-ELSE
        ////////////////////////////////////////////////////////////// */
        
    let mc = Some(test2::c { x: (1), y: (0) });
    
    if let Some(n) = mc {
        println!("{:#?}",n);
    }
    
    let mc = Some(test2::a);
    
    if let Some(test2::c { x:a,y:c}) = mc {
        println!("{} {}",a,c);
    } else {
        println!("Not d");
    }



}

impl home {
    fn getHomeV6(userID:u16) -> Self {
        home {
            userId: userID,
            ip_type:IpAddr::v6
        }
    }
}


// enum can take any type inside it
#[derive(Debug)]
enum test2 {
    a,
    b(i32),
    c {x:i32,y:i32},
    d(i32,f32,u32)
}


    






















