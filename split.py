import sys
import cv2 as cv
import numpy as np

off = 155
h = 80

def regions(width):
    if width > 1280: # hack for stations on the M1
        return {
            'name': 360,
            'rest': 450
        }
    else:
        return {
            'name': 342,
            'rest': 210 + 152 + 65 # includes from time to track
        }

def box(n, x, w):
    return (x, off + h * n, x + w, off + h * (n + 1))

def split(n, im_raw):
    im = cv.imread(im_raw)

    x = 0
    for name, width in regions(im.shape[1]).items():
        if isinstance(width, (list, tuple)):
            x += width[0]
            width = width[1]

        # crop image and convert to grayscale
        y = off + h * n
        reg = im[y:y+h, x:x+width].copy()
        reg = cv.cvtColor(reg, cv.COLOR_RGB2GRAY)

        # apply a (rectangular) max filter with size 3x3
        reg = cv.dilate(reg, cv.getStructuringElement(cv.MORPH_RECT, (3, 3)))

        reg = cv.medianBlur(reg, 7)

        # apply a min filter with size 5x5
        reg = cv.erode(reg, cv.getStructuringElement(cv.MORPH_RECT, (5, 5)))

        _, reg = cv.threshold(reg, 0, 255, cv.THRESH_OTSU)

        black, white = np.unique(reg, return_counts=True)[1]
        if black > white:
            reg = cv.bitwise_not(reg)

        # add white 12px border
        reg = cv.copyMakeBorder(reg, 12, 12, 12, 12, cv.BORDER_CONSTANT, None, 255)

        cv.imwrite(f'out/{name}.png', reg)

        x += width

    return True

if __name__ == '__main__':
    split(int(sys.argv[1]), sys.argv[2])
