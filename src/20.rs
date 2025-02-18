use std::fmt::{Debug, Display};

fn main() {
    dyn__();
}


fn dyn__() {
    println!("=== dyn__ ===");

    // 动物
    // 所有使用Animal trait 的struct 都需要实现Debug trait
    trait Animal: Debug  {
        fn name(&self) -> String;
        fn say(&self);
        fn run(&self) {
            println!("{} is running...", self.name())
        }
    }

    #[derive(Debug)]
    struct Dog {
        name: String
    }

    #[derive(Debug)]
    struct Cat {
        name: String
    }

    #[derive(Debug)]
    struct Duck {
        name: String
    }

    impl Animal for Dog {
        fn name(&self) -> String {
            self.name.clone()
        }
        fn say(&self) {
            println!("{} say Dog!", self.name)
        }
    }

    impl Dog {
        fn new(name: &str) -> Self {
            Self {
                name: String::from(name)
            }
        }
    }

    impl Animal for Cat {
        fn name(&self) -> String {
            self.name.clone()
        }
        fn say(&self) {
            println!("{} say cat!", self.name)
        }
    }

    impl Cat {
        fn new(name: &str) -> Self {
            Self {
                name: String::from(name)
            }
        }
    }

    impl Animal for Duck {
        fn name(&self) -> String {
            self.name.clone()
        }
        fn say(&self) {
            println!("{} say Duck!", self.name)
        }
    }

    impl Duck {
        fn new(name: &str) -> Self {
            Self {
                name: String::from(name)
            }
        }
    }

    struct Zoo {
        animals: Vec<Box<dyn Animal>>
    }

    // 为Zoo实现自定义打印
    impl Display for Zoo {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
           let mut animals = String::new();
            for animal in self.animals.iter() {
                animals.push_str(&animal.name());
                animals.push_str(", ");
            }
            write!(f, "Zoo: {}", animals)
        }
    }

    impl Zoo {
        fn new() -> Self {
            Zoo {
                animals: Vec::<Box<dyn Animal>>::new()
            }
        }

        fn add_animal(&mut self, animal: Box<dyn Animal>) {
            self.animals.push(animal);
        }

        fn look(&self) {
            println!("{}", self);
        }

        fn run(&self) {
            for animal in self.animals.iter() {
                animal.run();
            }
        }

        fn say(&self) {
            for animal in self.animals.iter() {
                animal.say();
            }
        }

        fn get_animal(&self, index: usize) -> &Box<dyn Animal> {
            self.animals.get(index).unwrap()
        }
    }



    let dog = Dog::new("史努比");
    let cat = Cat::new("哆啦B梦");
    let duck = Duck::new("唐老鸭");

    let mut zoo = Zoo::new();
    zoo.add_animal(Box::new(dog));
    zoo.add_animal(Box::new(cat));
    zoo.add_animal(Box::new(duck));

    zoo.look();
    zoo.run();
    zoo.say();

    let one = zoo.get_animal(0);

    println!("{:?}", one);

    zoo.look();

    zoo.add_animal(Box::new(Cat::new("汤姆")));
    zoo.look();


    // 我们为字符串切片实现了 Animal trait 使他看上去像动物
    impl Animal for &str {
        fn name(&self) -> String {
            self.to_string()
        }

        fn say(&self) {
            println!("我不是动物，我是String: {}", self)
        }
    }


    // 虽然&str不是动物，但是我们为&str 实现了Animal trait
    // 这就是："如果它走起路来像鸭子，叫起来像鸭子，那它就是鸭子"

    let like_animal = Box::new("我不是动物，但是我有动物的特征");
    zoo.add_animal(like_animal);
    zoo.look();
    zoo.run();
    zoo.say();

}