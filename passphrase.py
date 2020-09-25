import configparser as cfp


class WordList:
    def dump(self):
        for w in self.list:
            print(w)
        print(self.wordCt)


    def __init__(self):
        self.list = [] # maybe a set???
        self.wordCt = 0


def configure():
    global wordList
    conf = cfp.ConfigParser()
    conf.read("config.ini")


# def genWord(wl):
#     return = ""


def main():
    configure()
    wordList = WordList()
    wordList.dump()


main()
