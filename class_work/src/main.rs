fn generics_method_definitions() {
    struct File<T> {
        name: String,
        data: T,
    }

    impl<T> File<T> {
        fn new(name: &str, content: T) -> File<T> {
            File { name: String::from(name), data: content }
        }
    }

    let textfile = File::new("lets'go", vec!["K'Maro".to_string()]);
    let imagefile = File::new("MonaLisa", vec![0, 123, 255]);

    println!("Textfile name {:?}. Textfile content {:?}", textfile.name, textfile.data);
    println!("Imagefile name {:?}. Imagefile content {:?}", imagefile.name, imagefile.data);
}

fn main(){
    generics_method_definitions();
}

