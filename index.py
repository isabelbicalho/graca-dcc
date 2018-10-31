import os
import requests
import traceback
import json
from flask import Flask, request
from unicodedata import normalize
import unicodedata
import re
import textwrap

from rivescript import rivescript

rs = rivescript.RiveScript()

rs.load_directory('rive-files')
rs.sort_replies()

token = os.environ.get('FB_ACCESS_TOKEN')
app = Flask(__name__)

def remove_accents(sentence, codif='utf-8'):
    try:
        nfkd = unicodedata.normalize('NFKD', sentence)
        sentence = u"".join([c for c in nfkd if not unicodedata.combining(c)])
        return re.sub('[^a-zA-Z0-9 \\\]', '', sentence)
    except TypeError:
        return normalize('NFKD', sentence.decode(codif)).encode('ASCII','ignore')
    return sentence

@app.route('/', methods=['GET', 'POST'])
def webhook():
    if request.method == 'POST':
        try:
            data = json.loads(request.data.decode())
            text = data['entry'][0]['messaging'][0]['message']['text']
            sender = data['entry'][0]['messaging'][0]['sender']['id']
            msg = rs.reply(sender,remove_accents(text))
            msgs = textwrap.wrap(msg,600,break_long_words=False)
            for m in msgs:
                payload = {'recipient': {'id': sender}, 'message': {'text': m}}
                r = requests.post('https://graph.facebook.com/v2.6/me/messages/?access_token=' + token, json=payload)
        except Exception as e:
            print(traceback.format_exc())
    elif request.method == 'GET': # Para a verificacao inicial
        if request.args.get('hub.verify_token') == os.environ.get('FB_VERIFY_TOKEN'):
            return request.args.get('hub.challenge')
        return "Wrong Verify Token"
    return 'Nothing'

if __name__ == '__main__':
    app.run(debug=True)
