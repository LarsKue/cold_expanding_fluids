use itertools::izip;

use crate::utils::approx_equal;

/// O(n^2) Chaining Cluster Algorithm
///
/// Returns clustered objects and unclustered remains
///
/// # Arguments
/// `objects` Objects to cluster
///
/// `distance_func` The distance function to determine closeness of objects
///
/// `epsilon` The chaining radius. Objects in any cluster will never be
///     further apart than this to at least one more object in the cluster
///
/// `min_cluster_size` The minimum size a cluster must have to count it
pub fn cluster_link<T, F>(mut objects: Vec<T>, distance_func: F, epsilon: f64, min_cluster_size: usize) -> (Vec<Vec<T>>, Vec<T>)
    where F: Fn(&T, &T) -> f64
{
    let mut clusters = Vec::new();
    let mut unclustered = Vec::new();

    // keep removing the last object in the vector
    while let Some(object) = objects.pop() {
        // create a new cluster with objects close to the initial object
        let mut cluster: Vec<T> = objects.drain_filter(|other| distance_func(&object, other) < epsilon).collect();

        // we also want to add the object itself to the cluster
        cluster.push(object);

        // keep chaining until the cluster is complete, or no more objects are left to add
        let mut i = 0;
        while i < cluster.len() && !objects.is_empty() {
            // we must collect the objects before adding them to the cluster
            // to avoid invalidating this reference
            let current_object = &cluster[i];
            let mut more_objects: Vec<T> = objects.drain_filter(|other| distance_func(&current_object, other) < epsilon).collect();

            // join the cluster and the additional objects we just chained
            cluster.append(&mut more_objects);
            i += 1;
        }

        // check if we found any objects to cluster with
        if cluster.len() < min_cluster_size {
            // there are not enough objects in the cluster to count it
            unclustered.append(&mut cluster);
        } else {
            // there are enough objects in the cluster, add it to the result
            clusters.push(cluster);
        }

    }

    (clusters, unclustered)
}


use crate::vec3::Vec3;

/// Finds and returns the space a number of vectors reside in
///
/// # Arguments
/// `objects` The vector objects
fn find_space(objects: &Vec<Vec3>) -> [[f64; 2]; 3] {
    let vxmin = objects.iter().fold_first(|v1, v2| if v1.x < v2.x { v1 } else { v2 }).unwrap();
    let vxmax = objects.iter().fold_first(|v1, v2| if v1.x > v2.x { v1 } else { v2 }).unwrap();
    let vymin = objects.iter().fold_first(|v1, v2| if v1.y < v2.y { v1 } else { v2 }).unwrap();
    let vymax = objects.iter().fold_first(|v1, v2| if v1.y > v2.y { v1 } else { v2 }).unwrap();
    let vzmin = objects.iter().fold_first(|v1, v2| if v1.z < v2.z { v1 } else { v2 }).unwrap();
    let vzmax = objects.iter().fold_first(|v1, v2| if v1.z > v2.z { v1 } else { v2 }).unwrap();

    [[vxmin.x, vxmax.x], [vymin.y, vymax.y], [vzmin.z, vzmax.z]]
}


/// Discretizes a single axis coordinate into a grid index
fn find_idx(coordinate: f64, [min, max]: [f64; 2], n: usize) -> usize {
    // edge case: the space is fully flat in at least one dimension
    if approx_equal(min, max) { 0 } else { ((n - 1) as f64 * (coordinate - min) / (max - min)).round() as usize }
}


/// Organizes a number of vectors into discretized grid clusters
///
/// # Arguments
/// `objects` The vector objects
///
/// `space` The space the vectors reside in
///
/// `ns` Number of grid points to use per axis. Order is [x, y, z]
fn grid_objects(mut objects: Vec<Vec3>, space: [[f64; 2]; 3], ns: [usize; 3]) -> Vec<Vec<Vec<Vec<Vec3>>>> {

    let mut grid: Vec<Vec<Vec<Vec<Vec3>>>> = vec![vec![vec![Vec::new(); ns[2]]; ns[1]]; ns[0]];

    while let Some(object) = objects.pop() {
        // discretize object coordinates
        let xidx = find_idx(object.x, [space[0][0], space[0][1]], ns[0]);
        let yidx = find_idx(object.y, [space[1][0], space[1][1]], ns[1]);
        let zidx = find_idx(object.z, [space[2][0], space[2][1]], ns[2]);

        grid[xidx][yidx][zidx].push(object);
    }

    grid
}


/// Returns the [x, y, z] index of the highest density cluster above
/// a certain minimum for a number of gridded vector objects
///
/// # Arguments
/// `objects` Grid clustered vector objects
///
/// `min_density` The minimum density at which to count a grid cluster
fn highest_density_cluster_index(objects: &Vec<Vec<Vec<Vec<Vec3>>>>, min_density: usize) -> Option<[usize; 3]> {
    let mut highest_density: Option<usize> = None;
    let mut highest_density_index: Option<[usize; 3]> = None;

    for x in 0..objects.len() {
        for y in 0..objects[x].len() {
            for z in 0..objects[x][y].len() {
                let density = objects[x][y][z].len();
                // only take clusters with at least the minimum density
                if density >= min_density {
                    match highest_density {
                        None => {
                            // no cluster has been found yet, set hd and hdi
                            highest_density = Some(density);
                            highest_density_index = Some([x, y, z]);
                        }
                        Some(hd) => {
                            // there is already a cluster with hd, check if this one is higher
                            if hd < density {
                                highest_density = Some(density);
                                highest_density_index = Some([x, y, z]);
                            }
                        }
                    }
                }
            }
        }
    }

    highest_density_index
}


/// Find and return neighbors of the cluster at [x, y, z] which have at least the minimum density
///
/// # Arguments
/// `objects` Grid clustered vector objects which to search
///
/// `[x, y, z]` The grid coordinates of the cluster whose neighbors should be searched for
///
/// `min_density` The minimum density at which to count a grid cluster
fn high_density_neighbors(objects: &Vec<Vec<Vec<Vec<Vec3>>>>, [x, y, z]: [usize; 3], min_density: usize) -> Vec<[usize; 3]> {

    let xmin = if x == 0 { 0 } else { x - 1 };
    let xmax = if x == objects.len() - 1 { objects.len() } else { x + 2 };

    let mut result = Vec::new();

    for xidx in xmin..xmax {
        let ymin = if y == 0 { 0 } else { y - 1 };
        let ymax = if y == objects[xidx].len() - 1 { objects[xidx].len() } else { y + 2 };

        for yidx in ymin..ymax {
            let zmin = if z == 0 { 0 } else { z - 1 };
            let zmax = if z == objects[xidx][yidx].len() - 1 { objects[xidx][yidx].len() } else { z + 2 };

            for zidx in zmin..zmax {
                // do not return xyz itself for optimization purposes
                if [x, y, z] != [xidx, yidx, zidx] && objects[xidx][yidx][zidx].len() >= min_density {
                    result.push([xidx, yidx, zidx]);
                }
            }

        }
    }


    result
}


/// Find and return a chain of neighbors and neighbors neighbors of the cluster at [x, y, z] which
/// have at least the minimum density
///
/// # Arguments
/// `objects` Grid clustered vector objects which to search
///
/// `[x, y, z]` The grid coordinates of the cluster whose neighbors should be searched for
///
/// `min_density` The minimum density at which to count a grid cluster
fn chain_high_density_neighbors_iterative(objects: &Vec<Vec<Vec<Vec<Vec3>>>>, [x, y, z]: [usize; 3], min_density: usize) -> Vec<[usize; 3]> {

    let mut result = Vec::new();
    let mut neighbors = high_density_neighbors(objects, [x, y, z], min_density);

    while let Some(neighbor) = neighbors.pop() {
        // unique results only
        if !result.contains(&neighbor) {
            // add neighbors neighbors
            neighbors.append(&mut high_density_neighbors(objects, neighbor, min_density));
            result.push(neighbor);
        }
    }

    result
}


/// O(n) Grid Clustering Algorithm.
///
/// Returns clustered objects and unclustered remains.
///
/// # Arguments
/// `objects` Vector objects which should be clustered.
///
/// `ns` The number of grid points to use per axis. Order is [x, y, z].
///     Higher values improve the clustering quality, but drastically
///     slow the algorithm. If you change these values you may also
///     need to readjust your `min_density` and `min_cluster_size`.
///
/// `min_density` The minimum density at which to count a grid cluster.
///     Higher values speed up the algorithm, but will cluster less objects.
///     Turning this value too high will reduce clustering quality.
///
/// `min_cluster_size` The minimum size a cluster must have to count it.
///     This parameter has no effect if it is lower than `min_density`.
///     Changing this value does not speed up the algorithm. It is for
///     clustering quality only.
///
/// # Remarks
/// This algorithm is designed to be fast, which has the drawback that it requires better
/// knowledge of your data set in order to generate useful results. You may need to fine
/// tune the parameters `ns` and `min_density` in order to properly cluster the data points.
pub fn cluster_grid3(mut objects: Vec<Vec3>, ns: [usize; 3], min_density: usize, min_cluster_size: usize) -> (Vec<Vec<Vec3>>, Vec<Vec3>) {
    let space = find_space(&objects);

    // organize objects into a grid
    let mut objects = grid_objects(objects, space, ns);

    let mut result = Vec::new();
    let mut unclustered = Vec::new();

    while let Some([x, y, z]) = highest_density_cluster_index(&objects, min_density) {

        // get all neighbors to the highest density cluster
        let neighbors = chain_high_density_neighbors_iterative(&objects, [x, y, z], min_density);

        // replace the cluster in objects
        let mut cluster = std::mem::replace(&mut objects[x][y][z], Vec::new());

        // replace all neighbors and add them to the cluster
        for neighbor in neighbors {
            let mut neighbor_cluster = std::mem::replace(&mut objects[neighbor[0]][neighbor[1]][neighbor[2]], Vec::new());
            cluster.append(&mut neighbor_cluster);
        }

        // add the cluster to the result
        if cluster.len() < min_cluster_size {
            unclustered.append(&mut cluster);
        } else {
            result.push(cluster);
        }
    }

    // group all unclustered objects that did not meet density requirements,
    // and add them to the already collected unclustered objects
    // we flatten 3x to go from 4d to 1d
    unclustered.extend(objects.iter().flatten().flatten().flatten());


    (result, unclustered)
}


fn ifind_space(objects: &Vec<Vec<f64>>, ndim: usize) -> Vec<[f64; 2]> {
    let min = objects.iter()
        .fold_first(|o1, o2| (0..ndim).map(|i| o1[i].min(o2[i])).collect())
        .expect("Cannot find space for zero objects.");
    let max= objects.iter()
        .fold_first(|o1, o2| (0..ndim).map(|i| o1[i].max(o2[i])).collect())
        .unwrap();

    min.iter().zip(max.iter()).map(|m1, m2| [m1, m2]).collect()
}

fn igrid_objects(objects: &Vec<Vec<f64>>, space: Vec<[f64; 2]>, ns: &Vec<usize>) -> Vec<Vec<usize>> {
    objects.iter().map(|o| izip!(o.iter(), space.iter(), ns.iter()).map(|(coordinate, [min, max], n)| find_idx(coordinate, [min, max], n))).collect()
}

/// Highest Density Cluster Index
fn hdci(igrid: &Vec<Vec<usize>>, min_density: usize) -> Option<Vec<usize>> {
    let (idx, cluster) = igrid.iter().enumerate().fold_first(|(i, c1), (j, c2)| if c1.len() > c2.len() { (i, c1) } else { (j, c2) }).unwrap();
    if cluster.len() < min_density {
        None
    } else {
        Some(idx)
    }
}

fn _icluster_grid(objects: &Vec<Vec<f64>>, ns: &Vec<usize>, min_density: usize, min_cluster_size: usize, ndim: usize) -> (Vec<Vec<usize>>, Vec<usize>) {
    let space = ifind_space(objects, ndim);

    let igrid = igrid_objects(objects, space, ns);
}


pub fn icluster_grid(objects: &Vec<Vec<f64>>, ns: &Vec<usize>, min_density: usize, min_cluster_size: usize) -> (Vec<Vec<usize>>, Vec<usize>) {
    match objects.first() {
        Some(o) => {
            match o.len() {
                0 => {
                    (Vec::new(), Vec::new())
                },
                ndim => {
                    _icluster_grid(objects, ns, min_density, min_cluster_size, ndim)
                }
            }
        },
        None => {
            (Vec::new(), Vec::new())
        }
    }
}


