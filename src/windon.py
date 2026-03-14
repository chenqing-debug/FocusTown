from PyQt6.QtWidgets import QMainWindow, QWidget, QVBoxLayout, QPushButton
from PyQt6.QtGui import QResizeEvent
from PyQt6.QtCore import Qt

from focustown_core import AspectRatioConstraint, Size # pyright: ignore[reportAttributeAccessIssue]


class FixedRatioWindow(QMainWindow):
    def __init__(self, ratio_width: int = 16, ratio_height: int = 9):
        super().__init__()
        
        # 比例约束器
        self.constraint = AspectRatioConstraint(ratio_width, ratio_height)
        self.constraint.set_threshold(3)  # 3像素防抖
        
        self.setWindowTitle("FocusTown")
        
        # 计算最佳尺寸
        initial = self.constraint.fit_size(1280, 720)
        self.resize(initial.width, initial.height)
        
        self.setMinimumSize(320, 180)
        
        # 当前主题
        self.current_theme = "dark"
        
        # 防递归标志
        self._resizing = False
        self._last_size = None
        
        # 设置UI
        central_widget = QWidget()
        self.setCentralWidget(central_widget)
        
        layout = QVBoxLayout(central_widget)
        layout.setAlignment(Qt.AlignmentFlag.AlignCenter)
        
        self.theme_button = QPushButton("切换到白天模式")
        self.theme_button.clicked.connect(self.toggle_theme)
        layout.addWidget(self.theme_button)
        
        # 设置样式
        self.update_style()
    
    def update_style(self):
        """更新样式"""
        self.setProperty("theme", self.current_theme)
        
        qss = f"""
        QMainWindow[theme="{self.current_theme}"] {{
            background-color: {self.get_theme_color()};
        }}
        QPushButton {{
            padding: 10px 20px;
            font-size: 14px;
            border-radius: 5px;
            background-color: {"#555" if self.current_theme != "light" else "#ddd"};
            color: {"white" if self.current_theme != "light" else "black"};
        }}
        """
        self.setStyleSheet(qss)
        # 刷新样式
        self.style().unpolish(self)
        self.style().polish(self)
        self.update()
    
    def get_theme_color(self):
        """获取主题颜色"""
        colors = {
            "dark": "#2b2b2b",
            "light": "#f0f0f0",
            "black": "#000000"
        }
        return colors.get(self.current_theme, "#2b2b2b")
    
    def toggle_theme(self):
        """切换主题"""
        if self.current_theme == "dark":
            self.current_theme = "light"
            self.theme_button.setText("切换到纯黑模式")
        elif self.current_theme == "light":
            self.current_theme = "black"
            self.theme_button.setText("切换到暗夜模式")
        else:
            self.current_theme = "dark"
            self.theme_button.setText("切换到白天模式")
        
        self.update_style()

    def resizeEvent(self, event: QResizeEvent) -> None:
        # 防止递归
        if self._resizing:
            super().resizeEvent(event)
            return
        
        new_size = event.size()
        old_size = event.oldSize()
        
        # 第一次或无效尺寸，跳过
        if not old_size.isValid() or old_size.isEmpty():
            super().resizeEvent(event)
            return
        
        # 使用 Rust
        current = Size(new_size.width(), new_size.height())
        old = Size(old_size.width(), old_size.height())
        result = self.constraint.calculate(current, old)
        
        # 尺寸没变，不处理
        if result.width == new_size.width() and result.height == new_size.height():
            super().resizeEvent(event)
            return
        
        # 应用新尺寸
        self._resizing = True
        self.resize(result.width, result.height)
        self._resizing = False
        
        # 调用父类
        super().resizeEvent(event)