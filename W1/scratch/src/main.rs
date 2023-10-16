fn insert_value<'vec_lifetime>(my_vec: &'vec_lifetime mut Vec<&'vec_lifetime i32>, value: &'vec_lifetime i32) {
    my_vec.push(value)
}
fn main(){
    let mut my_vec = vec![];
    let val1 = 1;
    let val2 = 2;
    
    insert_value(&mut my_vec, &val1);
    insert_value(&mut my_vec, &val2);
    
    println!("{my_vec:?}");
}