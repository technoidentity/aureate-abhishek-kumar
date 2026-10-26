struct Node{
    value: i32,
    left:Option<Box<Node>>,
    right:Option<Box<Node>>,
    }
impl Node{
    fn new(value: i32) -> Self
    {
        Self { value, left: None, right: None }
    }
    fn insert(&mut self, value: i32){
        if value < self.value{
            match self.left.as_mut(){
                Some(node) => node.insert(value),
                None=> {
                    self.left = Some(Box::new(Node::new(value)));
                }
            }
        }else if value > self.value {
            match self.right.as_mut() {
                Some(node) => node.insert(value),
                None => {
                    self.right = Some(Box::new(Node::new(value)));
                }
            }
        }
    }
    fn search(&self, value:i32)->Option<&Node>{
        if value== self.value{
            Some(self)
        }else if value < self.value{
            match self.left.as_ref(){
                Some(node)=>node.search(value), 
                None=>None,
            }
        }else {
            match self.right.as_ref(){
                Some(node)=>node.search(value),
                None=>None,
            }
        }
    }
    
    fn delete(node: Option<Box<Node>>, value: i32) -> Option<Box<Node>> {
        match node {
            None => None,

            Some(mut node) => {
                if value < node.value {
                    node.left = Node::delete(node.left, value);
                } else if value > node.value {
                    node.right = Node::delete(node.right, value);
                } else {
                    if node.left.is_none() {
                        return node.right;
                    }

                    if node.right.is_none() {
                        return node.left;
                    }

                    let mut curr = node.right.as_ref().unwrap();

                    while let Some(left) = curr.left.as_ref() {
                        curr = left;
                    }

                    let min_value = curr.value;
                    node.value = min_value;

                    node.right = Node::delete(node.right, min_value);
                }

                Some(node)
            }
        }
    }

}

fn main() {
    let mut root = Node::new(10);
    root.insert(5);
    root.insert(15);

}