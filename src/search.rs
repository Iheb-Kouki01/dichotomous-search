
pub fn search (a:u32 , x: Vec<u32>  ) -> usize {

    let mut v=x.clone();
    v.sort();
    let i=(v.len()-1)/2;
    if ((a > v[i]) && (a < v[i+1]))||(a < v[0])||(a > v[v.len()-1]) {return 0;}  
    else if a == v[i] { return i+1;}
    else {  let t = v.split_off(i+1);
            match a < v[i] {
            true => return search(a, v)+1,
            false => return search(a, t)+i+1,
            };
    };
}