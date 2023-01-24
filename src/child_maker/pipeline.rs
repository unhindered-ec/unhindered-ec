#[derive(Clone)]
pub struct Pipeline<'a> {
    scorer: &'a (dyn Fn(&[bool]) -> Vec<i64> + Sync),
}

impl<'a> Pipeline<'a> {
    pub fn new(scorer: &'a (dyn Fn(&[bool]) -> Vec<i64> + Sync)) -> Self {
        Self { scorer }
    }
}

// impl<'a, S, R> ChildMaker<Vec<EcIndividual<Bitstring, TestResults<R>>>, S> for Pipeline<'a>
// where
//     S: Selector<Vec<EcIndividual<Bitstring, TestResults<R>>>>,
//     R: Sum + Copy + From<i64>,
// {
//     fn make_child(
//         &self,
//         rng: &mut ThreadRng,
//         population: &Vec<EcIndividual<Bitstring, TestResults<R>>>,
//         selector: &S,
//     ) -> EcIndividual<Bitstring, TestResults<R>> {
//         todo!();
//         // let first_parent = selector.select(rng, population);
//         // let second_parent = selector.select(rng, population);

//         // let genome = first_parent
//         //     .genome()
//         //     .two_point_xo(second_parent.genome(), rng)
//         //     .mutate_one_over_length(rng);
//         // let test_results = (self.scorer)(&genome).into_iter().map(From::from).sum();
//         // EcIndividual::new(genome, test_results)
//     }
// }
