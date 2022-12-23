# This code was based on the work of Atul Kumar (www.facebook.com/atul.kr.007)
# It was improved by Hugo Dubois in a learning perspective and in a school setting


class TrieNode:
     
    # Trie node class
    def __init__(self):
        # [None, None, None, None, None, ...] to the index 25
        # where 26 represents the number of letters in the alphabet
        self.children = [None for _ in range(26)]

        self.failure_link = None

        self.dictionnary_link = None

        self.parent = None

        # word_node is True if node represent the end of a word but it's not a leaf
        self.word_node = False
 
        # isEndOfWord is True if node represent the end of the word
        self.isEndOfWord = False

class Trie:
     
    # Trie data structure class
    def __init__(self):
        self.root = self.getNode()
        self.root.failure_link = self.root
 
    def getNode(self):
     
        # Returns new trie node (initialized to NULLs)
        return TrieNode()
 
    def _charToIndex(self,ch):
         
        # private helper function
        # Converts key current character into index
        # use only 'a' through 'z' and lower case
         
        return ord(ch)-ord('a')
 
 
    def insert(self,key):
         
        # If not present, inserts key into trie
        # If the key is prefix of trie node,
        # just marks leaf node
        pCrawl = self.root
        length = len(key)

        for level in range(length):
            index = self._charToIndex(key[level])
 
            # if current character is not present
            if not pCrawl.children[index]:

                pCrawl.children[index] = self.getNode()

            # Referencing the parent of the new node
            pCrawl.children[index].parent = pCrawl
            pCrawl.children[index].parent.isEndOfWord = False
            # Going to the new node
            pCrawl = pCrawl.children[index]
            if level == (length - 1):
                pCrawl.word_node = True

 
        # mark last node as leaf
        pCrawl.isEndOfWord = True
 
    def search(self, key):
         
        # Search key in the trie
        # Returns true if key presents
        # in trie, else false
        pCrawl = self.root
        length = len(key)
        letters_present = []
        dico_letter_present = []
        for level in range(length):
            index = self._charToIndex(key[level])

            # If the letter isn't present in the trie (not contained in the root's children)
            if not pCrawl.children[index] and pCrawl is self.root:
                pass

            # If the letter isn't present in the actual branch
            elif not pCrawl.children[index]:
                letters_present.clear()
                pCrawl = pCrawl.failure_link


        return pCrawl.isEndOfWord
    
    def failure_links(self):
        # Function that will go through the trie 
        # and that will build the failure links and 
        # the dictionnary links

        visited = []
        queue = []

        pCrawl = self.root
        visited.append(pCrawl)
        queue.append(pCrawl)

        while queue:

            node = queue.pop(0)
            length = len(node.children)

            for index in range(length):

                if node.children[index] is not None:

                    if node.children[index] not in visited:

                        visited.append(node.children[index])
                        queue.append(node.children[index])
                        # If my actual node is directly related to the root
                        # his failure link will be root
                        if node is self.root:         
                            node.children[index].failure_link = self.root

                        # If my actual node isn't directly related to the root
                        # go check his parent's failure link
                        # if this node (the node of the failure link)
                        # has a child that has the same character as my actual node
                        # my actual node's failure link will be this node
                        else:
                            try:
                                # pCrawl is the failure link
                                pCrawl = node.failure_link
                                if pCrawl.children[index]:
                                    node.children[index].failure_link = pCrawl.children[index]
                                else:
                                    if self.root.children[index]:
                                        node.children[index].failure_link = self.root.children[index]
                                    else:
                                        node.children[index].failure_link = self.root
                            except:
                                return False
        print("The failure links have been built")
        return True

    def dictionnary_links(self):

        pCrawl = self.root

        visited = []
        queue = []

        visited.append(pCrawl)
        queue.append(pCrawl)

        while queue:

            node = queue.pop(0)
            length = len(node.children)

            for index in range(length):

                child = node.children[index]

                if node.children[index] is not None:
                    print("ok")
                
                    if node.children[index] not in visited:

                        visited.append(child)
                        queue.append(child)
                        
                        if child.failure_link.word_node:
                            print("A dictionnary link was created")
                            child.dictionnary_link = child.failure_link

                        elif child.failure_link and not child.failure_link.word_node:
                            test = child.failure_link
                            while test.failure_link and test != self.root:
                                print(test)
                                if test.failure_link.word_node:
                                    print("Not a direct word_node was found")
                                    child.dictionnary_link = test.failure_link
                                    break
                                else:
                                    print("going further")
                                    test = test.failure_link

        return True

 
# driver function
def main():
 
    # Input keys (use only 'a' through 'z' and lower case)
    keys = ["the","a","there","anaswe","any",
            "by","their", "than", "me", "anasuit"]
    output = ["Not present in trie",
              "Present in trie"]
 
    # Trie object
    # At this moment 't' is just a node
    t = Trie()
    print("this it t : {} ".format(t))
 
    # Construct trie
    for key in keys:
        t.insert(key)
    print("this is t : {}".format(t))

    # Construct the failure links in the trie
    t.failure_links()

     # Construct the dictionnary links in the trie
    t.dictionnary_links()
 
    # Search for different keys
    print("{} ---- {}".format("anrlphabtheirmot",output[t.search("anrlphabtheirmot")]))
    """
    print("{} ---- {}".format("these",output[t.search("these")]))
    print("{} ---- {}".format("their",output[t.search("their")]))
    print("{} ---- {}".format("thaw",output[t.search("thaw")]))
    print("{} ---- {}".format("that",output[t.search("that")]))
    print("{} ---- {}".format("zionyxay",output[t.search("zionyxay")]))
    """

 
if __name__ == '__main__':
    main()