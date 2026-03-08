pub struct Stats {
    pub sent: u64,
    pub received: u64,
    pub rtts: Vec<f64>,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            sent: 0,
            received: 0,
            rtts: Vec::new(),
        }
    }

    pub fn add_sent(&mut self) {
        self.sent += 1;
    }

    pub fn add_received(&mut self, rtt: f64) {
        self.received += 1;
        self.rtts.push(rtt);
    }

    pub fn loss_percent(&self) -> f64 {
        if self.sent == 0 {
            return 0.0;
        }
        ((self.sent - self.received) as f64 / self.sent as f64) * 100.0
    }

    pub fn min(&self) -> f64 {
        self.rtts.iter().cloned().reduce(f64::min).unwrap_or(0.0)
    }

    pub fn max(&self) -> f64 {
        self.rtts.iter().cloned().reduce(f64::max).unwrap_or(0.0)
    }

    pub fn avg(&self) -> f64 {
        if self.rtts.is_empty() {
            return 0.0;
        }
        self.rtts.iter().sum::<f64>() / self.rtts.len() as f64
    }

    pub fn print_summary(&self, destination: &str) {
        println!("\n--- {} ping statistics ---", destination);
        println!(
            "{} packets transmitted, {} received, {:.0}% packet loss",
            self.sent,
            self.received,
            self.loss_percent()
        );
        if !self.rtts.is_empty() {
            println!(
                "rtt min/avg/max = {:.3}/{:.3}/{:.3} ms",
                self.min(),
                self.avg(),
                self.max()
            );
        }
    }
}
