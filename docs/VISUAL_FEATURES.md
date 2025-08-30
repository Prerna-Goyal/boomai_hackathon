# Patient Monitor Visual Features Documentation

## Professional Medical Equipment Appearance

The ECG Monitor has been completely redesigned to replicate the authentic look and feel of professional hospital patient monitoring equipment, matching real-world medical monitors used in intensive care units and general patient care.

## Complete Patient Monitor Interface

### Layout Structure
- **Header Bar (35px)**: Patient information, bed number, and ECG indicator
- **Main Display Area**: Split between waveforms (60%) and vital signs (40%)
- **Footer Controls (30px)**: Medical equipment control buttons
- **Full-Screen Experience**: No external UI elements to break immersion

## Color Scheme - Medical Equipment Standard

### Background Colors
- **Main Background**: Pure black (`#000000`) - Professional medical monitor
- **Panel Background**: Dark charcoal (`#0A0A0A`) - Vital signs panel areas
- **Vital Background**: Very dark gray (`#141414`) - Individual parameter panels
- **Header Background**: Medical blue (`#0064C8`) - Standard patient info bar

### Waveform Colors (Medical Standard)
- **ECG Waveforms**: Bright green (`#00FF00`) - Traditional cardiac monitor color
- **PLETH Waveform**: Cyan blue (`#00C8FF`) - Pulse oximetry standard
- **RESP Waveform**: Yellow (`#FFFF00`) - Respiratory monitoring standard
- **Grid Lines**: Dark green (`#003C00`, `#006000`) - Medical grid system

### Vital Signs Display Colors
- **Heart Rate**: Bright green (`#00FF00`) - Cardiac parameter standard
- **SpO2**: Cyan blue (`#00C8FF`) - Pulse oximetry standard  
- **Blood Pressure**: White (`#FFFFFF`) - NIBP standard display
- **Temperature**: White (`#FFFFFF`) - Temperature monitoring standard
- **Text Labels**: White (`#FFFFFF`) - High contrast readability

## Patient Monitor Layout

### Header Section (Top Bar)
```
┌─────────────────────────────────────────────────────────────┐
│ BED NO: 01   PATL: DEMO PATIENT   01-01-2024         [ECG] │
└─────────────────────────────────────────────────────────────┘
```
- **Patient Information**: Bed number, patient name, date
- **ECG Indicator**: Green label showing active ECG monitoring
- **Medical Blue Background**: Standard patient monitor header color

### Main Display Area (Split Layout)

#### Left Side: Waveform Display (60% width)
```
┌─────────────────────────────────┐ ┌─────────────────┐
│ II   ~~~~∩~~~∩~~~∩~~~∩~~~       │ │                 │
│      ~  ~ ~ ~ ~ ~ ~ ~ ~         │ │   ECG    80     │
├─────────────────────────────────┤ │                 │
│ V1   ~~∩~~~∩~~~∩~~~∩~~~        │ ├─────────────────┤
│      ~  ~ ~ ~ ~ ~ ~ ~ ~         │ │  SpO2    96     │
├─────────────────────────────────┤ │                 │
│ PLETH ∩   ∩   ∩   ∩   ∩        │ ├─────────────────┤
├─────────────────────────────────┤ │ NIBP  120/80    │
│ RESP  ～   ～   ～   ～         │ │       90        │
└─────────────────────────────────┘ └─────────────────┘
```

#### Right Side: Vital Signs Panels (40% width)
- **Large Numerical Displays**: Hospital-style prominent numbers
- **Four Stacked Panels**: ECG (HR), SpO2, NIBP, Temperature
- **Bar Indicators**: Visual level indicators for each parameter
- **Professional Styling**: Dark panels with bright colored numbers

### Footer Section (Control Bar)
```
┌─────────────────────────────────────────────────────────────┐
│            [SILENCE]  [PAUSE]  [FREEZE]                    │
└─────────────────────────────────────────────────────────────┘
```

## Vital Signs Panels Detail

### 1. ECG Heart Rate Panel
- **Large Number**: "80" in bright green, 72pt font
- **Label**: "ECG" in smaller green text
- **Bar Indicator**: 8-segment vertical bar, 6/8 segments filled
- **Background**: Dark charcoal with rounded corners

### 2. SpO2 Panel  
- **Large Number**: "96" in cyan blue, 72pt font
- **Percentage Symbol**: "%" in smaller cyan text
- **Label**: "SpO2" in cyan
- **Bar Indicator**: 8-segment bar, 7/8 segments filled (96%)

### 3. NIBP (Blood Pressure) Panel
- **Primary Reading**: "120/80" in white, 32pt font
- **Mean Pressure**: "90" in white, 24pt font
- **Units**: "mmHg" in smaller gray text
- **RESP Integration**: "RESP 30" in yellow (combined panel)

### 4. Temperature Panel
- **Multiple Sensors**: "T1 37.2  T2" and "T3 37.0  T4 0.2"
- **Units**: "°C" indicator
- **White Text**: Standard temperature display color

## Waveform Display System

### ECG Waveforms (Lead II, V1)
- **Realistic Morphology**: Authentic P-QRS-T complexes
- **Medical Grid**: Standard 5mm and 25mm grid spacing
- **Bright Green**: Traditional cardiac monitor color (`#00FF00`)
- **2px Line Width**: Professional trace thickness
- **QRS Detection**: Subtle markers at detected beats

### PLETH Waveform (Pulse Oximetry)
- **Pulsatile Pattern**: Realistic arterial pulsation waves
- **Cyan Blue Color**: Standard pulse oximetry display (`#00C8FF`)
- **Slower Frequency**: Matches pulse rate (80 BPM)
- **Arterial Waveform Shape**: Authentic pulse contour

### RESP Waveform (Respiration)
- **Slow Sine Wave**: Realistic breathing pattern
- **Yellow Color**: Standard respiratory monitoring (`#FFFF00`)
- **30 Breaths/Minute**: Realistic respiratory rate
- **Smooth Curves**: Natural respiratory flow pattern

## Medical Grid System

### ECG Grid Standards
- **Fine Grid (5mm)**: Dark green (`#002800`) - Standard ECG paper small squares
- **Major Grid (25mm)**: Medium green (`#003C00`) - Standard ECG paper large squares
- **Proper Scaling**: 25mm/second time base, 10mm/mV voltage scale
- **Medical Accuracy**: Matches real ECG calibration standards

## Interactive Elements

### Control Overlay (Minimized)
- **Floating Window**: Semi-transparent overlay in top-left corner
- **Essential Controls**: Play/Pause button and speed slider only
- **Non-Intrusive**: Doesn't interfere with medical monitor appearance
- **Quick Access**: Easy control without disrupting patient monitoring view

### Footer Controls (Simulated)
- **SILENCE Button**: Yellow background - alarm silence function
- **PAUSE Button**: Gray background - monitoring pause
- **FREEZE Button**: Blue background - waveform freeze function
- **Medical Styling**: Matches real patient monitor button appearance

## Authentic Medical Equipment Features

### Professional Appearance Elements
- **No Desktop UI**: Full immersion as medical equipment
- **Hospital Layout**: Exactly matches real patient monitors
- **Medical Color Standards**: Follows established healthcare display conventions
- **Equipment Branding**: Professional medical device appearance
- **Clinical Information**: Realistic patient and technical data display

### Real-World Accuracy
- **Vital Sign Ranges**: Normal adult values (HR: 80, SpO2: 96%, BP: 120/80)
- **Waveform Morphology**: Medically accurate signal patterns
- **Display Hierarchy**: Critical information prominently displayed
- **Safety Colors**: Red for alarms, green for normal, yellow for warnings

## Technical Implementation

### Rendering Performance
- **60fps Animation**: Smooth real-time waveform scrolling
- **Efficient Drawing**: Optimized for Raspberry Pi GPU
- **Medical Accuracy**: Precise timing and amplitude scaling
- **Low Latency**: Real-time response for medical applications

### Display Optimization
- **Full Screen**: Complete medical monitor immersion
- **High Contrast**: Excellent visibility in clinical environments
- **Large Fonts**: Easy reading from bedside distance
- **Professional Layout**: Intuitive for healthcare professionals

This visual design creates a completely authentic patient monitoring experience that healthcare professionals would instantly recognize and trust, while remaining accessible for educational and demonstration purposes.