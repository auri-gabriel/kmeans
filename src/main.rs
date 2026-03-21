use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use rand::prelude::*;

const NUM_CLUSTERS: usize = 3;
const NUM_POINTS: usize = 3000;

fn calculate_distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    (x1 - x2).hypot(y1 - y2)
}

fn main() {
    // Arquivos
    let input_file = File::open("./data.dat").expect("Erro ao abrir input");
    let reader = BufReader::new(input_file);

    let mut cluster_files = vec![
        File::create("cluster1.dat").unwrap(),
        File::create("cluster2.dat").unwrap(),
        File::create("cluster3.dat").unwrap(),
    ];

    let mut centroids_file = File::create("centroids.dat").unwrap();

    // Dados
    let mut x_coords = vec![0.0f32; NUM_POINTS];
    let mut y_coords = vec![0.0f32; NUM_POINTS];
    let mut cluster_assignments = vec![0usize; NUM_POINTS];

    let mut centroid_x = vec![0.0f32; NUM_CLUSTERS];
    let mut centroid_y = vec![0.0f32; NUM_CLUSTERS];
    let mut cluster_sizes = vec![0usize; NUM_CLUSTERS];

    let mut rng = rand::rng();

    // Leitura do arquivo
    for (i, line) in reader.lines().enumerate().take(NUM_POINTS) {
        let line = line.unwrap();
        let parts: Vec<f32> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        x_coords[i] = parts[0];
        y_coords[i] = parts[1];

        let cluster = rng.random_range(0..NUM_CLUSTERS);
        cluster_assignments[i] = cluster;
        cluster_sizes[cluster] += 1;
    }

    let mut has_changes = true;

    while has_changes {
        has_changes = false;

        // Reset cluster sizes
        cluster_sizes.fill(0);

        for &c in &cluster_assignments {
            cluster_sizes[c] += 1;
        }

        // Calcular centróides
        for i in 0..NUM_CLUSTERS {
            centroid_x[i] = 0.0;
            centroid_y[i] = 0.0;

            for j in 0..NUM_POINTS {
                if cluster_assignments[j] == i {
                    centroid_x[i] += x_coords[j];
                    centroid_y[i] += y_coords[j];
                }
            }

            if cluster_sizes[i] > 0 {
                centroid_x[i] /= cluster_sizes[i] as f32;
                centroid_y[i] /= cluster_sizes[i] as f32;
            }
        }

        // Reatribuição
        for j in 0..NUM_POINTS {
            let mut current_cluster = cluster_assignments[j];
            let mut current_distance = calculate_distance(
                x_coords[j],
                y_coords[j],
                centroid_x[current_cluster],
                centroid_y[current_cluster],
            );

            for i in 0..NUM_CLUSTERS {
                let new_distance = calculate_distance(
                    x_coords[j],
                    y_coords[j],
                    centroid_x[i],
                    centroid_y[i],
                );

                if new_distance < current_distance {
                    current_distance = new_distance;
                    current_cluster = i;
                    has_changes = true;
                }
            }

            cluster_assignments[j] = current_cluster;
        }
    }

    let mut cluster_elements = vec![0usize; NUM_CLUSTERS];

    // Centróides finais
    for i in 0..NUM_CLUSTERS {
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;

        for j in 0..NUM_POINTS {
            if cluster_assignments[j] == i {
                sum_x += x_coords[j];
                sum_y += y_coords[j];
                cluster_elements[i] += 1;
            }
        }

        if cluster_elements[i] > 0 {
            centroid_x[i] = sum_x / cluster_elements[i] as f32;
            centroid_y[i] = sum_y / cluster_elements[i] as f32;
        }
    }

    // Output
    // clusters
    for j in 0..NUM_POINTS {
        let c = cluster_assignments[j];
        writeln!(
            cluster_files[c],
            "{} {}",
            x_coords[j],
            y_coords[j]
        ).unwrap();
    }

    // centróides
    for i in 0..NUM_CLUSTERS {
        writeln!(
            centroids_file,
            "{} {}",
            centroid_x[i],
            centroid_y[i]
        ).unwrap();
    }
}
