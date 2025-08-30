use eframe::egui;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

mod ecg_display;
mod edf_parser;
mod qrs_parser;

use ecg_display::EcgDisplay;
use edf_parser::EdfReader;
use qrs_parser::QrsReader;

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 768.0;
const SAMPLE_RATE: f32 = 360.0; // Common ECG sample rate
const DISPLAY_SECONDS: f32 = 10.0; // Show 10 seconds of data
const MAX_SAMPLES: usize = (SAMPLE_RATE * DISPLAY_SECONDS) as usize;

#[derive(Clone)]
pub struct EcgSample {
    pub timestamp: f64,
    pub lead1: f32,
    pub lead2: f32,
    pub lead_v1: f32,
    pub is_qrs: bool,
}

pub struct EcgMonitor {
    samples: Arc<Mutex<VecDeque<EcgSample>>>,
    heart_rate: Arc<Mutex<f32>>,
    is_running: Arc<Mutex<bool>>,
    display_speed: Arc<Mutex<f32>>,
    edf_data: Vec<EcgSample>,
    current_index: usize,
    last_update: Instant,
    display: EcgDisplay,
}

impl Default for EcgMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl EcgMonitor {
    fn new() -> Self {
        let samples = Arc::new(Mutex::new(VecDeque::with_capacity(MAX_SAMPLES)));
        let heart_rate = Arc::new(Mutex::new(75.0));
        let is_running = Arc::new(Mutex::new(true));
        let display_speed = Arc::new(Mutex::new(1.0));

        // Load ECG data
        let edf_data = Self::load_ecg_data();

        let monitor = Self {
            samples: samples.clone(),
            heart_rate: heart_rate.clone(),
            is_running: is_running.clone(),
            display_speed: display_speed.clone(),
            edf_data,
            current_index: 0,
            last_update: Instant::now(),
            display: EcgDisplay::new(),
        };

        // Start data streaming thread
        Self::start_data_thread(samples, heart_rate, is_running, display_speed);

        monitor
    }

    fn load_ecg_data() -> Vec<EcgSample> {
        let mut samples = Vec::new();

        // Try to load EDF file
        if let Ok(mut edf_reader) = EdfReader::new("r01.edf") {
            println!("Successfully loaded EDF file");

            // Load QRS annotations
            let qrs_annotations =
                QrsReader::load_annotations("r01.edf.qrs").unwrap_or_else(|_| Vec::new());

            let signal_data = edf_reader.read_signals().unwrap_or_else(|_| Vec::new());

            for (i, data_point) in signal_data.iter().enumerate() {
                let timestamp = i as f64 / SAMPLE_RATE as f64;
                let is_qrs = qrs_annotations
                    .iter()
                    .any(|&qrs_time| (qrs_time - timestamp).abs() < 0.01);

                samples.push(EcgSample {
                    timestamp,
                    lead1: data_point.get(0).copied().unwrap_or(0.0),
                    lead2: data_point.get(1).copied().unwrap_or(0.0),
                    lead_v1: data_point.get(2).copied().unwrap_or(0.0),
                    is_qrs,
                });
            }
        } else {
            // Fallback: Generate synthetic ECG data
            println!("Could not load EDF file, generating synthetic ECG data");
            samples = Self::generate_synthetic_ecg();
        }

        samples
    }

    fn generate_synthetic_ecg() -> Vec<EcgSample> {
        let mut samples = Vec::new();
        let duration = 60.0; // 60 seconds of data
        let heart_rate = 75.0;
        let rr_interval = 60.0 / heart_rate;

        for i in 0..(duration * SAMPLE_RATE) as usize {
            let t = i as f64 / SAMPLE_RATE as f64;
            let heart_cycle = (t % rr_interval) / rr_interval;

            // Generate realistic ECG waveform
            let lead1 = Self::ecg_waveform(heart_cycle, 1.0);
            let lead2 = Self::ecg_waveform(heart_cycle, 1.2);
            let lead_v1 = Self::ecg_waveform(heart_cycle, 0.8);

            // Add some noise
            let noise = (rand::random::<f32>() - 0.5) * 0.05;

            let is_qrs = heart_cycle > 0.15 && heart_cycle < 0.25;

            samples.push(EcgSample {
                timestamp: t,
                lead1: lead1 + noise,
                lead2: lead2 + noise,
                lead_v1: lead_v1 + noise,
                is_qrs,
            });
        }

        samples
    }

    fn ecg_waveform(t: f64, amplitude: f32) -> f32 {
        // Simplified ECG waveform generation
        let t = t as f32;

        if t < 0.1 {
            // P wave
            (t * 20.0 * std::f32::consts::PI).sin() * 0.2 * amplitude
        } else if t < 0.15 {
            // PR segment
            0.0
        } else if t < 0.18 {
            // Q wave
            -0.3 * amplitude
        } else if t < 0.22 {
            // R wave
            ((t - 0.18) * 50.0 * std::f32::consts::PI).sin() * 2.0 * amplitude
        } else if t < 0.26 {
            // S wave
            -0.5 * amplitude
        } else if t < 0.35 {
            // ST segment
            0.0
        } else if t < 0.55 {
            // T wave
            ((t - 0.35) * 5.0 * std::f32::consts::PI).sin() * 0.4 * amplitude
        } else {
            // Baseline
            0.0
        }
    }

    fn start_data_thread(
        samples: Arc<Mutex<VecDeque<EcgSample>>>,
        _heart_rate: Arc<Mutex<f32>>,
        is_running: Arc<Mutex<bool>>,
        display_speed: Arc<Mutex<f32>>,
    ) {
        thread::spawn(move || {
            let edf_data = Self::load_ecg_data();
            let mut current_index = 0;
            let mut _last_qrs_time = 0.0;
            let _qrs_intervals: VecDeque<f64> = VecDeque::new();

            loop {
                if !*is_running.lock().unwrap() {
                    thread::sleep(Duration::from_millis(100));
                    continue;
                }

                let speed = *display_speed.lock().unwrap();
                let sleep_duration = Duration::from_millis((1000.0 / (SAMPLE_RATE * speed)) as u64);

                if !edf_data.is_empty() {
                    let sample = edf_data[current_index].clone();

                    // Heart rate calculation is now handled in the display module
                    if sample.is_qrs {
                        _last_qrs_time = sample.timestamp;
                    }

                    // Add sample to display queue
                    let mut samples_lock = samples.lock().unwrap();
                    if samples_lock.len() >= MAX_SAMPLES {
                        samples_lock.pop_front();
                    }
                    samples_lock.push_back(sample);
                    drop(samples_lock);

                    current_index = (current_index + 1) % edf_data.len();
                }

                thread::sleep(sleep_duration);
            }
        });
    }
}

impl eframe::App for EcgMonitor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Request continuous repaints for smooth animation
        ctx.request_repaint();

        // Top panel with medical-style controls
        egui::TopBottomPanel::top("controls")
            .default_height(50.0)
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(0, 100, 200)))
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.add_space(20.0);

                    // Play/Pause button
                    let mut is_running = self.is_running.lock().unwrap().clone();
                    let play_button = ui.add(
                        egui::Button::new(
                            egui::RichText::new(if is_running { "⏸ PAUSE" } else { "▶ PLAY" })
                                .size(16.0)
                                .color(egui::Color32::WHITE),
                        )
                        .fill(if is_running {
                            egui::Color32::from_rgb(200, 80, 0)
                        } else {
                            egui::Color32::from_rgb(0, 150, 0)
                        })
                        .min_size(egui::Vec2::new(100.0, 30.0)),
                    );

                    if play_button.clicked() {
                        is_running = !is_running;
                        *self.is_running.lock().unwrap() = is_running;
                    }

                    ui.add_space(30.0);

                    // Speed control
                    ui.label(
                        egui::RichText::new("SPEED:")
                            .size(14.0)
                            .color(egui::Color32::WHITE),
                    );

                    let mut speed = self.display_speed.lock().unwrap().clone();
                    if ui
                        .add(
                            egui::Slider::new(&mut speed, 0.1..=5.0)
                                .suffix("x")
                                .custom_formatter(|n, _| format!("{:.1}x", n))
                                .min_decimals(1)
                                .text_color(egui::Color32::WHITE),
                        )
                        .changed()
                    {
                        *self.display_speed.lock().unwrap() = speed;
                    }

                    ui.add_space(30.0);

                    // Status indicator
                    let status_color = if is_running {
                        egui::Color32::from_rgb(0, 255, 0)
                    } else {
                        egui::Color32::from_rgb(255, 150, 0)
                    };

                    ui.label(
                        egui::RichText::new(if is_running {
                            "● MONITORING"
                        } else {
                            "● PAUSED"
                        })
                        .size(16.0)
                        .color(status_color)
                        .strong(),
                    );

                    // Push time to the right
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(20.0);
                        ui.label(
                            egui::RichText::new(
                                chrono::Local::now().format("%H:%M:%S").to_string(),
                            )
                            .size(16.0)
                            .color(egui::Color32::WHITE)
                            .monospace(),
                        );
                    });
                });
            });

        // Full screen patient monitor display
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::BLACK))
            .show(ctx, |ui| {
                let samples = self.samples.lock().unwrap().clone();
                self.display.draw_ecg(ui, &samples);
            });
    }
}

#[tokio::main]
async fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    // let options = eframe::NativeOptions {
    //     viewport: egui::ViewportBuilder::default()
    //         .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
    //         .with_min_inner_size([800.0, 600.0])
    //         .with_title("ECG Monitor - Raspberry Pi")
    //         .with_icon(eframe::icon_data::from_png_bytes(&[]).unwrap_or_default()),
    //     ..Default::default()
    // };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_min_inner_size([800.0, 600.0])
            .with_title("ECG Monitor - Raspberry Pi")
            .with_icon(eframe::icon_data::from_png_bytes(&[]).unwrap_or_default()),

        // Force GL (OpenGL ES 2.0) instead of WGPU — Raspberry Pi compatible
        renderer: eframe::Renderer::Glow,

        // Disable MSAA to avoid unsupported framebuffer configs
        multisampling: 0,

        // Force vsync to reduce GPU load on Raspberry Pi
        vsync: true,

        ..Default::default()
    };

    eframe::run_native(
        "ECG Monitor",
        options,
        Box::new(|_cc| Box::new(EcgMonitor::new())),
    )
}

// Simple random number generation for synthetic data
mod rand {
    use std::sync::atomic::{AtomicU64, Ordering};

    static SEED: AtomicU64 = AtomicU64::new(1);

    pub fn random<T>() -> T
    where
        T: From<f32>,
    {
        let seed = SEED.fetch_add(1, Ordering::Relaxed);
        let a = 1664525u64;
        let c = 1013904223u64;
        let m = 2u64.pow(32);

        let next = (a.wrapping_mul(seed).wrapping_add(c)) % m;
        let normalized = next as f32 / m as f32;

        T::from(normalized)
    }
}
