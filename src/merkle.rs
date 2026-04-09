use crate::crypto::double_hash;

#[derive(Clone)]
pub struct TreeNode {
    pub hash_node: [u8; 32],
    pub left_node: Option<Box<TreeNode>>,
    pub right_node: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn build(txids: &mut Vec<[u8; 32]>) -> TreeNode {
        if txids.len() % 2 != 0 {
            let last = txids[txids.len() - 1];
            txids.push(last);
        }

        let mut nodes: Vec<TreeNode> = vec![];

        for i in (0..txids.len()).step_by(2) {
            let combined = [txids[i], txids[i + 1]].concat();
            nodes.push(TreeNode {
                hash_node: double_hash(&combined),
                left_node: None,
                right_node: None,
            });
        }

        loop {
            if nodes.len() <= 1 {
                break;
            }

            if nodes.len() % 2 != 0 {
                let last = nodes[nodes.len() - 1].clone();
                nodes.push(last);
            }

            let mut next_level: Vec<TreeNode> = Vec::new();

            for i in (0..nodes.len()).step_by(2) {
                let left = nodes[i].clone();
                let right = nodes[i + 1].clone();

                let combined = [left.hash_node, right.hash_node].concat();

                next_level.push(TreeNode {
                    hash_node: double_hash(&combined),
                    left_node: Some(Box::new(left)),
                    right_node: Some(Box::new(right)),
                });
            }

            nodes = next_level;
        }

        nodes.pop().unwrap()
    }
}
