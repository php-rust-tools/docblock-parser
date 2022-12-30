#[derive(Debug)]
pub struct Document {
    pub nodes: Vec<Node>,
}

impl Document {
    pub fn collect(&self) -> Vec<Node> {
        self.nodes.to_vec()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    ParamTag {
        tag: String,
        _type: Option<Type>,
        variable: Variable,
        // description: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: String,
    pub reference: bool,
    pub variadic: bool,
}
