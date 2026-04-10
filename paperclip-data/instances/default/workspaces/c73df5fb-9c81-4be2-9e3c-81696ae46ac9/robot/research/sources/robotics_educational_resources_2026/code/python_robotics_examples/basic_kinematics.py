import numpy as np


def simple_kinematics(theta1, theta2, l1, l2):
    """
    Calculates the (x, y) position of the end effector of a 2-DOF planar arm.

    Args:
        theta1 (float): The angle of the first joint in radians.
        theta2 (float): The angle of the second joint in radians (relative to link 1).
        l1 (float): The length of the first link.
        l2 (float): The length of the second link.

    Returns:
        tuple: (x, y) coordinates of the end effector.
    """
    x = l1 * np.cos(theta1) + l2 * np.cos(theta1 + theta2)
    y = l1 * np.sin(theta1) + l2 * np.sin(theta1 + theta2)
    return x, y


if __name__ == "__main__":
    # Example usage
    l1_len = 1.0
    l2_len = 1.0
    t1 = np.pi / 4  # 45 degrees
    t2 = np.pi / 4  # 45 degrees

    pos = simple_kinematics(t1, t2, l1_len, l2_len)
    print(f"End effector position at theta1={t1} rad, theta2={t2} rad: {pos}")
