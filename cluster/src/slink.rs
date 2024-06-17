use crate::dendrogram::Dendrogram;

/// Implementation of SLINK algorithm for single-linkage clustering.
///
/// A one-liner over-simplication of the SLINK algorithm I would give is that it is dynamic
/// programming on "compressed-pointer" representation.
pub fn slink<T, D>(samples : &[T], mut dissimilarity: D) -> Dendrogram
where
    D: FnMut(&T, &T) -> f64
{
    let mut merge_heights = vec![0.0f64; samples.len()];
    let mut merge_targets = vec![0usize; samples.len()];
    let mut merge_updates = vec![0.0f64; samples.len()];
    for n in 0..samples.len() {
        // 1: Compute merge_updates - an estimation on the lowest level at which i merge to the
        //    right with n.
        {
            // By definition, i merge with n directly when height is equal to their dissimilarity
            // if they have not already been merged.
            for i in 0..n {
                merge_updates[i] = dissimilarity(&samples[i], &samples[n]);
            }

            // 1: We merge to the right with merge_targets[i] at merge_heights[i].
            // 2: We merge to the right with n                at merge_updates[i].
            // Therefore, merge_targets[i] must have merge to the right with n at max(merge_heights[i], merge_updates[i]).
            for i in 0..n {
                merge_updates[merge_targets[i]] = merge_updates[merge_targets[i]].min(f64::max(
                    merge_heights[i],
                    merge_updates[i],
                ));
            }
        }

        // 2: Update merge_heights and merge_targets.
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

            // We have merge_targets[i] merge to something before i merge to merge_targets[i]. The
            // only possibility of that something is n. When i merge to merge_targets[i], the
            // rightmost sample is no longer merge_targets[i] but n instead.
            for i in 0..n {
                if merge_heights[i] >= merge_heights[merge_targets[i]] {
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

