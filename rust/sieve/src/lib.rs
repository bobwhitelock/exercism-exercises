
#![feature(step_by)]

pub fn primes_up_to(limit: i32) -> Vec<i32> {
    let potentials_upper_limit = limit + 1;
    let mut potentials = (2..potentials_upper_limit).collect::<Vec<i32>>();
    let mut p_index = 0;
    while p_index < potentials.len() {
        let p = potentials[p_index];
        let to_remove = ((2 * p)..potentials_upper_limit)
            .step_by(p)
            .collect::<Vec<i32>>();
        let mut indices_to_remove = vec![];
        for (i, potential) in potentials.iter().enumerate() {
            if to_remove.contains(potential) {
                indices_to_remove.push(i);
            }
        }
        for &i in indices_to_remove.iter().rev() {
            potentials.remove(i);
        }

        p_index += 1;
    }
    potentials
}
