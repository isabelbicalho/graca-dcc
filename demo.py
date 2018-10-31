
from unicodedata import normalize
import unicodedata
import re

from rivescript import rivescript

rs = rivescript.RiveScript()

rs.load_directory('rive-files')
rs.sort_replies()

def remove_accents(sentence, codif='utf-8'):
    try:
        nfkd = unicodedata.normalize('NFKD', sentence)
        sentence = u"".join([c for c in nfkd if not unicodedata.combining(c)])
        return re.sub('[^a-zA-Z0-9 \\\]', '', sentence)
    except TypeError:
        return normalize('NFKD', sentence.decode(codif)).encode('ASCII','ignore')
    return sentence
while True:
    pergunta = raw_input('Pergunta: ')
    print 'Resposta: '+rs.reply("bel",remove_accents(pergunta))
