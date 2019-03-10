use rs_data_structure::bintree::BinTree;

fn display() {
    let seq = "ABC##DE#G##F###";
    let tree = BinTree::from_seq_pre(seq.chars().into_iter(), &'#');
    println!("{}", tree);
}

fn main() {
    display();
}
