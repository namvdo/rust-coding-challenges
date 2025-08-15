use std::time::Instant;
use rand::prelude::*;

#[derive(Debug, Clone)]
struct Product {
    price: f64,
    in_stock: bool,
    is_premium: bool,
}

struct EcommerceProcessor {
    predictable_products: Vec<Product>,
    random_products: Vec<Product>,
}

impl EcommerceProcessor {
    fn new(size: usize) -> Self {
        let mut rng = thread_rng();
        
        // Create random products
        let mut random_products: Vec<Product> = (0..size)
            .map(|_| Product {
                price: rng.gen_range(10.0..500.0),
                in_stock: rng.gen_bool(0.7),  // 70% in stock
                is_premium: rng.gen_bool(0.3), // 30% premium
            })
            .collect();
        
        // Create predictable version: sort by price (low to high)
        let mut predictable_products = random_products.clone();
        predictable_products.sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
        
        // Shuffle the random version
        random_products.shuffle(&mut rng);
        
        Self {
            predictable_products,
            random_products,
        }
    }
    
    // Process products with PREDICTABLE branch patterns
    fn process_predictable_data(&self) -> f64 {
        let mut total_revenue = 0.0;
        
        // Since data is sorted by price, branches are very predictable:
        // - Early products: price < 100 (mostly true)
        // - Middle products: 100 <= price < 300 (mostly true) 
        // - Late products: price >= 300 (mostly true)
        for product in &self.predictable_products {
            if product.price < 100.0 {  // PREDICTABLE: mostly true early on
                if product.in_stock {
                    total_revenue += product.price;
                    if product.is_premium {
                        total_revenue += 5.0; // premium bonus
                    }
                }
            } else if product.price < 300.0 {  // PREDICTABLE: mostly true in middle
                if product.in_stock {
                    total_revenue += product.price * 1.1; // markup
                    if product.is_premium {
                        total_revenue += 15.0;
                    }
                }
            } else {  // PREDICTABLE: mostly true at end
                if product.in_stock {
                    total_revenue += product.price * 1.2; // higher markup
                    if product.is_premium {
                        total_revenue += 25.0;
                    }
                }
            }
        }
        
        total_revenue
    }
    
    // Process products with UNPREDICTABLE branch patterns
    fn process_unpredictable_data(&self) -> f64 {
        let mut total_revenue = 0.0;
        
        // Same exact logic, but data is shuffled randomly
        // Branches are now unpredictable - each condition could be true/false
        for product in &self.random_products {
            if product.price < 100.0 {  // UNPREDICTABLE: random true/false
                if product.in_stock {
                    total_revenue += product.price;
                    if product.is_premium {
                        total_revenue += 5.0;
                    }
                }
            } else if product.price < 300.0 {  // UNPREDICTABLE: random true/false
                if product.in_stock {
                    total_revenue += product.price * 1.1;
                    if product.is_premium {
                        total_revenue += 15.0;
                    }
                }
            } else {  // UNPREDICTABLE: random true/false
                if product.in_stock {
                    total_revenue += product.price * 1.2;
                    if product.is_premium {
                        total_revenue += 25.0;
                    }
                }
            }
        }
        
        total_revenue
    }
}

fn main() {
    const PRODUCT_COUNT: usize = 100_000;
    const ITERATIONS: usize = 100_000;
    
    println!("Branch Prediction Demo: E-commerce Order Processing");
    println!("==================================================");
    println!("Processing {} products, {} times each", PRODUCT_COUNT, ITERATIONS);
    println!();
    
    // Setup
    print!("Setting up test data... ");
    let processor = EcommerceProcessor::new(PRODUCT_COUNT);
    println!("Done!");
    println!();
    
         // Test 1: Predictable branches (sorted data)
     println!("🎯 Test 1: Predictable Branch Pattern");
     println!("   Data: Products sorted by price (low → high)");
     println!("   Branch behavior: Price checks are predictable");
     let start = Instant::now();
     
     let predictable_revenue = processor.process_predictable_data();
     for _ in 0..ITERATIONS {
         let _ = processor.process_predictable_data();
     }
     
     let predictable_time = start.elapsed();
     println!("   Revenue: ${:.2}", predictable_revenue);
     println!("   Result: {:?} total", predictable_time);
     println!("   Average: {:.2}μs per iteration", 
              predictable_time.as_micros() as f64 / ITERATIONS as f64);
     println!();
     
     // Test 2: Unpredictable branches (random data)
     println!("🎲 Test 2: Unpredictable Branch Pattern");
     println!("   Data: Same products, randomly shuffled");
     println!("   Branch behavior: Price checks are unpredictable");
     let start = Instant::now();
     
     let unpredictable_revenue = processor.process_unpredictable_data();
     for _ in 0..ITERATIONS {
         let _ = processor.process_unpredictable_data();
     }
     
     let unpredictable_time = start.elapsed();
     println!("   Revenue: ${:.2}", unpredictable_revenue);
     println!("   Result: {:?} total", unpredictable_time);
     println!("   Average: {:.2}μs per iteration", 
              unpredictable_time.as_micros() as f64 / ITERATIONS as f64);
    println!();
    
    // Analysis
    let predictable_ms = predictable_time.as_nanos() as f64 / 1_000_000.0;
    let unpredictable_ms = unpredictable_time.as_nanos() as f64 / 1_000_000.0;
    let slowdown = unpredictable_ms / predictable_ms;
    let penalty_percent = (slowdown - 1.0) * 100.0;
    
    println!("📊 Performance Analysis");
    println!("=======================");
    println!("Predictable data:   {:.1}ms", predictable_ms);
    println!("Unpredictable data: {:.1}ms", unpredictable_ms);
    println!("Slowdown factor:    {:.2}x", slowdown);
    println!("Performance penalty: {:.1}%", penalty_percent);
    println!();
    
    println!("💡 What This Demonstrates");
    println!("=========================");
    if penalty_percent > 5.0 {
        println!("✓ Clear branch misprediction penalty observed!");
        println!("  The CPU's branch predictor struggles with random data");
        println!("  Same algorithm + same data = different performance");
    } else {
        println!("⚠ Small difference - try increasing PRODUCT_COUNT or ITERATIONS");
    }
    println!();
    
    println!("🏪 Real-World E-commerce Impact");
    println!("===============================");
    let orders_per_sec_predictable = (ITERATIONS as f64 / predictable_time.as_secs_f64()) as u64;
    let orders_per_sec_unpredictable = (ITERATIONS as f64 / unpredictable_time.as_secs_f64()) as u64;
    
    println!("With predictable customer data:   {} orders/second", orders_per_sec_predictable);
    println!("With unpredictable customer data: {} orders/second", orders_per_sec_unpredictable);
    
    if orders_per_sec_predictable > orders_per_sec_unpredictable {
        let lost_capacity = orders_per_sec_predictable - orders_per_sec_unpredictable;
        println!("Lost processing capacity:         {} orders/second", lost_capacity);
        println!();
        println!("💰 Business Impact:");
        println!("   If each order = $50 average:");
        println!("   Lost revenue potential = ${}/second", lost_capacity * 50);
        println!("   That's ${}/hour from branch mispredictions!", lost_capacity * 50 * 3600);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_same_results_different_performance() {
        let processor = EcommerceProcessor::new(1000);
        
        // Both methods should produce the same total (same data, different order)
        let predictable_result = processor.process_predictable_data();
        let unpredictable_result = processor.process_unpredictable_data();
        
        // Results should be very close (allowing for floating point precision)
        assert!((predictable_result - unpredictable_result).abs() < 0.01, 
                "Results should be the same regardless of data order");
    }
    
    #[test]
    fn test_performance_difference() {
        let processor = EcommerceProcessor::new(5000);
        let iterations = 100;
        
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = processor.process_predictable_data();
        }
        let predictable_time = start.elapsed();
        
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = processor.process_unpredictable_data();
        }
        let unpredictable_time = start.elapsed();
        
        println!("Predictable: {:?}", predictable_time);
        println!("Unpredictable: {:?}", unpredictable_time);
        
        // We expect some performance difference due to branch prediction
        // Even a 5% difference is significant and measurable
        let ratio = unpredictable_time.as_nanos() as f64 / predictable_time.as_nanos() as f64;
        println!("Performance ratio: {:.3}", ratio);
        
        assert!(ratio > 1.01, "Expected measurable performance difference due to branch misprediction");
    }
}