use std::env;
use std::path::PathBuf;
use std::io::{self, Write};
use std::os::raw::c_int;
use libloading::{Library, Symbol};
use jni::{JavaVM, InitArgsBuilder, objects::JValue, JNIVersion};

fn main() {
    let mut input = String::new();

    let a: i32 = loop {
        print!("Enter first number: ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Invalid input. Please enter a valid integer."),
        }
    };

    let b: i32 = loop {
        print!("Enter second number: ");
        io::stdout().flush().unwrap();
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Invalid input. Please enter a valid integer."),
        }
    };

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let c_lib_path = out_dir.join("libc_add.dll");
    let cs_lib_path = out_dir.join("Sub.dll");
    let java_jar_path = out_dir.join("java_mul.jar");

    unsafe {
        let c_lib = Library::new(&c_lib_path).expect("Failed to load C DLL");
        let add: Symbol<unsafe extern "C" fn(c_int, c_int) -> c_int> =
            c_lib.get(b"add").unwrap();
        println!("C (add): {} + {} = {}", a, b, add(a, b));
    }

    unsafe {
        let cs_lib = Library::new(&cs_lib_path).expect("Failed to load C# DLL");
        let sub: Symbol<unsafe extern "C" fn(c_int, c_int) -> c_int> =
            cs_lib.get(b"cs_sub").unwrap();
        println!("C# (sub): {} - {} = {}", a, b, sub(a, b));
    }

    let class_path_option = format!("-Djava.class.path={}", java_jar_path.display());

    let jvm_args = InitArgsBuilder::new()
        .version(JNIVersion::V8)
        .option(&class_path_option)
        .build()
        .unwrap();


    let jvm = JavaVM::new(jvm_args).expect("Failed to create JVM");
    let mut env = jvm.attach_current_thread().unwrap();

    let class = env.find_class("Main").unwrap();

    let result_mul = env
        .call_static_method(class, "mul", "(II)I", &[JValue::from(a), JValue::from(b)])
        .unwrap()
        .i()
        .unwrap();

    println!("Java (mul): {} * {} = {}", a, b, result_mul);
}
