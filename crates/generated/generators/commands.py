import common

class Node:

    # You should never need to call this method. Instead use Node.from_file()
    def __init__(self,json_rep:dict):

        assert all( key in ("type","name","executable","redirects","children","parser") for key in json_rep)


        #Currently the commands.json file only has these types. I don't think 
        # they are going to add more of then, but the code assumes that these 
        # are all the types
        self.type = json_rep["type"]
        assert self.type in ("literal","root","argument")


        #The commands.json file is somewhat annoying, because the name of a node, is 
        #contained in its parrent, and not in the json object.  
        self.name = json_rep["name"]
        assert self.name != "" and self.name != None

        
        #If a command can terminate on this node, then its executable.
        self.executable = json_rep.get("executable",False)
        assert self.executable in (True,False)


        #Some values are redirects to nodes elswhere in the command structure/tree. If we only follow the
        #children, then we have a spanning tree, but if we include redirects the graph can have cycles.
        #A redirect says that the "self's" children are in fact the same as the node we are redirecting to. 
        self.redirects = json_rep.get("redirect",[])


        self.children = tuple(map(lambda x: Node(x), json_rep["children"]))

        #Only arguments have parsers
        self.parser = json_rep.get("parser",None)
        assert self.parser == None or self.type == "argument" 


    def __eq__(self,other):
        if other == None:
            return False
        if type(other) != type(self):
            assert False
        return other.type == self.type and other.name == self.name and self.parser == other.parser and self.redirects == other.redirects

    def __hash__(self):
        return hash((self.name,self.type,str(self.parser)))

    def __repr__(self):
        if self.parser:
            return str({"name": self.name, "type": self.type, "parser": self.parser})
        else:
            return str({"name": self.name, "type": self.type})

    def __str__(self):
        if self.parser:
            return str({"name": self.name, "type": self.type, "parser": self.parser})
        else:
            return str({"name": self.name, "type": self.type})

    #Iterates over children, but not redirects
    def __iter__(self):
        for childe in self.children:
            yield from childe
        yield self

    
    #Iterates over nodes that "self" redirects to, irregardless if it points to a node that has
    #self as one of its children / grand/ grand..grand-children.
    def redirects_itr(self,root:"Node"):
        for name in self.redirects:
            n = root.find(name)
            if n:
                yield n
    
    
    def parrent_to(self,other):
        return any(x == other for x in self)

    
    #This command goes through every node in the tree, and returns the path down to the nodes marked as executable
    def commands_itr(self,path=tuple()):

        if self.executable:
            yield path + (self,)

        for childe in self.children:
            yield from childe.commands_itr(path + (self,))
        
    #Redirects itr

    
data = common.load_minecraft_json("commands.json")
root = Node(data["root"])
parsers = data["parsers"]

#for command in root.commands_itr():
#    print(command)




#print(root)
#print(root.children)