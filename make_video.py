import cv2
import os
import sys


def create_mp4(image_sequence, output_name, frame_rate):
    files = os.listdir(image_sequence)
    files.remove(".DS_Store")
    names = [file.split('.')[0] for file in files]
    try:
        names = sorted([int(n) for n in names])
    except:
        print("Could not sort sequence.")
        quit()

    extension = files[1].split('.')[-1]
    dims = cv2.imread(f"{image_sequence}/{files[0]}").shape

    frameSize = (dims[0], dims[1])

    fourcc = cv2.VideoWriter_fourcc('m','p','4','v')
    video_writer = cv2.VideoWriter(output_name, fourcc, frame_rate, frameSize)

    for name in names:
        path = f"{image_sequence}/{name}.{extension}"
        im = cv2.imread(path)
        video_writer.write(im)

    video_writer.release()

    print(f"{output_name} exported.")

if __name__ == "__main__":

    args = sys.argv
    arg_count = len(args)

    sequence = args[1]

    if arg_count == 2:
        output_name = "output.mp4"
    elif arg_count > 2:
        output_name = args[2]

    if arg_count >3:
        frame_rate = round(float(args[3]))
    else:
        frame_rate = 15


    create_mp4(sequence, output_name, frame_rate)
        