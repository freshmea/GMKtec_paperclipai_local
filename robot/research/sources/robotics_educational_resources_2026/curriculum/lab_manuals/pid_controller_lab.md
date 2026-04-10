# Lab Manual: Implementing a PID Controller

## Objective
The goal of this lab is to design and implement a Proportional-Integral-Derivative (PID) controller to control the position of a simulated robot arm (or motor).

## Background
A PID controller is a control loop feedback mechanism widely used in industrial control systems. It calculates an error value as the difference between a measured process variable and a desired setpoint.

The control signal is calculated as:
$u(t) = K_p e(t) + K_i \int e(t) dt + K_d \frac{de(t)}{dt}$

Where:
- $K_p$: Proportional gain
- $K_i$: Integral gain
- $K_d$: Derivative gain
- $e(t)$: Error at time $t$

## Equipment
- Simulation environment (e.g., Gazebo, Webots, or a custom Python script).
- Python development environment.

## Procedure

### 1. Mathematical Modeling
- Understand the system dynamics of the plant you are controlling.
- Define the setpoint (target position).

### 2. Implementation
- Implement the PID algorithm in Python.
- Create a loop that:
    1. Reads the current state (position).
    2. Calculates the error.
    3. Computes the PID output.
    4. Applies the output to the system.
    5. Waits for the next time step.

### 3. Tuning
- **Step 1: Proportional Gain ($K_p$)**: Increase $K_p$ until the system responds quickly but starts to oscillate.
- **Step 2: Derivative Gain ($K_d$)**: Increase $K_d$ to dampen the oscillations and improve stability.
- **Step 3: Integral Gain ($K_i$)**: Increase $K_i$ to eliminate steady-state error.

## Results and Analysis
- Plot the setpoint vs. measured position.
- Identify the overshoot, rise time, and settling time.
- Document the final tuned parameters ($K_p, K_i, K_d$).

## Troubleshooting
- **Oscillations**: Too much $K_p$ or too little $K_d$.
- **Slow response**: Too little $K_p$.
- **Steady-state error**: Too little $K_i$.
