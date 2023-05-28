use ec_core::generator::{CollectionContext, Generator};
use ec_linear::genome::LinearContext;
use rand::rngs::ThreadRng;

use crate::state::push_state::PushInstruction;

#[derive(Debug, PartialEq, Eq)]
pub struct Plushy {
    instructions: Vec<PushInstruction>,
}

// TODO: We might want to implement some sort of `Into`
//   trait instead of just having a getter. Having something
//   like `to_instructions()` since we're cloning?
impl Plushy {
    #[must_use]
    pub fn get_instructions(&self) -> Vec<PushInstruction> {
        self.instructions.clone()
    }
}

impl Generator<Plushy, LinearContext<CollectionContext<PushInstruction>>> for ThreadRng {
    fn generate(&mut self, context: &LinearContext<CollectionContext<PushInstruction>>) -> Plushy {
        let instructions: Vec<PushInstruction> = self.generate(context);
        Plushy { instructions }
    }
}

#[cfg(test)]
mod test {
    use rand::thread_rng;

    use crate::state::push_state::IntInstruction;

    use super::*;

    #[test]
    #[allow(clippy::unwrap_used)]
    fn plush_generator() {
        let instructions = vec![
            PushInstruction::IntInstruction(IntInstruction::Add),
            PushInstruction::IntInstruction(IntInstruction::Subtract),
            PushInstruction::IntInstruction(IntInstruction::Multiply),
            PushInstruction::IntInstruction(IntInstruction::ProtectedDivide),
        ];
        let collect_context = CollectionContext::new(instructions).unwrap();
        let mut rng = thread_rng();
        let plushy: Plushy = rng.generate(&LinearContext {
            length: 10,
            element_context: collect_context,
        });
        assert_eq!(10, plushy.instructions.len());
    }
}
