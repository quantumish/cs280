# """
# This file implements the functions described in the writeup, and is used to generate the images/results.

# To reproduce, place your dog photo in the working directory and rename it to dog.jpg. Additionally, run
# ```
# wget https://upload.wikimedia.org/wikipedia/commons/thumb/b/b6/Image_created_with_a_mobile_phone.png/440px-Image_created_with_a_mobile_phone.png -O image.png
# ```
# to get the larger image used in the advanced exercises.
# """


from skimage import io
import numpy as np
import matplotlib.pyplot as plt
from time import time
import math

### Provided methods for loading/displaying images
def load(image_path):
    out = io.imread(image_path)
    out = out.astype(np.float64) / 255
    return out

def display(img, title=None):
    # Show image
    plt.figure(figsize = (5,5))
    plt.imshow(img, cmap="Greys_r")
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
    plt.savefig(f"./{name}.jpg")
    
# Use default image.
image_path = './dog.jpg'
image = load(image_path)

# Convolution implementation provided by the assignment
def naive_convolution_filter(image, kernel):
    """
    Args:
        image: numpy array of shape (Hi, Wi).
        kernel: numpy array of shape (Hk, Wk).
    Returns:
        out: numpy array of shape (Hi, Wi).
    """
    out = np.zeros(image.shape)
    
    for image_row in range(image.shape[0]):
        for image_column in range(image.shape[1]):
            output_value = 0 
            for kernel_row in range(kernel.shape[0]):
                for kernel_column in range(kernel.shape[1]):
                    image_row_offset = math.ceil(kernel_row - kernel.shape[0] / 2)
                    image_column_offset = math.ceil(kernel_column - kernel.shape[1] / 2)
                    
                    if (image_row + image_row_offset < 0 or 
                        image_row + image_row_offset >= image.shape[0] or
                        image_column + image_column_offset < 0 or 
                        image_column + image_column_offset >= image.shape[1]):
                        image_value = 0
                    else:
                        image_value = image[image_row + image_row_offset, image_column + image_column_offset]

                    output_value += image_value * kernel[kernel_row, kernel_column]

            out[image_row, image_column] = output_value
            
    return out


##################################
#         Exercise 1.            #
##################################

# # Left shift? 
filter1 = np.array(
[
    [0,0,0,0,0],
    [0,0,0,0,0],
    [0,0,0,0,1],
    [0,0,0,0,0],
    [0,0,0,0,0]
])

# # Brightens the image, as it has the same structure as the identity kernel but with a 2 in the center (so it doubles the pixel values instead of preserving them)
filter2 = np.array(
[
    [0,0,0],
    [0,2,0],
    [0,0,0]
])

# # Hint: check the lecture slides for a familiar-looking filter
# # Some sort of weighted blur kernel that brightens the image while blurring it?
# # Ah, it's a sharpen/emboss filter, which makes some sense
filter3 = np.array(
[
    [-.11,-.11,-.11],
    [-.11,1.88,-.11],
    [-.11,-.11,-.11],
])

save(naive_convolution_filter(image, filter1), "filter1")
save(naive_convolution_filter(image, filter2), "filter2")
save(naive_convolution_filter(image, filter3), "filter3")

##################################
#         Exercise 2.            #
##################################

def naive_convolution_filter_rgb(image, kernels):
    """
    Args:
        image: numpy array of shape (Hi, Wi).
        kernels: List of numpy arrays of various shapes
    Returns:
        out: numpy array of shape (Hi, Wi).
    """
    out = np.zeros(image.shape)

    assert(len(kernels) < 3 and len(kernels) > 0)
    for image_row in range(image.shape[0]):
        for image_column in range(image.shape[1]):
            output_value = np.zeros(3)
            for i in range(3):            
                chan = ["R", "G", "B"][i]
                if chan not in kernels:
                    output_value[i] += image[image_row, image_column][i]
                    continue
                kernel = kernels[chan]
                for kernel_row in range(kernel.shape[0]):
                    for kernel_column in range(kernel.shape[1]):
                        image_row_offset = math.ceil(kernel_row - kernel.shape[0] / 2)
                        image_column_offset = math.ceil(kernel_column - kernel.shape[1] / 2)
                        
                        if (image_row + image_row_offset < 0 or 
                            image_row + image_row_offset >= image.shape[0] or
                            image_column + image_column_offset < 0 or 
                            image_column + image_column_offset >= image.shape[1]):
                            image_value = np.zeros(3)
                        else:
                            image_value = image[image_row + image_row_offset, image_column + image_column_offset]

                        # print(image_value)
                        output_value[i] += image_value[i] * kernel[kernel_row, kernel_column]                        

            out[image_row, image_column] = output_value
            
    return out

save(naive_convolution_filter_rgb(image, {"R": filter2, "B": filter1}), "imagerb")

##################################
#         Exercise 3.            #
##################################

blur_nicely = np.array([
    [0.1, 0.1, 0.1],
    [0.1, 0.1, 0.1],
    [0.1, 0.1, 0.1]
])

save(naive_convolution_filter(image, blur_nicely), "blur_nicely")

fil = np.array([
    [0.5, 0.5, 0.5],
    [0.5, -3, 0.5],
    [0.5, 0.5, 0.5]
])

save(naive_convolution_filter(image, fil), "emboss")

fil2 =  np.array([
    [1, 0, -1],
    [1, 0, -1],
    [1, 0, -1],
])

save(naive_convolution_filter(image, fil2), "edges")

##################################
#     Advanced Exercise 2.       #
##################################

# Start using larger image.
image = load("./image.png")

import numba


@numba.jit(nopython=True, nogil=True)
def numba_convolution_filter(image, kernel):
    """
    Args:
        image: numpy array of shape (Hi, Wi).
        kernel: numpy array of shape (Hk, Wk).
    Returns:
        out: numpy array of shape (Hi, Wi).
    """
    out = np.zeros(image.shape)
    
    for image_row in range(image.shape[0]):
        for image_column in range(image.shape[1]):
            output_value = 0.0
            for kernel_row in range(kernel.shape[0]):
                for kernel_column in range(kernel.shape[1]):
                    image_row_offset = math.ceil(kernel_row - kernel.shape[0] / 2)
                    image_column_offset = math.ceil(kernel_column - kernel.shape[1] / 2)
                    
                    if (image_row + image_row_offset < 0 or 
                        image_row + image_row_offset >= image.shape[0] or
                        image_column + image_column_offset < 0 or 
                        image_column + image_column_offset >= image.shape[1]):
                        image_value = 0.0
                    else:
                        image_value = image[image_row + image_row_offset, image_column + image_column_offset]

                    output_value += image_value * kernel[kernel_row, kernel_column]

            out[image_row, image_column] = output_value
            
    return out


def matrix_convolution_filter(image, kernel):
    """
    Args:
        image: numpy array of shape (Hi, Wi).
        kernel: numpy array of shape (Hk, Wk).
    Returns:
        out: numpy array of shape (Hi, Wi).
    """
    out = np.zeros(image.shape)
    rows, cols = image.shape[:2]
    image = np.pad(image, (math.floor(kernel.shape[0]/2), math.floor(kernel.shape[1]/2)))

    for image_row in range(rows):
        for image_column in range(cols):
            region = np.zeros((kernel.shape[0], kernel.shape[1]))
            for kernel_row in range(kernel.shape[0]):
                for kernel_column in range(kernel.shape[1]):
                    region[kernel_row, kernel_column] = image[image_row+kernel_row, image_column+kernel_column]
            new = kernel * region
            out[image_row, image_column] = np.sum(new)            
            
    return out

@numba.jit(nopython=True, nogil=True)
def numba_matrix_convolution_filter(image, kernel):
    """
    Args:
        image: numpy array of shape (Hi, Wi).
        kernel: numpy array of shape (Hk, Wk).
    Returns:
        out: numpy array of shape (Hi, Wi).
    """
    out = np.zeros(image.shape)
    rows, cols = image.shape[:2]
    # np.pad() not implemented in Numba, so we make do manually
    img = np.zeros((image.shape[0]+1, image.shape[1]+1))
    img[math.floor(kernel.shape[0]/2):image.shape[0]+math.floor(kernel.shape[0]/2),
        math.floor(kernel.shape[1]/2):image.shape[1]+math.floor(kernel.shape[1]/2)] = image
    
    for image_row in range(rows):
        for image_column in range(cols):
            region = np.zeros((kernel.shape[0], kernel.shape[1]))
            for kernel_row in range(kernel.shape[0]):
                for kernel_column in range(kernel.shape[1]):
                    region[kernel_row, kernel_column] = img[image_row+kernel_row, image_column+kernel_column]
            new = kernel * region
            out[image_row, image_column] = np.sum(new)            
            
    return out

# quick and dirty utility from https://stackoverflow.com/a/12201744
def rgb2gray(rgb):
    return np.dot(rgb[...,:3], [0.2989, 0.5870, 0.1140])

image = rgb2gray(image)

import time
b = time.time()
naive_convolution_filter(image, fil)
a = time.time()
print(f"naive: {a-b}")

b = time.time()
numba_convolution_filter(image, fil)
a = time.time()
print(f"numba naive: {a-b}")

b = time.time()
matrix_convolution_filter(image, fil)
a = time.time()
print(f"matrix: {a-b}")

b = time.time()
numba_matrix_convolution_filter(image, fil)
a = time.time()
print(f"numba matrix: {a-b}")

##################################
#     Advanced Exercise 1.       #
##################################

shift = np.zeros((81, 81))
shift[0, 0] = 1

@numba.jit(nopython=True)
def replicate(image, image_row, image_column, image_row_offset, image_column_offset):
    row = min(image_row + image_row_offset, image.shape[0])
    row = max(row, 0)
    col = min(image_column + image_column_offset, image.shape[1])
    col = max(col, 0)    
    return image[row, col]
    
@numba.jit(nopython=True)
def boundary_convolution_filter(image, kernel, strategy=replicate):
    """
    Args:
        image: numpy array of shape (Hi, Wi).
        kernel: numpy array of shape (Hk, Wk).
        strategy: Numba-accelerated (w/ nopython pipeline) function taking
            image, image_row, image_col, image_row_offset, image_col_offset and returning
            and returning a pixel value. Defaults to replicate().
    Returns:
        out: numpy array of shape (Hi, Wi).
    """
    out = np.zeros(image.shape)
    
    for image_row in range(image.shape[0]):
        for image_column in range(image.shape[1]):
            output_value = np.zeros(3)
            for kernel_row in range(kernel.shape[0]):
                for kernel_column in range(kernel.shape[1]):
                    image_row_offset = math.ceil(kernel_row - kernel.shape[0] / 2)
                    image_column_offset = math.ceil(kernel_column - kernel.shape[1] / 2)

                    image_value = strategy(image, image_row, image_column, image_row_offset, image_column_offset)
                    output_value += image_value * kernel[kernel_row, kernel_column]

            out[image_row, image_column] = output_value
            
    return out

save(boundary_convolution_filter(load('./dog.jpg'), shift, replicate), "dog_replicate")
