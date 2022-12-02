use std::{iter::zip, str::Chars};

fn main() {
    change_by_fct_1();
    change_by_fct_2();
    dont_change_by_fct_3();
    edit_a_string();
    edit_a_string_2()
}

fn edit_a_string(){
    let mut txt = String::from("Add me something ...");
    print!("txt before: {}.\t", txt);
    change_txt(&mut txt);
    println!("txt after: {}", txt);
}
fn change_txt(txt: &mut String) {
    let txt_chars: Chars = txt.chars();
    let iter = txt_chars.clone().count();
    let mut txt_without_dots = String::from("");
    for (letter, i) in zip(txt_chars, 0..iter) {
        if i > iter-4 {
            break;
        }
        txt_without_dots.push(letter);
    }
    *txt = txt_without_dots;
    txt.push_str("<-- Added !");
}


fn edit_a_string_2(){
    let mut txt = String::from("Add me something ...");
    print!("txt before: {}.\t", txt);
    change_txt(&mut txt);
    println!("txt after: {}", txt);
}
fn change_txt2(txt: &mut String) {
    let len = txt.len();
    txt[..txt.len()-4].to_string().push_str("<-- Added !");
}


fn dont_change_by_fct_3(){
    let int_value:i32 = 32;
    dont_change_val3(int_value);
    println!("int_value: {}", int_value);
}
fn dont_change_val3(mut i:i32){
    let j:i32 = 24;
    i=j;
}


fn change_by_fct_2(){
    let int_value:&mut i32 = &mut 32;
    change_val2(int_value);
    println!("int_value: {}", int_value);
}
fn change_val2(i: &mut i32){
    let j:i32 = 24;
    *i=j;
}


fn change_by_fct_1(){
    let mut int_value:&mut i32 = &mut 32;
    change_val(int_value);
    println!("int_value: {}", int_value);
}
fn change_val(i: &mut i32){
    let mut j:i32 = 24;
    *i=j;
}