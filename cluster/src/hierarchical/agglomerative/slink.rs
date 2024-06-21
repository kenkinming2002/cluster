use super::dendrogram::Dendrogram;

/// Implementation of SLINK algorithm for single-linkage clustering.
///
/// A one-liner over-simplication of the SLINK algorithm I would give is that it is dynamic
/// programming on "compressed-pointer" representation.
pub fn slink<T, D>(samples : &[T], mut dissimilarity: D) -> Dendrogram
where
    D: FnMut(&T, &T) -> f64
{
    let mut merge_heights = vec![0.0; samples.len()];
    let mut merge_targets = vec![0;   samples.len()];
    let mut merge_updates = vec![0.0; samples.len()];
    for n in 0..samples.len() {
        // Compute merge_updates - the lowest level at which i merge to the right with n. Note that
        // our computation is not the same as in original paper, in the sense that our
        // merge_updates is not the same as the mu function found in the original paper.
        {
            // By definition, i merge with n directly when height is equal to their dissimilarity
            // if they have not already been merged.
            for i in 0..n {
                merge_updates[i] = dissimilarity(&samples[i], &samples[n]);
            }

            // Note we have i merge with merge_targets[i] at merge_heights[i] without considering n.
            // Hence, we have the following bounds
            //  - merge_updates[i]                <= max(merge_heights[i], merge_updates[merge_targets[i]])
            //  - merge_updates[merge_targets[i]] <= max(merge_heights[i], merge_updates[i])
            // We have to be kinda careful about the order of update.
            for i in 0..n { merge_updates[merge_targets[i]] = merge_updates[merge_targets[i]].min(f64::max(merge_heights[i], merge_updates[i]               )); }
            for i in 0..n { merge_updates[i]                = merge_updates[i]               .min(f64::max(merge_heights[i], merge_updates[merge_targets[i]])); }
        }

        // Update merge_heights and merge_targets.
        {
            // Initialization
            merge_heights[n] = f64::INFINITY;
            merge_targets[n] = n;

            // Check if i merge to n at a lower height than its original merge target.
            for i in 0..n {
                if merge_heights[i] >= merge_updates[i] {
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

