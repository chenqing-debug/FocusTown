from windon import FixedRatioWindow
from PyQt6.QtWidgets import QApplication
import sys

if __name__ == "__main__":
    app = QApplication(sys.argv)
    window = FixedRatioWindow(16, 9)
    window.show()
    sys.exit(app.exec())