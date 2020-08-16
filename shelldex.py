import sys
from typing import List
import requests
import json
import random

missingno_ascii: str = '''
################@@##########   ## z ##  ## #############
########################### `` zz#  # ++  #@@###########
###########################    # #z  `   z*#############
##########################@   ` #+` # + z*##@#@#########
#########################@# ##*#    # # #  @@###########
########################@##:  .  +#z #`+ +z##@@@########
##########################@#;#,@:@:#  z++z @############
#########################@##;:;;:;:# z+##+ #@###########
#########################@#;```` ` #`+#++  #############
##########################@@;;@:@:#   #;#:##@###########
########################@#@+z     `     z +#############
########################@#@@## # z  z # +#`#@###########
#########################@##  #`# z:M@@@@;@W##@#########
########################@#@ `z + ###z ## ###############
########################### :@@@@;:`##W@@#:#@###########
#########################@#z:;;;:;@:@@##  #@############
###########################; # # +z+#  +#` #############
#########################@#::;:#:;;WW;@#:@;#@##@########
############################  `*  + ` +##+ ###@#########
##########################@@z z `+z# ### ` @############
########################@##:#:W@:;@     z+#@@##@########
#########################@#  ##  #+  ` #  ##@#@#########
###########################:### z##@@W@:iWW#############
###########################  ` # ## # ##  #@##@#########
####################+###z+# #++ # ### + #  ###@#########
################### `    ## @iW@#@@ ## # ###@##@########
####################  `  ##@::@@:@;z  + # #@############
################### `   `+ #W;;@@:@+  z # ###@##########
###################    ` #@  +# `   z# # # @############
###################;###*z#### z#   #  +`# +#@@##########
##################@#  ``  + #W;::#Wz ##++ `#@#@#########
################### @#W@@:@   #`# z  `+ z+ #############
####################    #   @W#@@@# ##### ##############
###################@;@,@:,##z;+  #  ` # # ##############
################@##  `   z#W@::#,;; ##  `+ ##@##########
##################### # #+ z+#z ##  * +  ` @##@#########
################@#@++#### z+####  ##z##+ +@@############
################@##:n##+#+ ;:z*z  #    `z@i#@###########
#################@W`.@:@;@;@W@#:;@;;;;:;:@ #############
###################: # + # ##:# ``#;@:@:@###############
#################@#; # # # ;# ::Wi;`z`#`#@:#############
################@## # # # n  z#`.#+::;:::;+@############
##################@`   `   @  #   z+z####+i#@##@########
################@#@ ` `    ` ` `##  + # # #@@@@@########
###################`          #*#+# # #  ` W#@@@########
###################*zz+####+# #`  `###+; # #@W##########
###################        ## #+ # ,;@#@z n#############
###################     `  #+  ####;@;@#``##@###########
################@#@    #+z+  ##    z++z### ##@##########
#################@#*##+ + #  z# # +@;;:;,:@@###@########
################@###++z+`z ++ #  `### ###z+#@@##########
##################@ ` `  + #z# ##+   #z# # @############
#################@#z+++##`++##+    ;;@W@:@;###@#########
###################+z+# +#`    # # ++    #+#@###########
################@##+z#++  #     #+ z#`# z# #@##@########
###################   `##  #### #+  #     z@##@#########
'''

n_args = len(sys.argv)
pkmn: str = ''

if (n_args == 1):
    print('Enter an existing Pokémon:')
    pkmn = input().lower()
elif(n_args == 2):
    pkmn = sys.argv[1]
else:
    print('Too many arguments, only one Pokémon at once')
    quit()

if pkmn == 'missingno':
    print(missingno_ascii)
    quit()

resp = requests.get(f'https://pokeapi.co/api/v2/pokemon/{pkmn}')

if resp.status_code != 200:
    print('\nNo information recorded about such Pokémon\n')
else:
    last_game: str = 'ultra-sun-ultra-moon'     # Update when available
    resp_json = resp.json()

    # HEADER INFO
    dex_id: int = resp_json['id']
    print(f'\n#{dex_id} {pkmn.upper()}\n')
    print(' / '.join([tp['type']['name'].capitalize() for tp in resp_json['types']]) + '\n')

    resp_desc = requests.get(f'https://pokeapi.co/api/v2/pokemon-species/{pkmn}')
    desc: str = ''

    if resp_desc.status_code != 200:
        desc = 'COULDN\'T GET POKÉMON DESCRIPTION, TRY AGAIN LATER'
    else:
        resp_desc_json = resp_desc.json()['flavor_text_entries']
        eng_descriptions: List[str] = [d['flavor_text'] for d in resp_desc_json if d['language']['name'] == 'en']
        desc_id: int = random.randint(0, len(eng_descriptions) + 1)
        desc = eng_descriptions[desc_id]

    print(desc + '\n')

    # ABILITIES
    print('=> ABILITIES\n')

    for ab in resp_json['abilities']:
        resp_ability = requests.get(ab['ability']['url'])

        if resp_ability.status_code != 200:
            desc = 'COULDN\'T GET ABILITY DESCRIPTION, TRY AGAIN LATER'
        else:
            # Getting the latest description in English
            resp_ability_json = resp_ability.json()
            all_descriptions = resp_ability_json['flavor_text_entries']

            for d in all_descriptions:
                if d['language']['name'] == 'en' and d['version_group']['name'] == last_game:
                    desc = d['flavor_text']
                    break
        
        name_ab: str = ab['ability']['name'].capitalize()
        print(f'{name_ab}: {desc}\n')

    # MOVES
    print('=> MOVES\n')

    for move in resp_json['moves']:
        print(move['move']['name'].capitalize())

    print()
