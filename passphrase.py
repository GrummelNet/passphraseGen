import configparser as cfp


class WordList:
    def dump(self):
        for w in self.list:
            print(w)
        print(self.wordCt)
        print(self.minWordLen)
        print(self.maxWordLen)


    # constructor helper function
    def loadList(self, listFn):
        corpus = open(listFn)
        for word in corpus:
            self.list.append(word.strip())
            self.wordCt += 1


    def __init__(self, conf):
        self.list = [] # maybe a set???
        self.wordCt = 0
        self.minWordLen = conf['DEFAULT']['minWordLen']
        self.maxWordLen = conf['DEFAULT']['maxWordLen']
        self.loadList(conf['TESTING']['test1'])


def configure():
    conf = cfp.ConfigParser()
    conf.read("config.ini")
    rv = WordList(conf)
    return rv


# def genWord(wl):
#     return = ""


def main():
    wordList = configure()
    wordList.dump()


main()
