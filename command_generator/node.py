"""
This file contains a class representation of the nodes in the commands.json file.
"""

import json

class Node:

    @staticmethod
    def from_file(path:str):
        with open(path) as fp:
            return Node("root",json.load(fp))

    def __init__(self,name:str,json_rep:dict):
        #The commands.json file is somewhat annoying, because the name of a node, is 
        #contained in its parrent, and not in the json object.  
        self.name = name
        assert name != "" and name != None

        #Currently the commands.json file only has these types. I don't think 
        # they are going to add more of then, but the code assumes that these 
        # are all the types
        self.type = json_rep["type"]
        assert self.type in ("literal","root","argument")


        #If a command can terminate on this node, then its executable.
        self.executable = json_rep.get("executable",False)
        assert self.executable in (True,False)

        #Only arguments have parsers
        self.parser = json_rep.get("parser",None)
        assert self.parser == None or self.type == "argument"

        #Some parsers also have properties, that modify the base parser.
        #As an example the IntegerParser could have a property that it only
        #accepts integers greather then 0. In the generated rust code, we 
        #assume those two parsers are completely different. So adding a modifier
        #results in a new parser.
        self.parser_modifier = json_rep.get("properties","")
       

        self.children = tuple(Node(name,rep) for name,rep in json_rep.get("children",{}).items())

        #Some values are redirects to nodes elswhere in the command structure/tree. If we only follow the
        #children, then we have a spanning tree, but if we include redirects the graph can have cycles.
        #A redirect says that the "self's" children are in fact the same as the node we are redirecting to. 
        #Although there are 4-5 (ish) redirects some of them are not problematic, as they don't introduce cycles.
        #Atm.. All the "problematic" redirects are the ones pointing to the execute command. 
        self.redirects = json_rep.get("redirect",[])
        

    def __eq__(self,other):
        return type(other) == type(self) and other.type == self.type and other.name == self.name and self.parser == other.parser and self.parser_modifier == other.parser_modifier and self.redirects == other.redirects


    def __hash__(self):
        return hash((self.name,self.type,self.parser,str(self.parser_modifier)))

    def __str__(self):
        if self.parser:
            return str({"name": self.name, "type": self.type, "parser": self.parser,"modifier": self.parser_modifier})
        else:
            return str({"name": self.name, "type": self.type})

    def __iter__(self):
        for childe in self.children:
            yield from childe
        yield self

    def find(self,name) -> "Node":
        results = [x for x in self if x.name == name]

        if len(results) == 0:
            assert False
            return None,
        elif len(results) == 1:
            return results[0]
        else:
            #We assume the nodes name is unique.
            assert False
    
    #Iterates over nodes that "self" redirects to, irregardless if it points to a node that has
    #self as one of its children / grand/ grand..grand-children.
    def redirects_itr(self,root:"Node"):
        for name in self.redirects:
            n = root.find(name)
            if n:
                yield n
    
    
    def parrent_to(self,other):
        return len([x for x in self if x == other]) > 0

    
    

         
        

if __name__ == "__main__":
    
    n = Node.from_file("./commands.json")
    print(n)
    i = 0
    for node in n:
        i += 1
        print(node)
    print(i)

    print(tuple(map(str,set(x for x in n if x.type == "literal"))))