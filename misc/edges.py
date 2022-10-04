
from skimage import io
import numpy as np
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from time import time

def display(img, title=None):
    # Show image
    plt.figure(figsize = (5,5))
    plt.imshow(img)
    plt.title(title)
    plt.axis('off')
    plt.show()

### Bonus utility for saving images
def save(img, name, title=None):
    # Show image
    plt.figure(figsize = (5,5))
    plt.imshow(img)
    plt.title(title)
    plt.axis('off')
    plt.savefig(f"./{name}.jpg", bbox_inches='tight')

import cv2

fil = np.array(
[
    [1,0,-1]
])

image = cv2.imread('./iguana.png')

# # The image is dark, and it's hard to tell where the edges are that we found. How can we improve on this? Can we do this in a single filter?

fil = np.array([[1, 0, -1]])
save(cv2.filter2D(image, -1, fil), "brighter")

# # The algorithm finds lots of edges, but we don't care about all of them. Could we keep only the brightest edges somehow?

fil = np.array([2, 0, -2])
result = cv2.filter2D(image, -1, fil)
threshold = 60
def f(x):
    if x < threshold:
        return 0
    else:
        return x
result = np.vectorize(f)(result)
save(result, "filtered")

# # We used a horizontal derivative filter. Create and run a vertical derivative filter. Should it look similar? Does it? Find an image where the horizontal and vertical derivative filters produce very different output.

stripes = cv2.imread('./stripes.jpg')
fil = np.array([[2, 0, -2]]) # needs to be a 2d array!
print(np.transpose(fil), fil)
save(cv2.filter2D(stripes, -1, np.transpose(fil)), "vert")
save(cv2.filter2D(stripes, -1, fil), "nonvert")

# # How does the derivative filter respond to noise? Load and run the filter on the noisy_einstein image. Can you improve on this result?

einstein = cv2.imread('./noisy_einstein.png')
save(cv2.filter2D(einstein, -1, fil*2), "einstein")
save(cv2.filter2D(cv2.filter2D(einstein, -1, np.ones((3,3))/9), -1, fil*2), "blur_einstein")

########
# Adv edge detection
#######

# Recall from lecture that the two parameters to Gaussian blur are kernel size and sigma. How do changing these parameters affect the output of the blur filter? You may want to find a different example image to illustrate your point.

save(cv2.GaussianBlur(einstein, (5,5), 0.1), "sigma01")
save(cv2.GaussianBlur(einstein, (5,5), 0.5), "sigma05")
save(cv2.GaussianBlur(einstein, (5,5), 1), "sigma1")
save(cv2.GaussianBlur(einstein, (5,5), 2), "sigma2")
save(cv2.GaussianBlur(einstein, (5,5), 10), "sigma10")

save(cv2.GaussianBlur(einstein, (5,5), 0.1), "kern5")
save(cv2.GaussianBlur(einstein, (15,15), 0.1), "kern15")
save(cv2.GaussianBlur(einstein, (25,25), 0.1), "kern25")
save(cv2.GaussianBlur(einstein, (45,45), 0.1), "kern45")

from skimage import feature, data

coins = data.coins()
# Play with the thresholds to get different output. How does changing each threshold affect the edges that the algorithm finds?
save(feature.canny(coins, sigma=1, low_threshold=5, high_threshold=50), "low_5")
save(feature.canny(coins, sigma=1, low_threshold=25, high_threshold=50), "low_25")
save(feature.canny(coins, sigma=1, low_threshold=40, high_threshold=50), "low_40")
save(feature.canny(coins, sigma=1, low_threshold=45, high_threshold=50), "low_45")

save(feature.canny(coins, sigma=1, low_threshold=25, high_threshold=30), "high_30")
save(feature.canny(coins, sigma=1, low_threshold=25, high_threshold=50), "high_50")
save(feature.canny(coins, sigma=1, low_threshold=25, high_threshold=80), "high_80")
save(feature.canny(coins, sigma=1, low_threshold=25, high_threshold=90), "high_90")

# Imagine that you have an image with lots of false positives: that is, it finds lots of edges that aren't actually edges. How would you adjust thresholds to improve the result?

## Raise them 

# Imagine that you have an image where the edges don't connect well: that is, it finds some edges, but the edges tend to be broken lines instead of solid lines. How would you adjust thresholds to improve the result?

## Lower the low threshold some

# Remember from exercise 1 that the two parameters to the Gaussian blur are kernel size and sigma, and that both affect the output of the blur filter. Notice that skimage's canny implementation only takes sigma as a parameter. Without modifying the source code, how might you incorporate a different kernel size into the implementation?

## I'd pre-blur the image, then set the sigma to not blur it at all?
prepross = cv2.GaussianBlur(coins, (5,5), 0.1)
save(feature.canny(coins, sigma=0, low_threshold=25, high_threshold=50), "prepross")

# Try to improve the edges you find by tweaking the parameters.

# Try running the edge detector on some different images. skimage.data has a good set to start with. You can also look at Berkeley's collection of benchmark images. Take notes on which images Canny performs well on, and which it does not.

### Hough Transform

# #  We've found some lines. Lots of them, in fact. Using only the techniques we've learned so far, how can we clean up this image to only show the lines that correspond to lanes? Optional: implement some of them and show the improvement in the produced image.

def show_lines(edge_image, lines):
    plt.figure(figsize = (5,5))
    plt.imshow(edge_image * 0)
    plt.axis('off')
    for line in lines:
        p0, p1 = line
        plt.plot((p0[0], p1[0]), (p0[1], p1[1]))
    plt.show()


def save_lines(edge_image, lines, name):
    plt.figure(figsize = (5,5))
    plt.imshow(edge_image * 0)
    plt.axis('off')
    for line in lines:
        p0, p1 = line
        plt.plot((p0[0], p1[0]), (p0[1], p1[1]))
    plt.savefig(f"./{name}.jpg", bbox_inches='tight')
    
from skimage.transform import probabilistic_hough_line

# # These lines should be ideal for both the canny edge detection and the hough transform, so let's just go crazy with the thresholding. We could also crank up the sigma value.

image = cv2.imread('./road.jpg', flags=cv2.IMREAD_GRAYSCALE)

edge_image = feature.canny(image, sigma=1, low_threshold=20, high_threshold=80)
lines = probabilistic_hough_line(edge_image, threshold=1, line_length=20, line_gap=5)
save_lines(image, lines, "poor_canny")

edge_image = feature.canny(image, sigma=1, low_threshold=100, high_threshold=120)
lines = probabilistic_hough_line(edge_image, threshold=1, line_length=20, line_gap=5)
save_lines(edge_image, lines, "mediocre_canny")

# image = cv2.imread('./road.jpg', flags=cv2.IMREAD_GRAYSCALE)
# edge_image = feature.canny(image, sigma=1, low_threshold=100, high_threshold=120)
# display(image)
# # We can also try masking out all the white spots in the image...

image = cv2.imread('./road.jpg', flags=cv2.IMREAD_GRAYSCALE)

for row in range(image.shape[0]):
    for col in range(image.shape[1]):
        if image[row, col] > 200:
            image[row,col] = 255
        else:
            image[row,col] = 0

edge_image = feature.canny(image, sigma=1, low_threshold=100, high_threshold=120)            
lines = probabilistic_hough_line(edge_image, threshold=1, line_length=5, line_gap=5)
save(image, "masked_image")
save_lines(image, lines, "masked")

# # print(image.shape)    
# display(edge_image)

# lines = probabilistic_hough_line(edge_image, threshold=1, line_length=5, line_gap=5)
# save_lines(edge_image, lines, "better_canny")

# We can also use additional information that we have about the image; namely, we know that our images are always coming from a camera mounted on the front of the car. How could we use this information to improve on our lane-finding algorithm? Optional: implement your suggestion and show the improvement in the produced image.


edge_image = feature.canny(image, sigma=1, low_threshold=100, high_threshold=120)
lines = probabilistic_hough_line(edge_image, threshold=50, line_length=25, line_gap=30)

final_lines = []
for line in lines:
    try:
        slope = (line[1][0]-line[0][0])/(line[1][1]-line[0][1])
        if abs(slope) > 4:
            continue
        if line[0][0] < image.shape[1]/2 and slope < 0:            
            final_lines.append(line)
        elif line[0][0] > image.shape[1]/2 and slope > 0:
            final_lines.append(line)
    except:
        pass
save_lines(edge_image, lines, "prefilter")
save_lines(edge_image, final_lines, "postfilter")

# A video is just a series of images (usually 30 images per second). Imagine that your lane-finding algorithm is being fed a video from a front-mounted camera. Describe how you would use your lane-finding algorithm to keep the car driving straight and in its lane.

