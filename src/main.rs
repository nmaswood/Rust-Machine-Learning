extern crate itertools;
use itertools::Zip;


fn same_dim(v1 : Vec<f64>, v2: Vec<f64>) -> bool {

    v1.len() ==  v2.len()
}

#[test]
fn test_same_dim(){

    let v1 = vec!(1.0,2.0,3.0);
    let v2 = vec!(3.0,2.0,1.0);

    let v3 = vec!(1.0,2.0,3.0,4.0);

    assert_eq!(true, same_dim(v1.clone(),v2.clone()));
    assert_eq!(false, same_dim(v1,v3));
}

fn vector_add(v1 : Vec<f64>, v2 : Vec<f64>) -> Vec<f64>{

    let mut v3: Vec<f64> = Vec::new();

    for (x,y) in Zip::new((v1.into_iter(), v2.into_iter())) {
        v3.push(x + y);
    }

    v3
}

#[test]
fn test_vector_add(){

    let v1 = vec!(1.0,2.0,3.0);
    let v3 = vec!(2.0,4.0,6.0);

    assert_eq!(true, vector_add(v1.clone(),v1.clone()) == v3);
}

fn vector_sub(v1 : Vec<f64>, v2 : Vec<f64>) -> Vec<f64>{

    let mut v3: Vec<f64> = Vec::new();

    for (x,y) in Zip::new((v1.into_iter(), v2.into_iter())) {
        v3.push(x -  y);
    }

    v3
}

#[test]
fn test_vector_sub(){

    let v1 = vec!(3.0,3.0,3.0);
    let v2 = vec!(1.0,1.0,1.0);

    let v3 = vec!(2.0,2.0,2.0);

    assert_eq!(true, vector_sub(v1,v2) == v3);
}


fn vector_mul(a: f64, v1 : Vec<f64>) -> Vec<f64>{

    v1.into_iter().map(|x| x * a).collect()
}

#[test]
fn test_vector_mul(){

    let v1 = vec!(1.0,1.0,1.0);
    let v2 = vec!(2.0,2.0,2.0);

    assert_eq!(true, vector_mul(2.0,v1) == v2);
}


fn vector_sum(vectors: Vec<Vec<f64>>) -> Vec<f64> {

    let mut first_vector = vectors[0].clone();

    for i in vectors.into_iter().skip(1){
       first_vector = vector_add(first_vector, i);
    }

    first_vector
}


fn vector_mean(v : Vec <f64> ) -> Vec <f64> {

    vector_mul(1 / v.len() as f64, v)
}

fn dot(v1: Vec<f64>, v2: Vec<f64>) -> f64 {

    let mut acc = 0.0;
    for (i , j) in Zip::new((v1.iter(), v2.iter())) {
        acc +=  i * j;
    }

    acc
}

fn sum_of_squares(v : Vec<f64>) -> f64{

    dot(v.clone(), v.clone())
}

fn magnitude(v : Vec<f64>) -> f64{

    sum_of_squares(v).sqrt() as f64
}

fn shape(v: Vec<Vec<f64>>) -> (u64,u64) {
    // rows, cols

    (v.len() as u64, v[0].len() as u64)
}

fn main() {

}
