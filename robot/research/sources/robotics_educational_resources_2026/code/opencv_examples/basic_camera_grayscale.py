import cv2
import numpy as np


def main():
    # Initialize camera (0 is default camera)
    cap = cv2.VideoCapture(0)

    if not cap.isOpened():
        print("Error: Could not open camera.")
        return

    print("Press 'q' to quit.")

    while True:
        # Capture frame-by-frame
        ret, frame = cap.read()

        if not ret:
            print("Error: Can't receive frame.")
            break

        # Convert to grayscale
        gray = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)

        # Display the resulting frame
        cv2.imshow("Original Frame", frame)
        cv2.imshow("Grayscale Frame", gray)

        # Break loop on 'q' key press
        if cv2.waitKey(1) & 0xFF == ord("q"):
            break

    # Release everything when done
    cap.release()
    cv2.destroyAllWindows()


if __name__ == "__main__":
    main()
