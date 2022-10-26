pub fn get_arg(x: usize) -> Vec<String>{
    vec![std::env::args().nth(x).expect("Failed when getting argument,")]
}
