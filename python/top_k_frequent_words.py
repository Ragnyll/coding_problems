#  Given an array of strings words and an integer k, return the k most frequent strings.

#  Return the answer sorted by the frequency from highest to lowest. Sort the words with the same frequency by their lexicographical order.

import heapq


class WordPriority:
    def __init__(self, word, priority):
        self.word = word
        self.priority = priority

    def __str__(self):
        return '{}: {}'.format(self.word, self.priority)

    def __lt__(self, rhs) -> bool:
        if self.priority < rhs.priority:
            return True
        elif self.priority == rhs.priority and self.word < rhs.word:
            return True
        return False


def topKFrequent(words: list[str], k: int) -> list[str]:
    word_set = set(words)
    heap = []
    heapq.heapify(heap)

    for word in word_set:
        heapq.heappush(heap, WordPriority(word, -1 * words.count(word)))

    k_freq = []
    for _ in range(k):
        k_freq.append(heapq.heappop(heap).word)

    return k_freq


k_freq = topKFrequent(['dog', 'dog', 'parrot', 'cat'], 2)
print(k_freq)
