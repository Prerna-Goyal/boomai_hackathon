use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};

#[derive(Debug, Clone)]
pub struct EdfHeader {
    pub version: String,
    pub patient_id: String,
    pub recording_id: String,
    pub start_date: String,
    pub start_time: String,
    pub header_bytes: u16,
    pub data_format: String,
    pub data_records: u32,
    pub record_duration: f64,
    pub signals: u16,
    pub signal_labels: Vec<String>,
    pub transducer_types: Vec<String>,
    pub physical_dimensions: Vec<String>,
    pub physical_minimums: Vec<f64>,
    pub physical_maximums: Vec<f64>,
    pub digital_minimums: Vec<i16>,
    pub digital_maximums: Vec<i16>,
    pub prefiltering: Vec<String>,
    pub samples_per_record: Vec<u16>,
    pub reserved: Vec<String>,
}

pub struct EdfReader {
    file: BufReader<File>,
    header: EdfHeader,
    current_record: u32,
    samples_per_signal: Vec<usize>,
}

impl EdfReader {
    pub fn new(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);

        let header = Self::read_header(&mut reader)?;
        let samples_per_signal = header
            .samples_per_record
            .iter()
            .map(|&x| x as usize)
            .collect();

        Ok(EdfReader {
            file: reader,
            header,
            current_record: 0,
            samples_per_signal,
        })
    }

    fn read_header(reader: &mut BufReader<File>) -> Result<EdfHeader, Box<dyn std::error::Error>> {
        let mut buffer = [0u8; 8];

        // Read version (8 bytes)
        reader.read_exact(&mut buffer)?;
        let version = String::from_utf8_lossy(&buffer).trim().to_string();

        // Read patient identification (80 bytes)
        let mut patient_buffer = [0u8; 80];
        reader.read_exact(&mut patient_buffer)?;
        let patient_id = String::from_utf8_lossy(&patient_buffer).trim().to_string();

        // Read recording identification (80 bytes)
        let mut recording_buffer = [0u8; 80];
        reader.read_exact(&mut recording_buffer)?;
        let recording_id = String::from_utf8_lossy(&recording_buffer)
            .trim()
            .to_string();

        // Read start date (8 bytes)
        let mut date_buffer = [0u8; 8];
        reader.read_exact(&mut date_buffer)?;
        let start_date = String::from_utf8_lossy(&date_buffer).trim().to_string();

        // Read start time (8 bytes)
        let mut time_buffer = [0u8; 8];
        reader.read_exact(&mut time_buffer)?;
        let start_time = String::from_utf8_lossy(&time_buffer).trim().to_string();

        // Read header bytes (8 bytes)
        let mut header_bytes_buffer = [0u8; 8];
        reader.read_exact(&mut header_bytes_buffer)?;
        let binding = String::from_utf8_lossy(&header_bytes_buffer);
        let header_bytes_str = binding.trim();
        let header_bytes: u16 = header_bytes_str.parse().unwrap_or(256);

        // Read data format version (44 bytes)
        let mut format_buffer = [0u8; 44];
        reader.read_exact(&mut format_buffer)?;
        let data_format = String::from_utf8_lossy(&format_buffer).trim().to_string();

        // Read number of data records (8 bytes)
        let mut records_buffer = [0u8; 8];
        reader.read_exact(&mut records_buffer)?;
        let binding = String::from_utf8_lossy(&records_buffer);
        let records_str = binding.trim();
        let data_records: u32 = records_str.parse().unwrap_or(0);

        // Read duration of a data record (8 bytes)
        let mut duration_buffer = [0u8; 8];
        reader.read_exact(&mut duration_buffer)?;
        let binding = String::from_utf8_lossy(&duration_buffer);
        let duration_str = binding.trim();
        let record_duration: f64 = duration_str.parse().unwrap_or(1.0);

        // Read number of signals (4 bytes)
        let mut signals_buffer = [0u8; 4];
        reader.read_exact(&mut signals_buffer)?;
        let binding = String::from_utf8_lossy(&signals_buffer);
        let signals_str = binding.trim();
        let signals: u16 = signals_str.parse().unwrap_or(1);

        // Read signal specifications
        let mut signal_labels = Vec::new();
        let mut transducer_types = Vec::new();
        let mut physical_dimensions = Vec::new();
        let mut physical_minimums = Vec::new();
        let mut physical_maximums = Vec::new();
        let mut digital_minimums = Vec::new();
        let mut digital_maximums = Vec::new();
        let mut prefiltering = Vec::new();
        let mut samples_per_record = Vec::new();
        let mut reserved = Vec::new();

        // Signal labels (16 bytes each)
        for _ in 0..signals {
            let mut label_buffer = [0u8; 16];
            reader.read_exact(&mut label_buffer)?;
            signal_labels.push(String::from_utf8_lossy(&label_buffer).trim().to_string());
        }

        // Transducer types (80 bytes each)
        for _ in 0..signals {
            let mut transducer_buffer = [0u8; 80];
            reader.read_exact(&mut transducer_buffer)?;
            transducer_types.push(
                String::from_utf8_lossy(&transducer_buffer)
                    .trim()
                    .to_string(),
            );
        }

        // Physical dimensions (8 bytes each)
        for _ in 0..signals {
            let mut dimension_buffer = [0u8; 8];
            reader.read_exact(&mut dimension_buffer)?;
            physical_dimensions.push(
                String::from_utf8_lossy(&dimension_buffer)
                    .trim()
                    .to_string(),
            );
        }

        // Physical minimums (8 bytes each)
        for _ in 0..signals {
            let mut min_buffer = [0u8; 8];
            reader.read_exact(&mut min_buffer)?;
            let binding = String::from_utf8_lossy(&min_buffer);
            let min_str = binding.trim();
            physical_minimums.push(min_str.parse().unwrap_or(-2048.0));
        }

        // Physical maximums (8 bytes each)
        for _ in 0..signals {
            let mut max_buffer = [0u8; 8];
            reader.read_exact(&mut max_buffer)?;
            let binding = String::from_utf8_lossy(&max_buffer);
            let max_str = binding.trim();
            physical_maximums.push(max_str.parse().unwrap_or(2047.0));
        }

        // Digital minimums (8 bytes each)
        for _ in 0..signals {
            let mut min_buffer = [0u8; 8];
            reader.read_exact(&mut min_buffer)?;
            let binding = String::from_utf8_lossy(&min_buffer);
            let min_str = binding.trim();
            digital_minimums.push(min_str.parse().unwrap_or(-2048));
        }

        // Digital maximums (8 bytes each)
        for _ in 0..signals {
            let mut max_buffer = [0u8; 8];
            reader.read_exact(&mut max_buffer)?;
            let binding = String::from_utf8_lossy(&max_buffer);
            let max_str = binding.trim();
            digital_maximums.push(max_str.parse().unwrap_or(2047));
        }

        // Prefiltering (80 bytes each)
        for _ in 0..signals {
            let mut prefilter_buffer = [0u8; 80];
            reader.read_exact(&mut prefilter_buffer)?;
            prefiltering.push(
                String::from_utf8_lossy(&prefilter_buffer)
                    .trim()
                    .to_string(),
            );
        }

        // Samples per record (8 bytes each)
        for _ in 0..signals {
            let mut samples_buffer = [0u8; 8];
            reader.read_exact(&mut samples_buffer)?;
            let binding = String::from_utf8_lossy(&samples_buffer);
            let samples_str = binding.trim();
            samples_per_record.push(samples_str.parse().unwrap_or(360));
        }

        // Reserved (32 bytes each)
        for _ in 0..signals {
            let mut reserved_buffer = [0u8; 32];
            reader.read_exact(&mut reserved_buffer)?;
            reserved.push(String::from_utf8_lossy(&reserved_buffer).trim().to_string());
        }

        Ok(EdfHeader {
            version,
            patient_id,
            recording_id,
            start_date,
            start_time,
            header_bytes,
            data_format,
            data_records,
            record_duration,
            signals,
            signal_labels,
            transducer_types,
            physical_dimensions,
            physical_minimums,
            physical_maximums,
            digital_minimums,
            digital_maximums,
            prefiltering,
            samples_per_record,
            reserved,
        })
    }

    pub fn read_signals(&mut self) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error>> {
        let mut all_samples = Vec::new();

        // Calculate total samples across all records
        let total_samples_per_signal: usize =
            self.header.samples_per_record[0] as usize * self.header.data_records as usize;

        // Initialize signal vectors
        for _ in 0..self.header.signals {
            all_samples.push(Vec::with_capacity(total_samples_per_signal));
        }

        // Seek to start of data (after header)
        self.file
            .seek(SeekFrom::Start(self.header.header_bytes as u64))?;

        // Read each data record
        for _record in 0..self.header.data_records {
            // Read samples for each signal in this record
            for signal_idx in 0..self.header.signals as usize {
                let samples_in_record = self.header.samples_per_record[signal_idx] as usize;

                for _sample in 0..samples_in_record {
                    // Read 16-bit signed integer (little endian)
                    match self.file.read_i16::<LittleEndian>() {
                        Ok(digital_value) => {
                            // Convert digital value to physical value
                            let physical_value =
                                self.digital_to_physical(digital_value, signal_idx);
                            all_samples[signal_idx].push(physical_value);
                        }
                        Err(_) => {
                            // If we can't read more data, return what we have
                            return Ok(self.transpose_samples(all_samples));
                        }
                    }
                }
            }
        }

        Ok(self.transpose_samples(all_samples))
    }

    fn digital_to_physical(&self, digital_value: i16, signal_idx: usize) -> f32 {
        let digital_min = self.header.digital_minimums[signal_idx] as f64;
        let digital_max = self.header.digital_maximums[signal_idx] as f64;
        let physical_min = self.header.physical_minimums[signal_idx];
        let physical_max = self.header.physical_maximums[signal_idx];

        let digital_range = digital_max - digital_min;
        let physical_range = physical_max - physical_min;

        if digital_range == 0.0 {
            return 0.0;
        }

        let normalized = (digital_value as f64 - digital_min) / digital_range;
        let physical_value = physical_min + normalized * physical_range;

        physical_value as f32
    }

    fn transpose_samples(&self, signal_samples: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
        if signal_samples.is_empty() || signal_samples[0].is_empty() {
            return Vec::new();
        }

        let num_samples = signal_samples[0].len();
        let num_signals = signal_samples.len();
        let mut transposed = Vec::with_capacity(num_samples);

        for sample_idx in 0..num_samples {
            let mut sample_vector = Vec::with_capacity(num_signals);
            for signal_idx in 0..num_signals {
                if sample_idx < signal_samples[signal_idx].len() {
                    sample_vector.push(signal_samples[signal_idx][sample_idx]);
                } else {
                    sample_vector.push(0.0);
                }
            }
            transposed.push(sample_vector);
        }

        transposed
    }

    pub fn get_header(&self) -> &EdfHeader {
        &self.header
    }

    pub fn get_sample_rate(&self) -> f32 {
        if self.header.record_duration > 0.0 && !self.header.samples_per_record.is_empty() {
            self.header.samples_per_record[0] as f32 / self.header.record_duration as f32
        } else {
            360.0 // Default ECG sample rate
        }
    }

    pub fn get_signal_names(&self) -> &Vec<String> {
        &self.header.signal_labels
    }
}
