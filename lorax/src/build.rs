use crate::shape::*;

struct IRBuilder {
    root: Region,
    block_idx: usize,
}

impl IRBuilder {
    fn get_block(&self, idx: usize) -> Option<&Block> {
        self.root.body.get(idx)
    }

    fn get_mut_block(&mut self, idx: usize) -> Option<&mut Block> {
        self.root.body.get_mut(idx)
    }

    fn push_op<T: Into<Operation>>(&mut self, op: T) -> Result<(), ()> {
        if let Some(block) = self.get_mut_block(self.block_idx) {
            block.body.push(op.into());
            Ok(())
        } else {
            Err(())
        }
    }
}
