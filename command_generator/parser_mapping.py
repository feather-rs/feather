"""
This file contains code to update the parser mappings in the file 'parser_mapping.json'.
This mapping stores what the different parsers are called. There is no good way to infer this
from commands.json, so the user of the utility must provide the names. 
These names should be valid rust struct names. 

This file sadly also contains code for generating the default implementation for all the parsers. 
You can find them in NEW_PARSERS.
"""

import json, os, pathlib,sys,textwrap
from node import Node

MAPPING_PATH = pathlib.Path("./mappings_and_logs/parser_mapping.json")
NEW_PARSERS = pathlib.Path("./generated/parsers.rs")

def create_file_if_missing() -> bool:
    assert not os.path.isdir(MAPPING_PATH)
    if not os.path.exists(MAPPING_PATH) or os.stat(MAPPING_PATH).st_size == 0:
        with open(MAPPING_PATH,"w") as fp:
            fp.write("{}")
        return True
    return False

def yn_input(msg:str) -> bool:
    yes = {'yes','y', 'ye'}
    no = {'no','n'}
    print("\n"+msg,end="")

    while True:
        choice = input().lower().strip()
        if choice in yes:
            return True
        elif choice in no:
            return False
        else:
            sys.stdout.write("Please respond with 'yes' or 'no'")

def get_mapping():
    create_file_if_missing()
    with open(MAPPING_PATH,"r") as fp:
        mapping = json.load(fp)
        return mapping

#Updates parser_mapping.json so that all know parsers are given a key value pair.
#the value is by default None.
def update_mapping(root:Node):
    current = set(node.parser + str(node.parser_modifier) for node in root if node.type == "argument")
    
    create_file_if_missing()   

    with open(MAPPING_PATH,"r") as fp:
        mapping = json.load(fp)

    existing = set(mapping.keys())

    if existing == current:
        #Then there is nothing to do.
        print("-------------------------------------------------")
        print("No new parsers detected in commands.json")
        print("-------------------------------------------------")
        return 

    new = current - existing
    if len(new) > 0:
        print("--- ............................................ ---")
        print("--- New entries detected for parser_mapping.json ---")
        print("--- ............................................ ---")
    
        for x in new:
            print("\t",x)

        if yn_input("Do you want to add them? y/n: "):
            print(f"Remember to edit the file : {MAPPING_PATH}")
            print("The entries that are mapped to Null are the new parsers.")
            print("They must be mapped to a name that can be used in the Rust code.")
            for x in new:
                mapping[x] = None
        else:
            print(f"Exiting program not having written anything to {MAPPING_PATH}.")
            sys.exit(1)


    outdated = existing - current
    if len(outdated) > 0:
        print("--- .......................................................... ---")
        print("--- Some entries in parser_mapping.json are no longer in use ---")
        print("--- .......................................................... ---")
        for x in outdated:
            print("\t",x)

        if yn_input("Do you want to remove all of them? y/n: "):
            for x in outdated:
                del mapping[x]
        else:
            print(f"Exiting program not having written anything to {MAPPING_PATH}.")
            sys.exit(1)


    with open(MAPPING_PATH,"w") as fp:
        json.dump(mapping,fp,indent=1,sort_keys=True)


def update_generated_parsers():
    with open(NEW_PARSERS,"w") as fp:
        for new_parser_name in get_mapping().values():

            func = f"""
            #[derive(Debug, Error)]
            pub enum {new_parser_name}ParseError {{}}

            #[derive(Clone, Debug)]
            pub struct {new_parser_name}(pub String);

            impl ArgumentKind<CommandCtx> for {new_parser_name} {{
                type ParseError = {new_parser_name}ParseError;

                fn satisfies<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> bool {{
                    input.advance_until(" ");
                    //TODO 
                    true
                }}

                fn parse<'a>(_ctx: &CommandCtx, input: &mut Input<'a>) -> Result<Self, Self::ParseError> {{
                    let text = input.advance_until(" ");
                    //TODO
                    Ok({new_parser_name}(text.to_owned()))
                }}
            }}

            """
            fp.write(textwrap.dedent(func))



if __name__ == "__main__":

    create_file_if_missing()
    root = Node.from_file("./commands.json")
    update_mapping(root)

    
