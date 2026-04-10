# Research: Common Algorithms in Robotics

## 1. Navigation and Path Planning
Algorithms used to move a robot from one point to another while avoiding obstacles.

- **A* (A-Star) Algorithm**: A widely used search algorithm for finding the shortest path between nodes in a graph.
- **Dijkstra's Algorithm**: Finds the shortest path from a starting node to all other nodes in a graph.
- **RRT (Rapidly-exploring Random Tree)**: A sampling-based algorithm used for high-dimensional path planning.
- **D* (Dynamic A*)**: An incremental version of A* that can adapt to changes in the environment.

## 2. Localization and Mapping
Algorithms used to determine the robot's position and create a map of its surroundings.

- **SLAM (Simultaneous Localization and Mapping)**: The process of building a map of an unknown environment while simultaneously keeping track of the robot's location within it.
- **Extended Kalman Filter (EKF)**: A mathematical method used to estimate the state of a nonlinear system from a series of noisy measurements.
- **Particle Filter**: A probabilistic algorithm used for localization and tracking in uncertain environments.

## 3. Control Algorithms
Algorithms used to manage the robot's motion and maintain stability.

- **PID (Proportional-Integral-Derivative) Control**: A common control loop feedback mechanism used to maintain a desired state (e.g., speed, position).
- **MPC (Model Predictive Control)**: An advanced control technique that uses a mathematical model of the robot to predict future behavior and optimize control actions.

## 4. Practical Application and Learning Resources

Understanding these algorithms is fundamental to robotics education. Students can implement these algorithms using various programming languages and tools.

### Educational Approach
- **Mathematical Foundation**: Start with the theory behind PID and A* in a controlled mathematical environment (e.g., using Python/NumPy).
- **Simulation-Based Learning**: Implement path planning (A*, RRT) in simulation environments like Gazebo or Webots to visualize the algorithm's effectiveness and handle obstacle avoidance safely.
- **Hardware Implementation**: Transition successful simulation-based algorithms to real hardware (e.g., Raspberry Pi or Arduino-based robots) to learn about real-world noise, latency, and sensor inaccuracies.

### Curated Resources
Practical implementations and tutorials for these algorithms are being organized in the research repository to support this learning pathway.
