use crate::EcgSample;
use eframe::egui;
use std::collections::VecDeque;

pub struct EcgDisplay {
    grid_color: egui::Color32,
    background_color: egui::Color32,
    ecg_color: egui::Color32,
    spo2_color: egui::Color32,
    nibp_color: egui::Color32,
    resp_color: egui::Color32,
    text_color: egui::Color32,
    vital_bg_color: egui::Color32,
    panel_bg_color: egui::Color32,
    header_bg_color: egui::Color32,
    last_qrs_times: Vec<f64>,
    last_update_time: f64,
}

impl EcgDisplay {
    pub fn new() -> Self {
        Self {
            grid_color: egui::Color32::from_rgb(0, 100, 0),
            background_color: egui::Color32::from_rgb(0, 0, 0),
            ecg_color: egui::Color32::from_rgb(0, 255, 0),
            spo2_color: egui::Color32::from_rgb(0, 200, 255),
            nibp_color: egui::Color32::from_rgb(255, 255, 255),
            resp_color: egui::Color32::from_rgb(255, 255, 0),
            text_color: egui::Color32::from_rgb(255, 255, 255),
            vital_bg_color: egui::Color32::from_rgb(20, 20, 20),
            panel_bg_color: egui::Color32::from_rgb(10, 10, 10),
            header_bg_color: egui::Color32::from_rgb(0, 100, 200),
            last_qrs_times: Vec::new(),
            last_update_time: 0.0,
        }
    }

    fn calculate_heart_rate(&mut self, samples: &VecDeque<crate::EcgSample>) -> i32 {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();

        // Find QRS complexes in recent samples
        for sample in samples.iter().rev().take(100) {
            if sample.is_qrs {
                let qrs_time = current_time
                    - (samples.len() as f64
                        - samples
                            .iter()
                            .position(|s| s.timestamp == sample.timestamp)
                            .unwrap_or(0) as f64)
                        / 360.0;

                // Only add if not already recorded
                if !self
                    .last_qrs_times
                    .iter()
                    .any(|&t| (t - qrs_time).abs() < 0.1)
                {
                    self.last_qrs_times.push(qrs_time);
                }
            }
        }

        // Keep only recent QRS times (last 10 seconds)
        self.last_qrs_times.retain(|&t| current_time - t < 10.0);

        // Calculate heart rate from intervals
        if self.last_qrs_times.len() >= 2 {
            self.last_qrs_times
                .sort_by(|a, b| a.partial_cmp(b).unwrap());
            let intervals: Vec<f64> = self
                .last_qrs_times
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect();

            if !intervals.is_empty() {
                let avg_interval = intervals.iter().sum::<f64>() / intervals.len() as f64;
                let bpm = 60.0 / avg_interval;
                return bpm.round() as i32;
            }
        }

        // Default heart rate with realistic variation based on breathing
        let breathing_effect = (current_time * 0.3).sin() * 2.0; // Respiratory sinus arrhythmia
        let random_variation = ((current_time * 7.3).sin() * (current_time * 11.7).cos()) * 1.5;
        (76.0 + breathing_effect + random_variation)
            .max(65.0)
            .min(90.0) as i32
    }

    fn calculate_spo2(&self) -> i32 {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();

        // Simulate realistic SpO2 variation with breathing correlation
        let base_spo2 = 97.5;
        let breathing_effect = (current_time * 0.3).sin() * 0.5; // Small breathing correlation
        let measurement_noise = ((current_time * 13.2).sin() * (current_time * 7.8).cos()) * 0.8;
        (base_spo2 + breathing_effect + measurement_noise)
            .max(94.0)
            .min(100.0)
            .round() as i32
    }

    fn calculate_blood_pressure(&self) -> (i32, i32, i32) {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();

        // Simulate realistic BP variation with physiological patterns
        let systolic_base = 118.0;
        let diastolic_base = 78.0;

        // Slow circadian-like variation
        let slow_trend = (current_time * 0.001).sin() * 4.0;
        // Breathing-related variation
        let breathing_sys = (current_time * 0.3).sin() * 3.0;
        let breathing_dia = (current_time * 0.3).sin() * 2.0;
        // Random measurement variation
        let sys_noise = ((current_time * 5.7).sin() * (current_time * 9.1).cos()) * 5.0;
        let dia_noise = ((current_time * 6.3).sin() * (current_time * 8.9).cos()) * 3.0;

        let systolic = (systolic_base + slow_trend + breathing_sys + sys_noise)
            .max(100.0)
            .min(140.0) as i32;
        let diastolic = (diastolic_base + slow_trend * 0.5 + breathing_dia + dia_noise)
            .max(60.0)
            .min(90.0) as i32;
        let mean_pressure = ((systolic + 2 * diastolic) / 3) as i32;

        (systolic, diastolic, mean_pressure)
    }

    fn calculate_respiratory_rate(&self) -> i32 {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();

        // Simulate realistic respiratory rate with natural variation
        let base_rate = 17.5;
        let slow_variation = (current_time * 0.008).sin() * 1.5; // Slow drift
        let activity_effect = (current_time * 0.05).cos() * 0.8; // Activity-like variation
        (base_rate + slow_variation + activity_effect)
            .max(12.0)
            .min(22.0)
            .round() as i32
    }

    fn calculate_temperature(&self) -> (f32, f32, f32, f32) {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();

        // Simulate realistic temperature readings with sensor-specific patterns
        // Core temperature (T1) - most stable
        let t1_base = 37.0;
        let t1_variation = (current_time * 0.003).sin() * 0.2 + (current_time * 0.0008).cos() * 0.1;

        // Peripheral temperature (T2) - slightly more variable
        let t2_base = 36.8;
        let t2_variation = (current_time * 0.005).sin() * 0.3 + (current_time * 0.012).cos() * 0.15;

        // Skin temperature (T3) - most variable
        let t3_base = 36.5;
        let t3_variation = (current_time * 0.008).sin() * 0.5 + (current_time * 0.015).cos() * 0.2;

        // Temperature difference (T4) - core-peripheral gradient
        let t4_base = 0.2;
        let t4_variation = (current_time * 0.01).sin() * 0.15;

        (
            ((t1_base + t1_variation).max(36.5).min(37.8)) as f32,
            ((t2_base + t2_variation).max(36.2).min(37.3)) as f32,
            ((t3_base + t3_variation).max(35.8).min(37.2)) as f32,
            ((t4_base + t4_variation).max(0.0).min(0.8)) as f32,
        )
    }

    pub fn draw_ecg(&mut self, ui: &mut egui::Ui, samples: &VecDeque<EcgSample>) {
        let available_rect = ui.available_rect_before_wrap();
        let response = ui.allocate_rect(available_rect, egui::Sense::hover());
        let painter = ui.painter();

        // Draw black background
        painter.rect_filled(response.rect, egui::Rounding::ZERO, self.background_color);

        // Draw header bar
        self.draw_header_bar(&painter, &response.rect);

        // Calculate layout areas
        let header_height = 35.0;
        let footer_height = 30.0;
        let main_rect = egui::Rect::from_min_max(
            response.rect.min + egui::Vec2::new(0.0, header_height),
            response.rect.max - egui::Vec2::new(0.0, footer_height),
        );

        // Left side: ECG waveforms (70% width) - more space for waveforms
        let waveform_width = main_rect.width() * 0.7;
        let waveform_rect = egui::Rect::from_min_size(
            main_rect.min,
            egui::Vec2::new(waveform_width, main_rect.height()),
        );

        // Right side: Vital signs panels (30% width) - only HR and SpO2
        let vitals_width = main_rect.width() * 0.3;
        let vitals_rect = egui::Rect::from_min_size(
            main_rect.min + egui::Vec2::new(waveform_width, 0.0),
            egui::Vec2::new(vitals_width, main_rect.height()),
        );

        // Draw ECG waveforms section
        self.draw_waveform_section(&painter, &waveform_rect, samples);

        // Draw vital signs panels (HR and SpO2 only)
        self.draw_vitals_section(&painter, &vitals_rect, samples);
    }

    fn draw_header_bar(&self, painter: &egui::Painter, rect: &egui::Rect) {
        let header_rect = egui::Rect::from_min_size(rect.min, egui::Vec2::new(rect.width(), 35.0));

        painter.rect_filled(header_rect, egui::Rounding::ZERO, self.header_bg_color);

        // Patient info
        painter.text(
            header_rect.min + egui::Vec2::new(10.0, 8.0),
            egui::Align2::LEFT_TOP,
            "BED NO: 01    PATL: DEMO PATIENT    01-01-2024",
            egui::FontId::proportional(14.0),
            egui::Color32::WHITE,
        );

        // ECG label in top right
        painter.rect_filled(
            egui::Rect::from_min_size(
                header_rect.max - egui::Vec2::new(60.0, 25.0),
                egui::Vec2::new(50.0, 20.0),
            ),
            egui::Rounding::same(3.0),
            egui::Color32::from_rgb(0, 150, 0),
        );
        painter.text(
            header_rect.max - egui::Vec2::new(35.0, 15.0),
            egui::Align2::CENTER_CENTER,
            "ECG",
            egui::FontId::proportional(12.0),
            egui::Color32::BLACK,
        );
    }

    fn draw_waveform_section(
        &mut self,
        painter: &egui::Painter,
        rect: &egui::Rect,
        samples: &VecDeque<EcgSample>,
    ) {
        // Background for waveform area
        painter.rect_filled(*rect, egui::Rounding::ZERO, self.panel_bg_color);

        // Draw grid
        self.draw_medical_grid(painter, rect);

        if samples.is_empty() {
            return;
        }

        // ECG leads section (top 70%)
        let ecg_height = rect.height() * 0.7;
        let ecg_rect =
            egui::Rect::from_min_size(rect.min, egui::Vec2::new(rect.width(), ecg_height));

        // Each ECG lead gets half of ECG section (only 2 leads now)
        let lead_height = ecg_height / 2.0;
        let ecg_leads = [
            egui::Rect::from_min_size(
                ecg_rect.min + egui::Vec2::new(60.0, 20.0),
                egui::Vec2::new(ecg_rect.width() - 80.0, lead_height - 30.0),
            ),
            egui::Rect::from_min_size(
                ecg_rect.min + egui::Vec2::new(60.0, lead_height + 20.0),
                egui::Vec2::new(ecg_rect.width() - 80.0, lead_height - 30.0),
            ),
        ];

        // Draw ECG lead labels
        self.draw_ecg_label(painter, &ecg_leads[0], "II");
        self.draw_ecg_label(painter, &ecg_leads[1], "V1");

        // Draw ECG waveforms
        self.draw_realistic_ecg(painter, &ecg_leads[0], samples, |s| s.lead2);
        self.draw_realistic_ecg(painter, &ecg_leads[1], samples, |s| s.lead_v1);

        // PLETH section only (bottom 30%)
        let pleth_height = rect.height() * 0.3;
        let pleth_rect = egui::Rect::from_min_size(
            rect.min + egui::Vec2::new(60.0, ecg_height + 20.0),
            egui::Vec2::new(rect.width() - 80.0, pleth_height - 30.0),
        );

        // Draw PLETH label and waveform
        self.draw_pleth_label(painter, &pleth_rect, "PLETH");
        self.draw_pleth_waveform(painter, &pleth_rect, samples);
    }

    fn draw_vitals_section(
        &mut self,
        painter: &egui::Painter,
        rect: &egui::Rect,
        samples: &VecDeque<EcgSample>,
    ) {
        // Background
        painter.rect_filled(*rect, egui::Rounding::ZERO, self.vital_bg_color);

        let panel_height = rect.height() / 2.0; // Only 2 panels now
        let margin = 10.0;

        // Heart Rate Panel (ECG) - Top half
        let hr_rect = egui::Rect::from_min_size(
            rect.min + egui::Vec2::new(margin, margin),
            egui::Vec2::new(rect.width() - 2.0 * margin, panel_height - margin * 2.0),
        );
        self.draw_hr_panel(painter, &hr_rect, samples);

        // SpO2 Panel - Bottom half
        let spo2_rect = egui::Rect::from_min_size(
            rect.min + egui::Vec2::new(margin, panel_height + margin),
            egui::Vec2::new(rect.width() - 2.0 * margin, panel_height - margin * 2.0),
        );
        self.draw_spo2_panel(painter, &spo2_rect);
    }

    fn draw_hr_panel(
        &mut self,
        painter: &egui::Painter,
        rect: &egui::Rect,
        samples: &VecDeque<EcgSample>,
    ) {
        // Dark background
        painter.rect_filled(
            *rect,
            egui::Rounding::same(5.0),
            egui::Color32::from_rgb(5, 5, 5),
        );

        // ECG label
        painter.text(
            rect.min + egui::Vec2::new(10.0, 10.0),
            egui::Align2::LEFT_TOP,
            "ECG",
            egui::FontId::proportional(16.0),
            self.ecg_color,
        );

        // Calculate real heart rate
        let heart_rate = self.calculate_heart_rate(samples);

        painter.text(
            rect.center() + egui::Vec2::new(0.0, 10.0),
            egui::Align2::CENTER_CENTER,
            &format!("{}", heart_rate),
            egui::FontId::proportional(72.0),
            self.ecg_color,
        );

        // Heart rate bar indicator - dynamic based on HR
        let bar_rect = egui::Rect::from_min_size(
            rect.max - egui::Vec2::new(20.0, rect.height() - 20.0),
            egui::Vec2::new(10.0, rect.height() - 40.0),
        );

        // Calculate segments to fill based on heart rate (normal range 60-100)
        let segments = 8;
        let hr_normalized =
            ((heart_rate - 50).max(0).min(50) as f32 / 50.0 * segments as f32) as usize;
        let segments_to_fill = hr_normalized.min(segments);

        let segment_height = bar_rect.height() / segments as f32;
        for i in 0..segments_to_fill {
            let segment_rect = egui::Rect::from_min_size(
                bar_rect.min + egui::Vec2::new(0.0, (segments - 1 - i) as f32 * segment_height),
                egui::Vec2::new(bar_rect.width(), segment_height - 1.0),
            );
            painter.rect_filled(segment_rect, egui::Rounding::ZERO, self.ecg_color);
        }
    }

    fn draw_spo2_panel(&self, painter: &egui::Painter, rect: &egui::Rect) {
        painter.rect_filled(
            *rect,
            egui::Rounding::same(5.0),
            egui::Color32::from_rgb(5, 5, 5),
        );

        // SpO2 label
        painter.text(
            rect.min + egui::Vec2::new(10.0, 10.0),
            egui::Align2::LEFT_TOP,
            "SpO2",
            egui::FontId::proportional(16.0),
            self.spo2_color,
        );

        // Large SpO2 number - real-time calculation
        let spo2_value = self.calculate_spo2();
        painter.text(
            rect.center() + egui::Vec2::new(0.0, 10.0),
            egui::Align2::CENTER_CENTER,
            &format!("{}", spo2_value),
            egui::FontId::proportional(72.0),
            self.spo2_color,
        );

        // % symbol
        painter.text(
            rect.center() + egui::Vec2::new(50.0, -15.0),
            egui::Align2::LEFT_CENTER,
            "%",
            egui::FontId::proportional(24.0),
            self.spo2_color,
        );

        // SpO2 bar indicator
        let bar_rect = egui::Rect::from_min_size(
            rect.max - egui::Vec2::new(20.0, rect.height() - 20.0),
            egui::Vec2::new(10.0, rect.height() - 40.0),
        );

        let segments = 8;
        let spo2_value = self.calculate_spo2();
        // Calculate segments based on SpO2 (85-100% range)
        let spo2_segments =
            (((spo2_value - 85).max(0).min(15) as f32 / 15.0) * segments as f32) as usize;
        let segments_to_fill = spo2_segments.min(segments);

        let segment_height = bar_rect.height() / segments as f32;
        for i in 0..segments_to_fill {
            let segment_rect = egui::Rect::from_min_size(
                bar_rect.min + egui::Vec2::new(0.0, (segments - 1 - i) as f32 * segment_height),
                egui::Vec2::new(bar_rect.width(), segment_height - 1.0),
            );
            painter.rect_filled(segment_rect, egui::Rounding::ZERO, self.spo2_color);
        }
    }

    fn draw_medical_grid(&self, painter: &egui::Painter, rect: &egui::Rect) {
        let small_grid = 5.0;
        let large_grid = 25.0;

        // Small grid
        self.draw_grid_lines(
            painter,
            rect,
            small_grid,
            egui::Color32::from_rgb(0, 40, 0),
            0.3,
        );
        // Large grid
        self.draw_grid_lines(
            painter,
            rect,
            large_grid,
            egui::Color32::from_rgb(0, 60, 0),
            0.5,
        );
    }

    fn draw_grid_lines(
        &self,
        painter: &egui::Painter,
        rect: &egui::Rect,
        spacing: f32,
        color: egui::Color32,
        width: f32,
    ) {
        let stroke = egui::Stroke::new(width, color);

        // Vertical lines
        let mut x = rect.left();
        while x <= rect.right() {
            painter.line_segment(
                [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
                stroke,
            );
            x += spacing;
        }

        // Horizontal lines
        let mut y = rect.top();
        while y <= rect.bottom() {
            painter.line_segment(
                [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
                stroke,
            );
            y += spacing;
        }
    }

    fn draw_ecg_label(&self, painter: &egui::Painter, rect: &egui::Rect, label: &str) {
        painter.text(
            rect.min - egui::Vec2::new(50.0, 0.0),
            egui::Align2::LEFT_TOP,
            label,
            egui::FontId::proportional(16.0),
            self.ecg_color,
        );
    }

    fn draw_pleth_label(&self, painter: &egui::Painter, rect: &egui::Rect, label: &str) {
        painter.text(
            rect.min - egui::Vec2::new(50.0, 0.0),
            egui::Align2::LEFT_TOP,
            label,
            egui::FontId::proportional(12.0),
            self.spo2_color,
        );
    }

    fn draw_realistic_ecg<F>(
        &self,
        painter: &egui::Painter,
        rect: &egui::Rect,
        samples: &VecDeque<EcgSample>,
        value_extractor: F,
    ) where
        F: Fn(&EcgSample) -> f32,
    {
        if samples.len() < 2 {
            return;
        }

        let samples_vec: Vec<&EcgSample> = samples.iter().collect();
        let baseline_y = rect.center().y;
        let amplitude_scale = rect.height() / 6.0; // Realistic ECG amplitude

        let mut points = Vec::new();
        for (i, sample) in samples_vec.iter().enumerate() {
            let time_fraction = i as f32 / samples_vec.len() as f32;
            let x = rect.left() + time_fraction * rect.width();
            let voltage = value_extractor(sample);
            let y = baseline_y - voltage * amplitude_scale;
            let y = y.clamp(rect.top(), rect.bottom());
            points.push(egui::pos2(x, y));
        }

        // Draw ECG waveform with realistic thickness
        if points.len() > 1 {
            for window in points.windows(2) {
                painter.line_segment(
                    [window[0], window[1]],
                    egui::Stroke::new(2.0, self.ecg_color),
                );
            }
        }

        // Draw QRS markers
        for (i, sample) in samples_vec.iter().enumerate() {
            if sample.is_qrs {
                let time_fraction = i as f32 / samples_vec.len() as f32;
                let x = rect.left() + time_fraction * rect.width();
                painter.line_segment(
                    [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 255, 100)),
                );
            }
        }
    }

    fn draw_pleth_waveform(
        &self,
        painter: &egui::Painter,
        rect: &egui::Rect,
        _samples: &VecDeque<EcgSample>,
    ) {
        // Generate realistic plethysmography waveform with variability
        let mut points = Vec::new();
        let num_points = 200;
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f32();

        let heart_rate = 78.0 as f32; // Use average heart rate for pleth
        let pulse_frequency = heart_rate / 60.0; // Hz

        for i in 0..num_points {
            let t = i as f32 / num_points as f32;
            let x = rect.left() + t * rect.width();

            // Realistic pleth waveform based on heart rate
            let phase = (t * 10.0 + current_time * pulse_frequency) * std::f32::consts::PI * 2.0;

            // Pleth has sharp systolic peak, gradual diastolic decay
            let systolic_peak = (phase.sin().max(0.0)).powf(2.0);
            let dicrotic_notch = (phase * 1.5).sin() * 0.1;
            let baseline_drift = (current_time * 0.1).sin() * 0.05;

            let pleth_wave = (systolic_peak + dicrotic_notch + baseline_drift + 0.1)
                .max(0.0)
                .min(1.0);
            let y = rect.bottom() - pleth_wave * rect.height() * 0.9;

            points.push(egui::pos2(x, y));
        }

        if points.len() > 1 {
            for window in points.windows(2) {
                painter.line_segment(
                    [window[0], window[1]],
                    egui::Stroke::new(2.0, self.spo2_color),
                );
            }
        }
    }
}

impl Default for EcgDisplay {
    fn default() -> Self {
        Self::new()
    }
}
