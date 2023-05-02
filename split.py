import sys
from PIL import Image, ImageOps, ImageFilter

off = 160
h = 79

regions = {
    'name': 342,
    'rest': 210 + 152 + 65 # includes from time to track
}

def box(n, x, w):
    return (x, off + h * n, x + w, off + h * (n + 1))

def split(n, im_raw):
    im = Image.open(im_raw)

    x = 0
    for name, width in regions.items():
        if isinstance(width, (list, tuple)):
            x += width[0]
            width = width[1]

        reg = im.crop(box(n, x, width)).convert('L')
        reg = reg.filter(ImageFilter.MedianFilter())
        #reg = ImageEnhance.Contrast(reg).enhance(2)

        colors = reg.convert('1', dither=Image.Dither.NONE).getcolors()
        if len(colors) < 2:
            return False

        black, white = colors
        if black[0] > white[0]:
            reg = ImageOps.invert(reg)

        reg = ImageOps.expand(reg, border=12, fill=0xffffff).convert('1', dither=Image.Dither.NONE)

        reg.save(f'out/{name}.png')

        x += width

    return True

if __name__ == '__main__':
    split(int(sys.argv[1]), sys.argv[2])
