use super::dendrogram::Dendrogram;

/// Implementation of CLINK algorithm for single-linkage clustering.
///
/// A one-liner over-simplication of the SLINK algorithm I would give is that it is dynamic
/// programming on "compressed-pointer" representation.
pub fn clink<T, D>(samples : &[T], mut dissimilarity: D) -> Dendrogram
where
    D: FnMut(&T, &T) -> f64
{
    let mut pis     = vec![0;   samples.len()];
    let mut lambdas = vec![0.0; samples.len()];
    let mut ms      = vec![0.0; samples.len()];

    // First element is special
    pis[0] = 0;
    lambdas[0] = f64::INFINITY;

    for n in 1..samples.len() {
        // Step 1:
        pis[n] = n;
        lambdas[n] = f64::INFINITY;

        // Step 2:
        for i in 0..n {
            ms[i] = dissimilarity(&samples[i], &samples[n]);
        }

        // Step 3:
        for i in 0..n {
            if lambdas[i] < ms[i] {
                ms[pis[i]] = ms[pis[i]].max(ms[i]);
                ms[i] = f64::INFINITY;
            }
        }

        // Step 4:
        let mut a = n-1;

        // Step 5:
        for i in (0..n).rev() {
            if lambdas[i] >= ms[pis[i]] {
                if ms[i] < ms[a] { a = i; }
            } else {
                ms[i] = f64::INFINITY;
            }
        }

        // Step 6:
        let mut b = pis[a];
        let mut c = lambdas[a];
        pis[a] = n;
        lambdas[a] = ms[a];

        // Step 7:
        if a < n-1 {
            while b < n-1 {
                let d = pis[b];
                let e = lambdas[b];

                pis[b] = n;
                lambdas[b] = c;

                b = d;
                c = e;
            }

            if b == n-1 {
                pis[b] = n;
                lambdas[b] = c;
            }
        }

        // Step 8:
        for i in 0..n {
            if pis[pis[i]] == n {
                if lambdas[i] >= lambdas[pis[i]] {
                    pis[i] = n;
                }
            }
        }
    }

    Dendrogram::new(
        lambdas,
        pis,
    )
}

