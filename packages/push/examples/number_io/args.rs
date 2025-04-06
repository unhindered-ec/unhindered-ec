use clap::Parser;

#[derive(clap::ValueEnum, Copy, Clone, Debug)]
pub enum RunModel {
    Serial,
    Parallel,
}

/// Simple genetic algorithm in Rust
#[derive(Parser, Debug, Copy, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
    /// Maximum number of generations to run
    #[clap(short = 'g', long, default_value_t = 100)]
    pub max_generations: usize,

    // It's not clear that `e` is awesome here, but it wasn't obvious
    // what was better.
    /// Maximum genome length
    #[clap(short = 'e', long, default_value_t = 100)]
    pub max_genome_length: usize,

    /// Maximum number of initial instructions
    #[clap(short = 'i', long, default_value_t = 1)]
    pub max_initial_instructions: usize,

    /// Population size
    #[clap(short, long, default_value_t = 100)]
    pub population_size: usize,

    /// Should we use parallelism when doing the run?
    #[clap(short = 'm', long, value_enum, default_value_t = RunModel::Parallel)]
    pub run_model: RunModel,

    /// Number of random training cases
    #[clap(short = 't', long, default_value_t = 100)]
    pub num_training_cases: usize,

    /// Lower bound (inclusive) for training case input
    /// Training case inputs will be in the range
    /// `(lower_input_bound..upper_input_bound)`
    #[clap(short = 'l', long, default_value_t = -100, allow_hyphen_values = true)]
    pub lower_input_bound: i64,

    /// Upper bound (exclusive) for training case input
    /// Training case inputs will be in the range
    /// `(lower_input_bound..upper_input_bound)`
    #[clap(short = 'u', long, default_value_t = 100, allow_hyphen_values = true)]
    pub upper_input_bound: i64,

    /// Penalty value to use when a program doesn't
    /// have a value on the expected "return" stack.
    #[clap(short = 'v', long, default_value_t = 1_000_000)]
    pub penalty_value: usize,
}
