use std::time::Instant;

#[derive(Clone, Copy)]
struct DataPoint {
    value: f64,
    timestamp: u64,
    category: u32,
}

struct CachePredictionDemo {
    sequential_data: Vec<DataPoint>,
    random_data: Vec<DataPoint>,
    size: usize,
}

impl CachePredictionDemo {
    fn new(size: usize) -> Self {
        let mut sequential_data: Vec<DataPoint> = (0..size)
            .map(|i| DataPoint {
                value: i as f64,
                timestamp: i as u64,
                category: i as u32 % 10,
            })
            .collect();

        // Create random access pattern
        let mut random_data = sequential_data.clone();
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        // Simple shuffle using hash-based swapping
        for i in 0..size {
            let mut hasher = DefaultHasher::new();
            i.hash(&mut hasher);
            let j = (hasher.finish() as usize) % size;
            random_data.swap(i, j);
        }

        Self {
            sequential_data,
            random_data,
            size,
        }
    }

    // Cache-friendly: Sequential access pattern
    fn process_sequential(&self) -> (f64, std::time::Duration) {
        let start = Instant::now();
        let mut sum = 0.0;
        
        // Sequential access - cache prefetcher can predict next addresses
        for data_point in &self.sequential_data {
            sum += data_point.value;
            // Simulate some computation
            if data_point.category % 2 == 0 {
                sum += data_point.timestamp as f64 * 0.1;
            }
        }
        
        let duration = start.elapsed();
        (sum, duration)
    }

    // Cache-unfriendly: Random access pattern
    fn process_random(&self) -> (f64, std::time::Duration) {
        let start = Instant::now();
        let mut sum = 0.0;
        
        // Random access - cache prefetcher cannot predict
        for data_point in &self.random_data {
            sum += data_point.value;
            // Same computation as sequential version
            if data_point.category % 2 == 0 {
                sum += data_point.timestamp as f64 * 0.1;
            }
        }
        
        let duration = start.elapsed();
        (sum, duration)
    }

    // Matrix multiplication showing cache blocking optimization
    fn matrix_multiply_naive(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let n = a.len();
        let mut c = vec![vec![0.0; n]; n];
        
        // Poor cache locality: accessing B column-wise
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    c[i][j] += a[i][k] * b[k][j]; // b[k][j] = bad cache access
                }
            }
        }
        c
    }

    fn matrix_multiply_cache_friendly(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let n = a.len();
        let mut c = vec![vec![0.0; n]; n];
        
        // Better cache locality: reorder loops to access memory sequentially
        for i in 0..n {
            for k in 0..n {
                for j in 0..n {
                    c[i][j] += a[i][k] * b[k][j]; // Better: a[i][k] stays in register
                }
            }
        }
        c
    }

    fn benchmark_matrix_operations(&self, size: usize) {
        let matrix_a: Vec<Vec<f64>> = (0..size)
            .map(|i| (0..size).map(|j| (i + j) as f64).collect())
            .collect();
        
        let matrix_b: Vec<Vec<f64>> = (0..size)
            .map(|i| (0..size).map(|j| (i * 2 + j) as f64).collect())
            .collect();

        println!("🔄 Matrix Multiplication Benchmark ({}x{}):", size, size);

        let start = Instant::now();
        let _ = Self::matrix_multiply_naive(&matrix_a, &matrix_b);
        let naive_time = start.elapsed();

        let start = Instant::now();
        let _ = Self::matrix_multiply_cache_friendly(&matrix_a, &matrix_b);
        let optimized_time = start.elapsed();

        println!("   Naive approach:    {:?}", naive_time);
        println!("   Cache-optimized:   {:?}", optimized_time);
        println!("   Speedup:           {:.2}x", 
                 naive_time.as_nanos() as f64 / optimized_time.as_nanos() as f64);
    }

    fn run_benchmarks(&self) {
        println!("🧪 Cache Prediction Performance Analysis");
        println!("Dataset size: {} elements ({:.2} MB)", 
                 self.size, 
                 (self.size * std::mem::size_of::<DataPoint>()) as f64 / 1024.0 / 1024.0);
        
        println!("\n📊 Memory Access Pattern Comparison:");
        
        let (seq_result, seq_time) = self.process_sequential();
        let (rand_result, rand_time) = self.process_random();
        
        println!("🔄 Sequential Access (Cache-Friendly):");
        println!("   Result: {:.2}", seq_result);
        println!("   Time:   {:?}", seq_time);
        
        println!("🎲 Random Access (Cache-Unfriendly):");
        println!("   Result: {:.2}", rand_result);
        println!("   Time:   {:?}", rand_time);
        
        let slowdown = rand_time.as_nanos() as f64 / seq_time.as_nanos() as f64;
        println!("📈 Performance Impact:");
        println!("   Random access is {:.2}x slower", slowdown);
        println!("   Cache prediction saves {:.1}% execution time", 
                 (1.0 - 1.0/slowdown) * 100.0);

        // Matrix multiplication demo
        println!("\n");
        self.benchmark_matrix_operations(256);
        
        println!("\n🎯 Key Insights:");
        println!("• Hardware prefetchers predict sequential patterns automatically");
        println!("• Random access patterns defeat cache prediction mechanisms");
        println!("• Algorithm design should consider memory access patterns");
        println!("• Cache-friendly code can be orders of magnitude faster");
    }
}

fn main() {
    // Test with different sizes to see cache effects
    let demo = CachePredictionDemo::new(1_000_000); // ~24MB dataset
    demo.run_benchmarks();
    
    println!("\n💡 Cache Prediction Mechanisms Explained:");
    println!("1. **Stride Prefetching**: Detects sequential access patterns");
    println!("2. **Stream Prefetching**: Identifies multiple memory streams");  
    println!("3. **Temporal Prediction**: Recently accessed → likely to access again");
    println!("4. **Spatial Prediction**: Nearby addresses → likely to access soon");
}