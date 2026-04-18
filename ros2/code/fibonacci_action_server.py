import rclpy
from rclpy.action import ActionServer
from rclpy.node import Node
from example_interfaces.action import Fibonacci

import time


class FibonacciActionServer(Node):
    def __init__(self):
        super().__init__("fibonacci_action_server")
        self._action_server = ActionServer(
            self, Fibonacci, "fibonacci", self.execute_callback
        )
        self.get_logger().info("Fibonacci Action Server is ready.")

    async def execute_callback(self, goal_handle):
        self.get_logger().info("Executing goal...")

        feedback_msg = Fibonacci.Feedback()
        feedback_msg.sequence = []

        for i in range(1, goal_handle.request.order + 1):
            feedback_msg.sequence.append(
                self._get_next_fibonacci(feedback_msg.sequence)
            )
            self.get_logger().info(f"Feedback: {feedback_msg.sequence}")

            goal_handle.publish_feedback(feedback_msg)
            time.sleep(1)

        goal_handle.succeed()

        result = Fibonacci.Result()
        result.sequence = feedback_msg.sequence
        return result

    def _get_next_fibonacci(self, sequence):
        if len(sequence) == 0:
            return 0
        elif len(sequence) == 1:
            return 1
        else:
            return sequence[-1] + sequence[-2]


def main(args=None):
    rclpy.init(args=args)
    action_server = FibonacciActionServer()
    rclpy.spin(action_server)
    action_server.destroy_node()
    rclpy.shutdown()


if __name__ == "__main__":
    main()
