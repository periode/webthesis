import a_mind_is_born_hex from '../corpus/a_mind_is_born.hex?raw'
import all_the_names_of_god_pl from '../corpus/all_the_names_of_god.pl?raw'
import all_the_names_of_god_txt from '../corpus/all_the_names_of_god.txt?raw'
import black_perl_pl from '../corpus/black_perl.pl?raw'
import bubblesort_jl from '../corpus/bubblesort.jl?raw'
import buffer_c from '../corpus/buffer.c?raw'
import circle_c from '../corpus/circle.c?raw'
import clearer_method_c from '../corpus/clearer_method.c?raw'
import enum_c from '../corpus/enum.c?raw'
import factorial_c from '../corpus/factorial.c?raw'
import fast_inverse_sqrt_c from '../corpus/fast_inverse_sqrt.c?raw'
import forkbomb_pl from '../corpus/forkbomb.pl?raw'
import game_of_life_apl from '../corpus/game_of_life.apl?raw'
import home_js from '../corpus/home.js?raw'
import home_minified_js from '../corpus/home_minified.js?raw'
import inductive_pl from '../corpus/inductive.pl?raw'
import interpreter_scheme from '../corpus/interpreter.scheme?raw'
import japh_pl from '../corpus/japh.pl?raw'
import mesh_matlab from '../corpus/mesh.matlab?raw'
import ms2000_c from '../corpus/ms2000.c?raw'
import nearest_neighbor_jl from '../corpus/nearest_neighbor.jl?raw'
import numero_mysterioso_asm from '../corpus/numero_mysterioso.asm?raw'
import pseudocode_txt from '../corpus/pseudocode.txt?raw'
import query_php from '../corpus/query.php?raw'
import recursive_iteration_cs from '../corpus/recursive_iteration.cs?raw'
import route_php from '../corpus/route.php?raw'
import select_lines_c from '../corpus/select_lines.c?raw'
import select_lines_sh from '../corpus/select_lines.sh?raw'
import simple_py from '../corpus/simple.py?raw'
import smr_c from '../corpus/smr.c?raw'
import unique_py from '../corpus/unique.py?raw'
import unmaintainable_py from '../corpus/unmaintainable.py?raw'
import unmaintainable_2_c from '../corpus/unmaintainable_2.c?raw'
import verbose_c from '../corpus/verbose.c?raw'
import verbose_refactored_c from '../corpus/verbose_refactored.c?raw'
import multiple_references_rs from '../corpus/multiple_references.rs?raw'
import representation_rs from '../corpus/representation.rs?raw'
import cynical_american_preamble_py from '../corpus/cynical_american_preamble.py?raw'
import unhandled_love_java from '../corpus/unhandled_love.java?raw'
import mac_sched_c from '../corpus/mac_sched.c?raw'
import nested_html from '../corpus/nested.html?raw'
import genalloc_c from '../corpus/genalloc.c?raw'

export const getRawSourceCode = (key: string): string => {
   switch (key) {
      case "a_mind_is_born.hex":
         return a_mind_is_born_hex
      case "all_the_names_of_god.pl":
         return all_the_names_of_god_pl
      case "all_the_names_of_god.txt":
         return all_the_names_of_god_txt
      case "black_perl.pl":
         return black_perl_pl
      case "bubblesort.jl":
         return bubblesort_jl
      case "buffer.c":
         return buffer_c
      case "circle.c":
         return circle_c
      case "clearer_method.c":
         return clearer_method_c
      case "enum.c":
         return enum_c
      case "factorial.c":
         return factorial_c
      case "fast_inverse_sqrt.c":
         return fast_inverse_sqrt_c
      case "forkbomb.pl":
         return forkbomb_pl
      case "game_of_life.apl":
         return game_of_life_apl
      case "home.js":
         return home_js
      case "home_minified.js":
         return home_minified_js
      case "inductive.pl":
         return inductive_pl
      case "interpreter.scheme":
         return interpreter_scheme
      case "japh.pl":
         return japh_pl
      case "mesh.matlab":
         return mesh_matlab
      case "ms2000.c":
         return ms2000_c
      case "nearest_neighbor.jl":
         return nearest_neighbor_jl
      case "numero_mysterioso.asm":
         return numero_mysterioso_asm
      case "pseudocode.txt":
         return pseudocode_txt
      case "query.php":
         return query_php
      case "recursive_iteration.cs":
         return recursive_iteration_cs
      case "route.php":
         return route_php
      case "select_lines.c":
         return select_lines_c
      case "select_lines.sh":
         return select_lines_sh
      case "simple.py":
         return simple_py
      case "smr.c":
         return smr_c
      case "unique.py":
         return unique_py
      case "unmaintainable.py":
         return unmaintainable_py
      case "unmaintainable_2.c":
         return unmaintainable_2_c
      case "verbose.c":
         return verbose_c
      case "verbose_refactored.c":
         return verbose_refactored_c
      case "multiple_references.rs":
         return multiple_references_rs
      case "representation.rs":
         return representation_rs
      case "cynical_american_preamble.py":
         return cynical_american_preamble_py
      case "unhandled_love.java":
         return unhandled_love_java
      case "mac_sched.c":
         return mac_sched_c
      case "nested.html":
         return nested_html
      case "genalloc.c":
         return genalloc_c
      default:
         console.warn(`no source available for ${key}`)
         return `no source available for ${key}`
   }
}