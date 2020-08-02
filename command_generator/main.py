import os, json, sys
from node import Node
from command import Command
import  parser_mapping

JSON_FILE = "./commands.json"
FUNC_FILE = "./generated/funcs.rs"

def check_for_command_json():
    if not os.path.exists(JSON_FILE) or os.stat(JSON_FILE).st_size < 100:
        print(f"Could not find old commands JSON_FILE: {JSON_FILE}")
        sys.exit(-1)
    
    if os.path.exists(JSON_FILE) and os.stat(JSON_FILE).st_size > 0:
        with open(JSON_FILE,"r") as fp:
            try:
                data = json.load(fp)
            except Exception as e:
                print(e)
                raise ValueError(f"The old commands json JSON_FILE {JSON_FILE} is not formatted properly!")
    
    return


    



if __name__ == "__main__":

    # Make sure command.json JSON_FILE exits
    check_for_command_json()

    # Update the parser mapping json file
    root = Node.from_file(JSON_FILE)
    parser_mapping.update_mapping(root)
    

    #Check if all parsers are given a name.
    #@TODO maybe check that the given names are valid/wellformed struct names.
    if any(x in ("null","Null",None,"") for x in parser_mapping.get_mapping().values()):
        print("\n\nMake sure to give *ALL* new parsers a name in their mapping file!")
        print("And then run this program again.")
        sys.exit(-1)
    


    # Update all the generated default parsers
    parser_mapping.update_generated_parsers()


    
    #Write all the commands into a file
    print("------------------------------------------------#")
    print(" Writing all the commands to ./generated/funcs.rs")
    print("------------------------------------------------#")

    commands = Command.from_file(JSON_FILE)
    pars_map = parser_mapping.get_mapping()
    with open(FUNC_FILE,"w") as fp:
        for cmd in commands:
            if cmd.infinite: 
                #@TODO make it so we can generate code for infinite recursive commands.
                continue
            fp.write(cmd.to_code(pars_map))
            fp.write("\n")
            #print(cmd.to_function_name() + ",")
            













    

    

    


