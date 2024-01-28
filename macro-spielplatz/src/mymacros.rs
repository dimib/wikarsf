
macro_rules! create_function {
    ($func_name:ident) => {
        pub fn $func_name() {
            println!("You called {:?}()", stringify!($func_name));
        }
    };
}

create_function!(foo);
create_function!(bar);

#[macro_export]
macro_rules! print_result {
    ($expression:expr) => {
        println!("{:?} = {:?}", stringify!($expression), $expression);
    };
}

#[macro_export]
macro_rules! compare_expressions {
    ($expression1:expr, $expression2:expr) => {
        $expression1 == $expression2
    };
}

#[macro_export]
macro_rules! calculate {
    (eval $e:expr) => {
        {
            let val: usize = $e;
            println!("{} = {}", stringify!{$e}, val);
        }
    };
}

#[macro_export]
macro_rules! vecxx {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}