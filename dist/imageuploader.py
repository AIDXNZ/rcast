import io
import tk
import tkinter
import os
from io import BytesIO
import mimetypes
from PIL import Image, ImageTk
import requests
import sys

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
            buf = io.BytesIO()
            new_image = image.rotate(90)
            guess = mimetypes.guess_type(f)
            print(guess[0])
            if guess[0] == 'image/png':
                new_image.save(buf, format('PNG'))
            else:
                new_image.save(buf, format('JPEG'))

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