import random
import time


class SimulatedIMU:
    """
    A simulated Inertial Measurement Unit (IMU) class.
    This provides realistic-looking acceleration and angular velocity data
    using random walks to simulate sensor drift and movement.
    """

    def __init__(self, noise_level=0.05):
        self.noise_level = noise_level
        # Initial orientation/velocity (simplified)
        self.accel_x = 0.0
        self.accel_y = 0.0
        self.accel_z = 9.81  # Gravity
        self.gyro_x = 0.0
        self.gyro_y = 0.0
        self.gyro_z = 0.0

    def get_reading(self):
        """
        Simulates a single sensor reading.
        Returns a dictionary containing accelerometer and gyroscope data.
        """
        # Add random noise to simulate real sensor behavior
        self.accel_x += random.uniform(-self.noise_level, self.noise_level)
        self.accel_y += random.uniform(-self.noise_level, self.noise_level)
        self.accel_z += random.uniform(-self.noise_level, self.noise_level)

        self.gyro_x += random.uniform(-self.noise_level, self.noise_level)
        self.gyro_y += random.uniform(-self.noise_level, self.noise_level)
        self.gyro_z += random.uniform(-self.noise_level, self.noise_level)

        return {
            "timestamp": time.time(),
            "accelerometer": {"x": self.accel_x, "y": self.accel_y, "z": self.accel_z},
            "gyroscope": {"x": self.gyro_x, "y": self.gyro_y, "z": self.gyro_z},
        }


if __name__ == "__main__":
    imu = SimulatedIMU()
    print("Starting IMU simulation. Press Ctrl+C to stop.\n")
    try:
        while True:
            reading = imu.get_reading()
            print(
                f"Time: {reading['timestamp']:.2f} | Accel: [{reading['accelerometer']['x']:.2f}, {reading['accelerometer']['y']:.2f}, {reading['accelerometer']['z']:.2f}] | Gyro: [{reading['gyroscope']['x']:.2f}, {reading['gyroscope']['y']:.2f}, {reading['gyroscope']['z']:.2f}]"
            )
            time.sleep(0.1)
    except KeyboardInterrupt:
        print("\nSimulation stopped.")
