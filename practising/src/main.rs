fn main() {
    // Exs with refs, borrowing and strings
    slices();
    slices_and_refs();
    
    // Exs with dereferencing
    refs();

    // Vec::<i32> vct = Vec::<i32>(1);

}


/* 
 * https://doc.rust-lang.org/book/ch04-03-slices.html
 */
fn slices_and_refs() {
    let mut txt_str:String = String::from("");
    extract("Hola WOrld !!!", &mut txt_str, (0, 2));
    println!("txt_str: {}", txt_str);
}

fn extract(txt_in: &str, txt_out: &mut String, indexes:(usize, usize)) -> usize {
    // txt_out = &txt_in[indexes.0..indexes.1].to_string();
    txt_in[indexes.0..indexes.1].clone_into(txt_out);
    return txt_out.chars().count();
}

fn slices() {
    let s = String::from("hello world !");
    let s1 = &s[..5];
    let s2 = &s[6..11];
    let s3 = &s2[..].chars().as_str();
    println!("s1: {}, s2: {}, s3: {}", s1, s2, s3);
}



fn fct_for_array(){
    print_hello_world();

    let mut nb1 = 10;
    log("nb1 is equal to: {nb1}");
    // let nb2  = 20;

    for _i in 0..10 {
        nb1 = add(nb1, 1);    
        log(nb1.to_string().as_str());
    }

    log("An array: ");

    let mut array = create_array(99);

    for _el in array {
        log("this is an element of the array: {el}");
    }
}

fn log(txt: &str){
    println!("logged: {txt}");
}

fn print_hello_world(){
    println!("Hello world !");
}

fn add(nb1:i32, nb2:i32) -> i32
{
    nb1 + nb2
}

fn create_array(init_value: i32) -> [i32; 32] {
    const LEN: usize = 32;
    let new_array = [init_value; LEN];
    return new_array;
}

fn dereference() {
    let mut txtToUnref: &mut String = &mut String::from("Hello I'm a ref !; ");

    // Use the ref like an object straightly calling tis methods
    txtToUnref.push_str("holaaaaaaa; ");
    txtToUnref.push_str(String::from(txtToUnref.as_str()).as_str());

    // Use the ref by directly changing its value deferencing itself 
    *txtToUnref = String::from(format!("; {} {}", txtToUnref.as_str(), "Really fun ;)"));
    
    // print the result
    println!("txtToUnref: {}", txtToUnref);
}

fn dereference2(txt_returned: &mut String) -> String {
    let mut txt_to_unref = String::from("Hello I'm a ref !; ");

    // Use the ref like an object straightly calling tis methods
    txt_to_unref.push_str("holaaaaaaa; ");
    txt_to_unref.push_str("_and_");
    txt_to_unref.push_str(String::from(txt_to_unref.as_str()).as_str());

    // Use the ref by directly changing its value deferencing itself 
    txt_to_unref = String::from(format!("; {} {}", txt_to_unref.as_str(), "Really fun ;)"));
    
    // print the result
    println!("txtToUnref: {}", txt_to_unref);
    *txt_returned = txt_to_unref.clone();

    txt_to_unref
}

fn refs(){
    dereference();
    let mut sentence2 = String::from("");
    let sentence = dereference2(&mut sentence2);
    println!("sentence = {}",sentence);
    println!("sentence2 = {}",sentence2);
}