import cv2
import numpy as np


def main():
    # Initialize camera (0 is usually the default webcam)
    cap = cv2.VideoCapture(0)

    if not cap.isOpened():
        print("Error: Could not open video device.")
        return

    print("Press 'q' to quit.")

    while True:
        # Capture frame-by-frame
        ret, frame = cap.read()
        if not ret:
            break

        # 1. Convert to Grayscale
        gray = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)

        # 2. Apply Gaussian Blur to reduce noise
        blurred = cv2.GaussianBlur(gray, (5, 5), 0)

        # 3. Edge Detection using Canny
        edges = cv2.Canny(blurred, 50, 150)

        # 4. Thresholding (Otsu's method)
        _, thresholded = cv2.threshold(
            blurred, 0, 255, cv2.THRESH_BINARY + cv2.THRESH_OTSU
        )

        # Display the original frame and the processed frames
        cv2.imshow("Original Frame", frame)
        cv2.imshow("Edges (Canny)", edges)
        cv2.imshow("Thresholded (Otsu)", thresholded)

        # Break loop on 'q' key press
        if cv2.waitKey(1) & 0xFF == ord("q"):
            break

    # Release resources
    cap.release()
    cv2.destroyAllWindows()


if __name__ == "__main__":
    main
