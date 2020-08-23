"""
A command is a list of nodes that mostly terminate on a node that is executable, unless they 
are the start of a infinite loop. At the time of writing this that is only applicable to the execute commands.

At the time i wrote this code leutenant had no way of including commands like these, so it was/is 
important to know what commands are not expressable using leutenant. 
"""

import json, os, pathlib,sys,textwrap
from node import Node
import parser_mapping
import re


# Generates all list of lists containing nodes, these lists of nodes,
# are the different commands that you can execute.
# result: list<list<Node>> where the inner list is a command. 
#
# Note that it also handles redirects, but it does not allow redirecting into a 
# node that is a higher up in the tree. That would result in infinite cycles.
# It therefor returns two lists, one containing all the "normal" commands,
# and one list containing all the onces that could loop indefinitely.
def _recursive_decent(root:Node,node:Node=None,result=[],path=[],redirected=[]):
    if node == None:
        for x in root.children:
            _recursive_decent(root,x,result,path.copy(),redirected)
        return result, redirected
    

    path.append(node)
    if node.executable:
        result.append(path.copy())

    for redirect in node.redirects_itr(root):
        assert redirect != None
        
        if not redirect.parrent_to(node):
            #This if check makes it so that when the node 'message' redirects to msg, then its "just works".
            # That simple rename does not introduce infinitely long commands.  
            for x in redirect.children:
                assert x != None
                _recursive_decent(root,x,result,path.copy(),redirected)
        else:
            assert redirect != None
            new_path = path.copy()
            new_path.append(redirect)
            redirected.append(new_path)
    for x in node.children:
        _recursive_decent(root,x,result,path.copy(),redirected)

    return result, redirected




class Command:

    
    @staticmethod
    def from_file(path):
        #Returns a list of commands present in the commands.json file
        root = Node.from_file(path)

        #result = a list of all 'normal' commands
        #redirects = a list of all commands that are infinitely recursive. 
        commands, redirects = _recursive_decent(root)


        for x in redirects:
            assert x != None

        return tuple([Command(cmd,False) for cmd in commands] + [Command(cmd,True) for cmd in redirects])


    def __init__(self,nodes:tuple,infinite:bool):
        self.nodes = nodes
        self.infinite = infinite
  
    def __eq__(self,other):
        if type(other) != type(self):
            return False
        elif not len(self.nodes) == len(other.nodes):
            return False
        elif not all(a == b for a,b in zip(self.nodes,other.nodes)):
            return False
        return True

    def __str__(self):
        #Returns a very simple representation of the command.
        #example:
        #       msg targets message
        def to_str(node):
            if node.type == "literal":
                return node.name
            elif node.type == "argument":
                if node.parser_modifier:
                    return f"{node.parser}{node.parser_modifier}>"
                else:
                    return f"{node.parser}"
            else:
                assert False #We should only have these two types of nodes

        return " ".join(map(to_str, self.nodes))

    def to_invocation_pattern(self) -> str:
        # Returns the str representation of what would be written inside
        # the macro definition of the leutenant function. It is also the short
        # str that explains how to invoke a command.  
        # Example:
        #   msg <target> <GreedyString> 
        def to_str(node):
            if node.type == "literal":
                return node.name
            elif node.type == "argument":
                return f"<{node.name}>"
            else:
                assert False #We should only have these two types of nodes


        return " ".join(map(to_str,self.nodes))

    def to_argument_signature(self,parser_mapping:dict) -> str:
        def to_str(node):
            if node.type == "literal":
                return None
            elif node.type == "argument":
                return f"_{node.name}:{parser_mapping[node.parser + str(node.parser_modifier)]}"
            else:
                assert False #We should only have these two types of nodes
        
        return ",\n                ".join(filter(lambda n: n != None, map(to_str,self.nodes)))

    def to_function_name(self) -> str:
        def to_str(node):
            if node.type in ("literal","argument"):

                return re.sub(r"\W+","_",node.name.replace(">","gt").replace("<","lt").replace("=","eq").replace("*","star"))
            else:
                assert False #We should only have these two types of nodes
        
        return "_".join(map(to_str,self.nodes))

    def to_code(self,parser_mapping:dict) -> str:
        usage = self.to_invocation_pattern()
        func_name = self.to_function_name()
        arg_sig = self.to_argument_signature(parser_mapping)

        res = textwrap.dedent(f"""
            #[command(usage=\"{usage}\")]
            pub fn {func_name}(
                ctx: &mut CommandCtx,
                {arg_sig}
            ) -> anyhow::Result<()> {{
                if let Some(mut sender_message_receiver) = ctx.world.try_get_mut::<MessageReceiver>(ctx.sender)
                {{
                    let return_text = Text::from("This command \\\"{usage}\\\" is not implemented in this version of feather.").gray().italic();
                    sender_message_receiver.send(return_text);
                }}
                Ok(Some("".to_string()))
            }}""")
        
        return res

        



if __name__ == "__main__":
    commands = Command.from_file("./commands.json")
    pars_map = parser_mapping.get_mapping()
    for x in commands:
        assert x.nodes != None

        print(x.to_code(pars_map))


    
