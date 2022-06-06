use nalgebra::*;
use itertools::Itertools;
use rayon::iter::ParallelBridge;
use rayon::prelude::ParallelIterator;


use crate::Rating;

pub fn validate_weights(rating_groups: &[Vec<Rating<f64>>], weights: Option<Vec<Vec<f64>>>) -> Vec<Vec<f64>> {
    weights.unwrap_or(rating_groups.iter().map(|it| (0..it.len()).map(|_| 1.).collect()).collect())
}

pub fn quality_1vs1(a: &[Rating<f64>], b: &[Rating<f64>], weights: Option<Vec<Vec<f64>>>, beta: f64) ->  f64 {
    quality(&[a.to_vec(),b.to_vec()], weights, beta)
}

pub fn free_for_all(rating_groups : &[Vec<Rating<f64>>], beta: f64) -> f64 {
    let idxs = 0..rating_groups.len();
    let pairs: Vec<f64> = idxs
        .combinations(2)
        .par_bridge()
        .map(|p| quality_1vs1(&rating_groups[p[0]], &rating_groups[p[1]], None, beta) )
        .collect();
    pairs.iter().sum::<f64>()/pairs.len() as f64
}

fn rotate_a_matrix (rating_groups : &[Vec<Rating<f64>>], flatten_weights: &Vec<f64>) -> DMatrix<f64> {
    let mut t = 0;
    let mut r = 0;
    let mut a_matrix = DMatrix::from_element(rating_groups.len()-1, flatten_weights.len(), 0.0);
    for i in 0..rating_groups.len()-1 {
        let ct = t.clone();
        let setter: Vec<usize> = (0..rating_groups[i].len())
            .into_iter()
            .map(|it| {
                let z = ct+it;
                a_matrix[(r,z)] = flatten_weights[z].clone();
                t += 1;
                return z
            })
            .collect();
        let x = setter[setter.len()-1] + 1;
        for d in x..rating_groups[i+1].len() + x {
            a_matrix[(r,d)] = -flatten_weights[d].clone();
        }
        r+=1;
    }
    a_matrix
}

pub fn quality(rating_groups: &[Vec<Rating<f64>>], weights: Option<Vec<Vec<f64>>>, beta: f64) -> f64 {
    let flatten_ratings : Vec<&Rating<f64>> = rating_groups
        .iter()
        .flatten()
        .collect();
    let flatten_weights = validate_weights(&rating_groups, weights)
        .into_iter()
        .flatten()
        .collect_vec();
    let len = flatten_ratings.len();

    let mean_matrix = DMatrix::from_column_slice(len, 1, 
                                                 &flatten_ratings
                                                 .iter()
                                                 .map(|r| r.mu())
                                                 .collect_vec()
    );
    let var_matrix = DMatrix::from_partial_diagonal(len, len,
                                                    &flatten_ratings
                                                    .iter()
                                                    .map(|r| r.sigma().powi(2) )
                                                    .collect_vec()
    );
    let rotated_a_matrix = rotate_a_matrix(&rating_groups, &flatten_weights);
    let a_matrix = rotated_a_matrix.transpose();
    let ata = beta.powi(2) * &rotated_a_matrix * &a_matrix;
    let atsa = &rotated_a_matrix * &var_matrix * &a_matrix;
    let start = mean_matrix.transpose() * &a_matrix;
    let middle = ata.clone() + atsa;
    let inv_middle = middle.clone().try_inverse().expect("middle inverse error");
    let end = rotated_a_matrix.clone() * &mean_matrix;
    let e_arg = (-0.5 * start * inv_middle * end).determinant();
    let s_arg = ata.determinant() / middle.determinant();
    return e_arg.exp() * s_arg.sqrt()
}

