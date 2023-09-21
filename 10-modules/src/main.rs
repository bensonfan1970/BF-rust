// This declaration will look for a file named `my.rs` and will
// insert its contents inside a module named `my` under this scope
mod my4;

fn function4() {
    println!("called `function4()`");
}

fn modules_file_hierachy() {
    println!("");
    println!("start modules_file_hierachy");

    my4::function4();

    function4();

    my4::indirect_access4();

    my4::nested4::function4();

    //my4::inaccessible4::public_function4();
    // function `public_function4` is not publicly re-exported

    println!("end modules_file_hierachy");
    println!("");
}

fn function3() {
    println!("called `function3()`");
}

mod cool {
    pub fn function3() {
        println!("called `cool::function3()`");
    }
}

mod my3 {
    fn function3() {
        println!("called `my3::function3()`");
    }
    
    mod cool {
        pub fn function3() {
            println!("called `my3::cool::function3()`");
        }
    }
    
    pub fn indirect_call() {
        // Let's access all the functions named `function3` from this scope!
        print!("called `my3::indirect_call()`, that\n> ");
        
        // The `self` keyword refers to the current module scope - in this case `my3`.
        // Calling `self::function3()` and calling `function3()` directly both give
        // the same result, because they refer to the same function.
        self::function3();
        function3();
        
        // We can also use `self` to access another module inside `my3`:
        self::cool::function3();
        
        // The `super` keyword refers to the parent scope (outside the `my3` module).
        super::function3();
        
        // This will bind to the `cool::function3` in the *crate* scope.
        // In this case the crate scope is the outermost scope.
        {
            use crate::cool::function3 as root_function3;
            root_function3();
        }
    }
}

fn modules_super_and_self() {
    println!("");
    println!("start modules_super_and_self");
    my3::indirect_call();
    println!("end modules_super_and_self");
    println!("");
}

// Bind the `deeply::nested::function` path to `other_function`.
use deeply::nested::function2 as other_function2;

fn function2() {
    println!("called `function2()`");
}

mod deeply {
    pub mod nested {
        pub fn function2() {
            println!("called `deeply::nested::function2()`");
        }
    }
}

fn modules_the_use_decalration_2() {
    println!("");
    println!("start modules_the_use_decalration_2");

    // Easier access to `deeply::nested::function2`
    other_function2();

    println!("Entering block");
    {
        // This is equivalent to `use deeply::nested::function2 as function2`.
        // This `function2()` will shadow the outer one.
        use crate::deeply::nested::function2;

        // `use` bindings have a local scope. In this case, the
        // shadowing of `function2()` is only in this block.
        function2();

        println!("Leaving block");
    }

    function2();

    println!("end modules_the_use_decalration_2");
    println!("");
}

/* doesn't compile
use crate::deeply::nested::{
    my_first_function,
    my_second_function,
    AndATraitType
};

fn modules_the_use_decalration() {
    println!("");
    println!("start modules_the_use_decalration");
    my_first_function();
    println!("end modules_the_use_decalration");
    println!("");
}
*/

mod my {
    // A public struct with a public field of generic type `T`
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // A public struct with a private field of generic type `T`
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // A public constructor method
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox {
                contents: contents,
            }
        }
    }
}

fn modules_struct_visibility() {
    println!("");
    println!("start modules_struct_visibility");

    // Public structs with public fields can be constructed as usual
    let open_box = my::OpenBox { contents: "public information" };

    // and their fields can be normally accessed.
    println!("The open box contains: {}", open_box.contents);

    // Public structs with private fields cannot be constructed using field names.
    // Error! `ClosedBox` has private fields
    //let closed_box = my::ClosedBox { contents: "classified information" };
    // TODO ^ Try uncommenting this line

    // However, structs with private fields can be created using
    // public constructors
    let _closed_box = my::ClosedBox::new("classified information");

    // and the private fields of a public struct cannot be accessed.
    // Error! The `contents` field is private
    //println!("The closed box contains: {}", _closed_box.contents);
    // TODO ^ Try uncommenting this line

    println!("end modules_struct_visibility");
    println!("");
}

// A module named `my_mod`
mod my_mod {
    // Items in modules default to private visibility.
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // Use the `pub` modifier to override default visibility.
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // Items can access other items in the same module,
    // even when private.
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    // Modules can also be nested
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        // Functions declared using `pub(in path)` syntax are only visible
        // within the given path. `path` must be a parent or ancestor module
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n> ");
            public_function_in_nested();
        }

        // Functions declared using `pub(self)` syntax are only visible within
        // the current module, which is the same as leaving them private
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        // Functions declared using `pub(super)` syntax are only visible within
        // the parent module
        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_function_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // pub(crate) makes functions visible only within the current crate
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }

    // Nested modules follow the same rules for visibility
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }

        // Private parent items will still restrict the visibility of a child item,
        // even if it is declared as visible within a bigger scope.
        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("called `my_mod::private_nested::restricted_function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

fn modules_visibility() {
    println!("");
    println!("start modules_visibility");

    // Modules allow disambiguation between items that have the same name.
    function();
    my_mod::function();

    // Public items, including those inside nested modules, can be
    // accessed from outside the parent module.
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) items can be called from anywhere in the same crate
    my_mod::public_function_in_crate();

    // pub(in path) items can only be called from within the module specified
    // Error! function `public_function_in_my_mod` is private
    //my_mod::nested::public_function_in_my_mod();
    // TODO ^ Try uncommenting this line

    // Private items of a module cannot be directly accessed, even if
    // nested in a public module:

    // Error! `private_function` is private
    //my_mod::private_function();
    // TODO ^ Try uncommenting this line

    // Error! `private_function` is private
    //my_mod::nested::private_function();
    // TODO ^ Try uncommenting this line

    // Error! `private_nested` is a private module
    //my_mod::private_nested::function();
    // TODO ^ Try uncommenting this line

    // Error! `private_nested` is a private module
    //my_mod::private_nested::restricted_function();
    // TODO ^ Try uncommenting this line

    println!("end modules_visibility");
    println!("");
}

fn main() {
    modules_visibility();
    modules_struct_visibility();
    //modules_the_use_decalration();
    modules_the_use_decalration_2();
    modules_super_and_self();
    modules_file_hierachy();
}
