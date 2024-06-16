use math::prelude::*;

fn neighbours<const N: usize>(samples : &[Vector<N>], epsilon : f64, index : usize) -> Vec<usize> {
    let mut result = Vec::new();
    for other_index in 0..samples.len() {
        if other_index != index {
            if (samples[other_index] - samples[index]).squared_length() < epsilon*epsilon {
                result.push(other_index);
            }
        }
    }
    result
}

/// DBSCAN algorithm.
///
/// Return (number of clusters, sample labels).
/// Samples that are classified as noise have label == samples.len().
pub fn dbscan<const N: usize>(samples : &[Vector<N>], epsilon : f64, min_pts : usize) -> (usize, Vec<usize>) {
    let mut labels = vec![samples.len(); samples.len()];
    let mut label_next = 0;
    for index in 0..samples.len() {
        if labels[index] == samples.len() {
            let neighbour_indices = neighbours(samples, epsilon, index);
            if neighbour_indices.len() + 1 >= min_pts {
                // We are a core point
                labels[index] = label_next;

                // Indices that may correspond to a core point.
                let mut pending_indices = Vec::new();

                // Initial set of pending indices.
                for &neighbour_index in &neighbour_indices {
                    if labels[neighbour_index] == samples.len() {
                        labels[neighbour_index] = label_next;
                        pending_indices.push(neighbour_index);
                    }
                }

                // Recursive expansion
                while let Some(pending_index) = pending_indices.pop() {
                    let neighbour_indices = neighbours(samples, epsilon, pending_index);
                    if neighbour_indices.len() + 1 >= min_pts {
                        for &neighbour_index in &neighbour_indices {
                            if labels[neighbour_index] == samples.len() {
                                labels[neighbour_index] = label_next;
                                pending_indices.push(neighbour_index);
                            }
                        }
                    }
                }
            }
            label_next += 1;
        }
    }
    (label_next, labels)
}

