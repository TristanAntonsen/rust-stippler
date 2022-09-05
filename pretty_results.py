import csv
import sys
from PIL import Image, ImageDraw, ImageFilter

def center_rectangle(x,y,l,w,c):
    l = l/2
    w = w/2
    draw.rectangle([x - w, y - l, x + w, y + l],fill=c)

def center_ellipse(x,y,r,c):

    draw.ellipse([x - r, y - r, x + r, y + r],fill=c)


args = sys.argv

weight_image_path = args[1]
weight_image = Image.open(weight_image_path)
width = weight_image.size[0]

image_resolution = 2160
img = Image.new('RGB', (image_resolution, image_resolution))
draw = ImageDraw.Draw(img)

scale = image_resolution / width


center_rectangle(image_resolution / 2, image_resolution / 2, image_resolution, image_resolution, 'white')

with open("end_points.csv","r") as csvFile:
    csvReader = csv.reader(csvFile)

    for row in csvReader:
        x = float(row[0]) * scale
        y = float(row[1]) * scale
        center_ellipse(x,y,4,'black')

img.save('final.png')
