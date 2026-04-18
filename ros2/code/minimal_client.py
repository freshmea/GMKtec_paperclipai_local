import rclpy
from rclpy.node import Node
from example_interfaces.srv import AddTwoInts


class MinimalClient(Node):
    def __init__(self):
        super().__init__("minimal_client")
        self.client = self.create_client(AddTwoInts, "add_two_ints")
        while not self.client.wait_for_service(timeout_sec=1.0):
            self.get_logger().info("service not available, waiting again...")
        self.req = AddTwoInts.Request()

    def send_request(self, a, b):
        self.req.a = a
        self.req.b = b
        self.future = self.client.call_async(self.req)
        rclpy.spin_until_future_complete(self, self.future)
        return self.future.result()


def main(args=None):
    rclpy.init(args=args)
    minimal_client = MinimalClient()

    a, b = 10, 20
    minimal_client.get_logger().info(f"Sending request: {a} + {b}")
    response = minimal_client.send_request(a, b)

    if response is not None:
        minimal_client.get_logger().info(f"Result: {response.sum}")
    else:
        minimal_client.get_logger().error("Service call failed")

    minimal_client.destroy_node()
    rclpy.shutdown()


if __name__ == "__main__":
    main()
