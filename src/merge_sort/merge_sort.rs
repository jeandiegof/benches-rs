use {itertools::Itertools, pinscher::Benchable, std::iter};

pub struct MergeSort {
    levels: usize,
    input: Vec<u64>,
}

impl MergeSort {
    const N: usize = 100_000_000;

    pub fn new(levels: usize) -> Self {
        let input = Self::input();

        Self { levels, input }
    }

    fn sort(&mut self) {
        let mut buffer: Vec<u64> = iter::repeat_with(Default::default)
            .take(self.input.len())
            .collect();

        Self::inner_merge_sort((&mut self.input, &mut buffer), self.levels)
    }

    /// pre-condition: we need an even number of levels
    /// and not more than log(n) levels
    fn inner_merge_sort<T: Copy + Ord + Send>(slices: (&mut [T], &mut [T]), levels: usize) {
        if levels == 0 {
            slices.0.sort();
        } else {
            let (input, output) = slices;
            let mid = input.len() / 2;
            let (left_input, right_input) = input.split_at_mut(mid);
            let (left_output, right_output) = output.split_at_mut(mid);
            rayon::join(
                || Self::inner_merge_sort((left_output, left_input), levels - 1),
                || Self::inner_merge_sort((right_output, right_input), levels - 1),
            );
            input
                .iter_mut()
                .zip(left_output.iter().merge(right_output.iter()))
                .for_each(|(input_element, output_element)| *input_element = *output_element)
        }
    }

    fn input() -> Vec<u64> {
        iter::repeat_with(rand::random).take(Self::N).collect()
    }
}

impl Benchable for MergeSort {
    fn name(&self) -> &'static str {
        "MergeSort"
    }

    fn setup(&mut self) {
        self.input = Self::input()
    }

    fn execute(&mut self) {
        self.sort();
        assert!(self.input.windows(2).all(|w| w[0] <= w[1]));
    }

    fn teardown(&mut self) {
        assert!(self.input.windows(2).all(|e| e[0] <= e[1]))
    }
}
