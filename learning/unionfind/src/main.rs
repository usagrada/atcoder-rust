mod union_find;

fn main() {
    println!("Hello, world!");
    let mut tree = union_find::UnionFind::new(5);
    println!("{:?}", tree);
    tree.unite(0,1);
    tree.unite(2,3);
    println!("{:?}", tree);
}
