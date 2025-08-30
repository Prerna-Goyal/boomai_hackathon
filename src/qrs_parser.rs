use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};

#[derive(Debug, Clone)]
pub struct QrsAnnotation {
    pub time: f64,
    pub annotation_type: char,
    pub subtype: u8,
    pub channel: u8,
}

pub struct QrsReader;

impl QrsReader {
    pub fn load_annotations(filename: &str) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);

        let mut annotations = Vec::new();
        let mut time_resolution = 1000.0; // Default time resolution in Hz

        // Try to read the header to get time resolution
        if let Ok(header) = Self::read_header(&mut reader) {
            time_resolution = header;
        }

        // Reset to beginning of file for annotation reading
        reader.seek(SeekFrom::Start(0))?;

        // Skip header if present (look for specific patterns)
        let mut buffer = [0u8; 4];
        if reader.read_exact(&mut buffer).is_ok() {
            // Check if this looks like a header
            if buffer.starts_with(b"## ") {
                // Skip to end of line and continue reading
                let mut line_buffer = Vec::new();
                loop {
                    let mut byte = [0u8; 1];
                    if reader.read_exact(&mut byte).is_err() {
                        break;
                    }
                    if byte[0] == b'\n' {
                        break;
                    }
                    line_buffer.push(byte[0]);
                }
            } else {
                // Reset to beginning if no header found
                reader.seek(SeekFrom::Start(0))?;
            }
        }

        // Read QRS annotations
        loop {
            match Self::read_annotation(&mut reader, time_resolution) {
                Ok(Some(time)) => annotations.push(time),
                Ok(None) => continue, // Skip non-QRS annotations
                Err(_) => break,      // End of file or error
            }
        }

        // If no annotations were read, try alternative parsing methods
        if annotations.is_empty() {
            annotations = Self::parse_alternative_format(filename, time_resolution)?;
        }

        // Sort annotations by time
        annotations.sort_by(|a, b| a.partial_cmp(b).unwrap());

        Ok(annotations)
    }

    fn read_header(reader: &mut BufReader<File>) -> Result<f64, Box<dyn std::error::Error>> {
        let mut line = String::new();
        let mut buffer = [0u8; 1];

        // Read first line
        while reader.read_exact(&mut buffer).is_ok() {
            if buffer[0] == b'\n' {
                break;
            }
            line.push(buffer[0] as char);
        }

        // Parse time resolution from header
        if line.contains("time resolution:") {
            let parts: Vec<&str> = line.split("time resolution:").collect();
            if parts.len() > 1 {
                let resolution_part = parts[1].trim();
                let numbers: Vec<&str> = resolution_part.split_whitespace().collect();
                if !numbers.is_empty() {
                    if let Ok(resolution) = numbers[0].parse::<f64>() {
                        return Ok(resolution);
                    }
                }
            }
        }

        Ok(1000.0) // Default resolution
    }

    fn read_annotation(
        reader: &mut BufReader<File>,
        time_resolution: f64,
    ) -> Result<Option<f64>, Box<dyn std::error::Error>> {
        // Try to read MIT-BIH annotation format
        // Each annotation is typically 2 bytes for time + 2 bytes for annotation info

        // Read time (16-bit unsigned integer)
        let time_raw = reader.read_u16::<LittleEndian>()?;

        // Read annotation type and additional info
        let ann_type = reader.read_u8()?;
        let _subtype = reader.read_u8()?;

        // Convert time to seconds
        let time_seconds = time_raw as f64 / time_resolution;

        // Check if this is a QRS annotation
        // Common QRS annotation codes: N (Normal), L (Left bundle), R (Right bundle), etc.
        if Self::is_qrs_annotation(ann_type) {
            Ok(Some(time_seconds))
        } else {
            Ok(None)
        }
    }

    fn is_qrs_annotation(ann_type: u8) -> bool {
        // MIT-BIH annotation codes for QRS complexes
        match ann_type {
            1 => true,  // Normal beat
            2 => true,  // Left bundle branch block beat
            3 => true,  // Right bundle branch block beat
            4 => true,  // Aberrant atrial premature beat
            5 => true,  // Premature ventricular contraction
            6 => true,  // Fusion of ventricular and normal beat
            7 => true,  // Nodal (junctional) escape beat
            8 => true,  // Atrial escape beat
            9 => true,  // Nodal (junctional) premature beat
            10 => true, // Ventricular escape beat
            11 => true, // Left bundle branch block beat
            12 => true, // Right bundle branch block beat
            _ => {
                // ASCII codes for beat annotations
                match ann_type as char {
                    'N' | 'L' | 'R' | 'A' | 'a' | 'J' | 'S' | 'V' | 'E' | 'j' | '/' | 'f' | 'Q' => {
                        true
                    }
                    _ => false,
                }
            }
        }
    }

    fn parse_alternative_format(
        filename: &str,
        time_resolution: f64,
    ) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
        let mut file = File::open(filename)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;

        let mut annotations = Vec::new();
        let mut i = 0;

        // Try different parsing strategies
        while i < contents.len() {
            // Strategy 1: Look for patterns that might be time stamps
            if i + 4 <= contents.len() {
                // Try reading as 32-bit integer (time in samples)
                let sample_number = u32::from_le_bytes([
                    contents[i],
                    contents[i + 1],
                    contents[i + 2],
                    contents[i + 3],
                ]);

                // Convert to reasonable time (filter out obviously wrong values)
                let time_seconds = sample_number as f64 / time_resolution;
                if time_seconds > 0.0 && time_seconds < 3600.0 {
                    // Reasonable time range (less than 1 hour)
                    annotations.push(time_seconds);
                }

                i += 4;
            } else {
                break;
            }
        }

        // Remove duplicates and sort
        annotations.sort_by(|a, b| a.partial_cmp(b).unwrap());
        annotations.dedup_by(|a, b| (*a - *b).abs() < 0.001); // Remove duplicates within 1ms

        // If we got too many annotations, they're probably not QRS complexes
        // Filter to keep only reasonable QRS intervals (0.3s to 2.0s apart)
        if annotations.len() > 1000 {
            let mut filtered = Vec::new();
            filtered.push(annotations[0]);

            for &time in annotations.iter().skip(1) {
                if let Some(&last_time) = filtered.last() {
                    let interval = time - last_time;
                    if interval >= 0.3 && interval <= 2.0 {
                        filtered.push(time);
                    }
                }
            }

            if !filtered.is_empty() {
                annotations = filtered;
            }
        }

        Ok(annotations)
    }

    pub fn generate_synthetic_qrs(duration_seconds: f64, heart_rate: f32) -> Vec<f64> {
        let mut annotations = Vec::new();
        let rr_interval = 60.0 / heart_rate as f64;
        let mut current_time = 0.5; // Start after 0.5 seconds

        // Add some variability to make it more realistic
        let mut rng_seed = 12345u64;

        while current_time < duration_seconds {
            annotations.push(current_time);

            // Add heart rate variability (±10% of RR interval)
            rng_seed = rng_seed.wrapping_mul(1664525).wrapping_add(1013904223);
            let variability = (rng_seed as f64 / u64::MAX as f64 - 0.5) * 0.2; // ±10%
            let next_interval = rr_interval * (1.0 + variability);

            current_time += next_interval;
        }

        annotations
    }
}
