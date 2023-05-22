
#[derive(Debug)]
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Node{ data: val, next: None }
    }
}

fn insert_at_end(root: &Node, new_val: i32) {
    let mut p = root.clone();

    loop {
        match p.next {
            Some(node) => {
                // print!("{} ->", p.data);
                match node {
                    // Some(
                }
                // p = node;
            },
            None => {
                p.next = Some(Box::new(Node::new(new_val)));
                // print!("{}", p.data);
                break;
            },
        }
    }

}

fn print_list(root: &Node) {
    let mut p = root.clone();

    loop {
        match &p.next {
            Some(node) => {
                print!("{} ->", p.data);
                p = node;
            },
            None => {
                print!("{}", p.data);
                break;
            },
        }
    }
}

fn main() {
    let mut root = Node{ data: 23, next: None };
    let mut first_node = Node{ data: 1000, next: None };
    let second_node = Node {data: 2000, next: None };

    root.next = Some(Box::new(first_node));
    first_node.next = Some(Box::new(second_node));


    // println!("{:?}", root);

    print_list(&root);
}
