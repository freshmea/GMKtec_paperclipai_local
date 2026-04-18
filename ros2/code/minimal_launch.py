from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():
    return LaunchDescription(
        [
            Node(
                package="rclpy",  # In a real scenario, this would be your package name
                executable="python3",
                arguments=["/home/aa/vllm/ros2/code/minimal_publisher.py"],
                name="minimal_publisher",
            ),
            Node(
                package="rclpy",
                executable="python3",
                arguments=["/home/aa/vllm/ros2/code/minimal_subscriber.py"],
                name="minimal_subscriber",
            ),
        ]
    )
