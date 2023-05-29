import io
import tk
import tkinter
import os
from io import BytesIO
import mimetypes
from PIL import Image, ImageTk
import requests
import sys
import cv2
import numpy as np

def RotateImage(rotateImage, angle):
    
    # Taking image height and width
    imgHeight, imgWidth = rotateImage.shape[0], rotateImage.shape[1]
  
    # Computing the centre x,y coordinates
    # of an image
    centreY, centreX = imgHeight//2, imgWidth//2
  
    # Computing 2D rotation Matrix to rotate an image
    rotationMatrix = cv2.getRotationMatrix2D((centreY, centreX), angle, 1.0)
  
    # Now will take out sin and cos values from rotationMatrix
    # Also used numpy absolute function to make positive value
    cosofRotationMatrix = np.abs(rotationMatrix[0][0])
    sinofRotationMatrix = np.abs(rotationMatrix[0][1])
  
    # Now will compute new height & width of
    # an image so that we can use it in
    # warpAffine function to prevent cropping of image sides
    newImageHeight = int((imgHeight * sinofRotationMatrix) +
                         (imgWidth * cosofRotationMatrix))
    newImageWidth = int((imgHeight * cosofRotationMatrix) +
                        (imgWidth * sinofRotationMatrix))
  
    # After computing the new height & width of an image
    # we also need to update the values of rotation matrix
    rotationMatrix[0][2] += (newImageWidth/2) - centreX
    rotationMatrix[1][2] += (newImageHeight/2) - centreY
  
    # Now, we will perform actual image rotation
    rotatingimage = cv2.warpAffine(
        rotateImage, rotationMatrix, (imgHeight, imgWidth))
  
    return rotatingimage

def img_paths():
    """Display images."""
    import requests
    
    image_paths = []
    image_links = []
    for r, d, fs in os.walk(os.getcwd() + "/config/images"):
        for f in fs:
            _p = os.path.join(r, f)
            _f = _p.replace(os.getcwd() + "/config/images", '').lstrip('/')
            image_paths.append(os.path.join('images', _f))
            image = Image.open(os.getcwd()+"/config/images/"+f)
            new_image = image.rotate(90, expand=True)
            new_image = new_image.convert('RGB')
            guess, _ = mimetypes.guess_type(f)
            #print(guess.strip("image/"))
            #img_encode = cv2.imencode("."+guess.replace("image/", ""), new_image)[1]
            
            buf = io.BytesIO()
            if guess[0] == 'image/png':
                new_image.save(buf, format('PNG'))
            else: 
                new_image.save(buf, format('JPEG'))
            #data_encode = np.array(img_encode)
            #bytes_encoded = data_encode.tobytes()
        
            
            #cv2.imwrite(format(os.getcwd()+"/config/images/"+f), new_image)

        
            payload = {'key': '6d207e02198a847aa98d0a2a901485a5','action': 'upload', }
            x = requests.post("https://freeimage.host/api/1/upload", payload, files={"source": buf.getvalue()})
            import json
            data = json.loads(x.text)
            image_links.append(data["image"]["url"])
    print(image_links)
    return image_links

urls = img_paths()
with open("config/links.txt", "w") as outfile:
    for url in urls:
        outfile.write(url+'\n')

sys.exit("Exiting")