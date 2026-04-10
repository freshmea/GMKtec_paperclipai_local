# Lesson Plan: Introduction to Robotics Sensors

## Overview
This lesson introduces students to the fundamental concept of sensors in robotics. Students will learn how robots perceive their environment through various modalities.

## Learning Objectives
- Define what a sensor is in the context of robotics.
- Distinguish between different types of sensors (Proprioceptive vs. Exteroceptive).
- Understand the difference between analog and digital sensor signals.
- Identify common sensors used in mobile robotics (IMU, LiDAR, Ultrasonic, Camera).

## Materials Needed
- A basic robot kit (e.g., Raspberry Pi or Arduino based).
- Various sensor modules (Ultrasonic, IMU, Camera).
- Computer with Python installed.

## Lesson Outline

### 1. Introduction (15 mins)
- Discussion: How do humans perceive the world? (Sight, Touch, Hearing).
- Analogy: Sensors are the "senses" of a robot.

### 2. Sensor Categorization (20 mins)
- **Proprioceptive Sensors**: Measure internal state (e.g., battery level, wheel encoders, IMU).
- **Exteroceptive Sensors**: Measure external environment (e.g., LiDAR, Ultrasonic, Camera, GPS).

### 3. Deep Dive: The Ultrasonic Sensor (25 mins)
- Principle of operation: Time-of-Flight (ToF).
- Calculation: $Distance = \frac{Speed \times Time}{2}$.
- Limitations: Surface texture, angle of incidence, and environmental noise.

### 4. Hands-on Activity (30 mins)
- Students connect an ultrasonic sensor to a microcontroller.
- Write a simple script to print distance values to the console.
- Experiment with different object materials (hard vs. soft) to see how they affect readings.

## Assessment
- Quiz on sensor types.
- Successful demonstration of the ultrasonic sensor reading code.
