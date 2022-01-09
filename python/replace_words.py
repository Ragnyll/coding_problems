#  In English, we have a concept called root, which can be followed by some other word to form another longer word - let's call this word successor. For example, when the root "an" is followed by the successor word "other", we can form a new word "another".

#  Given a dictionary consisting of many roots and a sentence consisting of words separated by spaces, replace all the successors in the sentence with the root forming it. If a successor can be replaced by more than one root, replace it with the root that has the shortest length.

#  Return the sentence after the replacement.
class Link:
    def __init__(self, is_final: bool):
        self.is_final = is_final
        self.children = {}


class Trie:
    def __init__(self):
        self.root = Link(True)

    def insert(self, word: str):
        if not word:
            return

        curr_link = self.root
        for c in word:
            if c not in curr_link.children:
                curr_link.children[c] = Link(False)
            curr_link = curr_link.children[c]

        curr_link.is_final = False

    def find_prefix(self, word: str) -> str:
        """find_prefix finds the shortest prefix in word

        :param word: the word to find the prefix for
        :type word: str
        :rtype: str
        """
        curr_link = self.root
        replace_with = ''
        for c in word:
            if c in curr_link.children:
                replace_with += c
                if curr_link.is_final:
                    return replace_with
                curr_link = curr_link.children[c]

        return ''


def replaceWords(dictionary: list[str], sentence: str) -> str:
    trie = Trie()
    for word in set(dictionary):
        trie.insert(word)

    words = sentence.split(' ')
    modified_sentence = ''
    for word in words:
        prefix = trie.find_prefix(word)
        if prefix:
            modified_sentence += prefix
        else:
            modified_sentence += word

    return modified_sentence
