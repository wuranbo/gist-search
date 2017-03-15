// #![feature(plugin)]
//
// #![plugin(clippy)]

pub fn main() {
    println!("gist search start.");
    let x = Some(1u8);
    match x {
        Some(y) => println!("{:?}", y),
        _ => ()
    }
}



fn get_default3<'m,K,V:Default>(

    map: &'m mut HashMap<K,V>,

    key: K)

    -> &'m mut V

{

    map.entry(key)

        .or_insert_with(|| V::default())

}