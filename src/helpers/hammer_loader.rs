use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

pub struct HammerLoader {
    is_running: Arc<AtomicBool>,
}

impl HammerLoader {
    pub fn new() -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub async fn execute_with_loading<F, T, E>(
        &self,
        future: F,
        message: &str,
    ) -> Result<T, E>
    where
        F: std::future::Future<Output = Result<T, E>>,
    {
        let is_running = Arc::clone(&self.is_running);
        is_running.store(true, Ordering::Relaxed);

        // Start the loading animation
        let loading_task = {
            let is_running = Arc::clone(&is_running);
            let message = message.to_string();
            tokio::spawn(async move {
                Self::show_hammer_loading(is_running, &message).await;
            })
        };

        // Execute the main task
        let result = future.await;

        // Stop the loading animation
        self.is_running.store(false, Ordering::Relaxed);
        loading_task.abort();

        // Clear the loading line
        print!("\r{}\r", " ".repeat(80));
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        result
    }

    async fn show_hammer_loading(is_running: Arc<AtomicBool>, message: &str) {
        let hammer_frames = ["🔨", "⚒️", "🔨", "⚒️"];
        let sparks = ["✨", "💥", "⭐", "🌟"];
        let mut frame_index = 0;
        let mut spark_index = 0;

        while is_running.load(Ordering::Relaxed) {
            let hammer = hammer_frames[frame_index % hammer_frames.len()];
            let spark = sparks[spark_index % sparks.len()];
            
            print!(
                "\r{} {} Smashing through the code... {} {}",
                hammer, spark, message, spark
            );
            std::io::Write::flush(&mut std::io::stdout()).unwrap();

            frame_index += 1;
            spark_index += 1;

            sleep(Duration::from_millis(300)).await;
        }
    }
}