use crate::dendrogram::Dendrogram;

/// Implementation of CLINK algorithm for single-linkage clustering.
///
/// A one-liner over-simplication of the SLINK algorithm I would give is that it is dynamic
/// programming on "compressed-pointer" representation.
pub fn clink<T, D>(samples : &[T], mut dissimilarity: D) -> Dendrogram
where
    D: FnMut(&T, &T) -> f64
{
    let mut merge_heights = vec![0.0; samples.len()];
    let mut merge_targets = vec![0;   samples.len()];
    let mut merge_updates = vec![0.0; samples.len()];
    for n in 0..samples.len() {
        // Compute merge_updates - the lowest level at which i can merge to the right with n.
        {
            // By definition, i can only merge with n when height is at least equal to their
            // dissimilarity.
            for i in 0..n {
                merge_updates[i] = dissimilarity(&samples[i], &samples[n]);
            }

            // Note we have i merge with merge_targets[i] at merge_heights[i] without considering n.
            // Hence, we have the following bounds
            //  - merge_updates[merge_targets[i]] >= merge_updates[i]                or merge_updates[merge_targets[i]] <= merge_heights[i] i.e. merge target can only merge with n either 1: after you               merge with n 2: before you merge with it
            //  - merge_updates[i]                >= merge_updates[merge_targets[i]] or merge_updates[i]                <= merge_heights[i] i.e. you          can only merge with n either 1: after your merge target merge with n 2: before you merge with your merge target
            for _ in 0..2 {
                for i in 0..n {
                    if merge_updates[i] >= merge_heights[i] && merge_updates[merge_targets[i]] >= merge_heights[i] {
                        let result = f64::max(merge_updates[i], merge_updates[merge_targets[i]]);
                        merge_updates[i]                = result;
                        merge_updates[merge_targets[i]] = result;
                    }
                }
            }
        }

        {
            // Initialization
            merge_heights[n] = f64::INFINITY;
            merge_targets[n] = n;

            // We have to merge with n either:
            //  - We can merge with before our original merge target
            //  - Our merge target merge to n before we merge to them.
            for i in (0..n).rev() {
                if merge_heights[i] >= merge_updates[i] || merge_heights[i] >= merge_updates[merge_targets[i]] {
                    merge_heights[i] = merge_updates[i];
                    merge_targets[i] = n;
                }
            }
        }
    }

    Dendrogram::new(
        merge_heights,
        merge_targets,
    )
}

