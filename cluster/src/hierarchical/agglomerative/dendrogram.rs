use super::disjoint_set::DisjointSet;

use itertools::Itertools;

/// Result of clustering analysis.
///
/// Internally, data is stored as what is known as "compressed-pointer" representation.
#[derive(Debug, Clone)]
pub struct Dendrogram {
    /// Lowest levels at which a particular sample is not longer the last sample in its cluster.
    merge_heights : Vec<f64>,
    /// The new last sample at the lowest levels at which a particular sample is not longer the last sample in its cluster.
    merge_targets : Vec<usize>,
}

/// A section of a dendrogram.
pub struct DendrogramSection {
    pub edges : Vec<(usize, usize)>,
    pub labels : Vec<usize>,
}

impl Dendrogram {
    /// Constructor.
    pub fn new(merge_heights : Vec<f64>, merge_targets : Vec<usize>) -> Self {
        assert_eq!(merge_heights.len(), merge_targets.len());
        Self {
            merge_heights,
            merge_targets,
        }
    }

    /// Visualize dendrogram.
    pub fn svg(&self, margin : f64, xscale : f64) -> DendrogramSvg<'_> {
        DendrogramSvg {
            dendrogram : self,
            margin,
            xscale,
        }
    }

    pub fn len(&self) -> usize {
        self.merge_heights.len()
    }

    /// Return section at ```height```.
    pub fn section_with_height(&self, height : f64) -> DendrogramSection {
        let mut disjoint_set = DisjointSet::new(self.len());
        let mut edges = Vec::new();
        for (index1, index2) in std::iter::zip(&self.merge_heights, &self.merge_targets)
            .enumerate()
            .filter(|(_, (&merge_height, _))| merge_height <= height)
            .map(|(item, (_, &merge_target))| (item, merge_target))
        {
            edges.push((index1, index2));
            disjoint_set.merge(index1, index2);
        }
        let labels = disjoint_set.connceted_component_labels();
        DendrogramSection { edges, labels, }
    }

    /// Return lowest section where number of cluster is ```cluster_count```.
    pub fn section_with_cluster_count(&self, cluster_count : usize) -> DendrogramSection {
        let mut disjoint_set = DisjointSet::new(self.len());
        let mut edges = Vec::new();
        for (index1, index2) in std::iter::zip(&self.merge_heights, &self.merge_targets)
            .enumerate()
            .sorted_by(|(_, (merge_height1, _)), (_, (merge_height2, _))| f64::partial_cmp(merge_height1, merge_height2).unwrap())
            .map(|(item, (_, &merge_target))| (item, merge_target))
        {
            edges.push((index1, index2));
            disjoint_set.merge(index1, index2);
            if disjoint_set.connected_component_count() <= cluster_count {
                break
            }
        }
        let labels = disjoint_set.connceted_component_labels();
        DendrogramSection { edges, labels, }
    }
}

/// Svg representation of dendrogram.
pub struct DendrogramSvg<'a> {
    dendrogram : &'a Dendrogram,
    margin : f64,
    xscale : f64,
}

impl std::fmt::Display for DendrogramSvg<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width  = self.dendrogram.merge_heights.len() as f64 * self.xscale;
        let height = self.dendrogram.merge_heights[..self.dendrogram.merge_heights.len()-1].iter().copied().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();

        writeln!(f, "<svg width=\"100%\" height=\"100%\" viewBox=\"{x} {y} {width} {height}\">", x=-self.margin, y=-self.margin, width=width+self.margin, height = height+self.margin)?;
        writeln!(f, "<rect fill=\"#ffffff\" stroke=\"#ffffff\" x=\"{x}\" y=\"{y}\" width=\"{width}\" height=\"{height}\"/>", x=-self.margin, y=-self.margin, width=width+self.margin, height = height+self.margin)?;

        for (item, (&merge_height, &merge_target)) in std::iter::zip(&self.dendrogram.merge_heights, &self.dendrogram.merge_targets).enumerate() {
            let x1 = item         as f64 * self.xscale;
            let x2 = merge_target as f64 * self.xscale;

            let y1 = height - 0.0;
            let y2 = height - merge_height.is_finite().then_some(merge_height).unwrap_or(height);

            writeln!(f, "<polyline points=\"{x1},{y1} {x1},{y2} {x2},{y2}\" fill=\"none\" stroke=\"black\"/>")?;
        }

        writeln!(f, "</svg>")?;
        Ok(())
    }
}


