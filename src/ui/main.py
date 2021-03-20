# This Python file uses the following encoding: utf-8
import sys
from PySide2.QtWidgets import QApplication
from PyQt5 import QtWidgets, uic


class Ui(QtWidgets.QDialog):
    def __init__(self):
        super(Ui, self).__init__()
        uic.loadUi('dashboard.ui', self)
        self.show()


if __name__ == "__main__":
#    app = QApplication([])
#    # ...
#    sys.exit(app.exec_())

    app = QtWidgets.QApplication(sys.argv)
    window = Ui()
    app.exec_()
