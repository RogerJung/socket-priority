import time
import cv2
import numpy as np
from PIL import ImageGrab
import pytesseract
import easyocr
reader = easyocr.Reader(['ch_sim','en']) # this needs to run only once to load the model into memory

def screenshot(bbox):
    # 截取屏幕
    screen = ImageGrab.grab(bbox=bbox)
    screen_np = np.array(screen)
    # 转换为BGR格式
    frame = cv2.cvtColor(screen_np, cv2.COLOR_RGB2BGR)
    return frame

def recognize_text(image):
    # 转换为灰度图像
    gray = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)
    # 应用OCR
    text = pytesseract.image_to_string(gray)
    return text

def main():
    bbox=(600,550,800,700)
    while True:
        # 截图
        frame = screenshot(bbox)
        # 进行图像识别
        result = reader.readtext(frame)
        # 打印识别出的文字
        try:
            print(result[0][1], result[1][1])
        except:
            pass
        # 显示图像 (可以注释掉以提高性能)
        cv2.imshow("Screen", frame)
        
        # 按 'q' 退出
        if cv2.waitKey(1) & 0xFF == ord('q'):
            break
        
        # 控制截图频率
        time.sleep(1)  # 每秒截图一次，可以调整为更快或更慢

    cv2.destroyAllWindows()

if __name__ == "__main__":
    main()

