# Anagramic Squares (98)

In [Problem 98](https://projecteuler.net/problem=98), we're asked to work anagrams and substitute letters for digits to convert words into numbers such that all of them are perfect squares.

## Looking at the words

The file that we're given starts like this:

```json
"A","ABILITY","ABLE","ABOUT","ABOVE"
```

There are a bunch of words in there. In a first step, we group the words by their anagram classes. For this we devise a way to form a representative or label of the class. Sorting the letters in each word is a way to map all elements of an anagram class to a common value which is unique from the other anagram classes.

Then we drop all the classes that only contain one element. The problem statement explicitly says that the anagrams need to be in the list, so if a class has only one element, we therefore cannot find an anagram to it.

After this grouping and filtering of the data, we're left with a bunch of anagram classes that contain two words, one class with three words:

```json
{
    "ABDOR"    : ["BOARD",     "BROAD"            ],
    "ACEINORT" : ["CREATION",  "REACTION"         ],
    "ACER"     : ["CARE",      "RACE"             ],
    "ACT"      : ["ACT",       "CAT"              ],
    "ADEGNR"   : ["DANGER",    "GARDEN"           ],
    "ADEL"     : ["DEAL",      "LEAD"             ],
    "AEHPS"    : ["PHASE",     "SHAPE"            ],
    "AEHRT"    : ["EARTH",     "HEART"            ],
    "AEHT"     : ["HATE",      "HEAT"             ],
    "AEIRS"    : ["ARISE",     "RAISE"            ],
    "AELM"     : ["MALE",      "MEAL"             ],
    "AELST"    : ["LEAST",     "STEAL"            ],
    "AEMN"     : ["MEAN",      "NAME"             ],
    "AENR"     : ["EARN",      "NEAR"             ],
    "AERT"     : ["RATE",      "TEAR"             ],
    "AEST"     : ["EAST",      "SEAT"             ],
    "AET"      : ["EAT",       "TEA"              ],
    "CDEINORTU": ["INTRODUCE", "REDUCTION"        ],
    "CDEIRT"   : ["CREDIT",    "DIRECT"           ],
    "CEENRT"   : ["CENTRE",    "RECENT"           ],
    "CEEPTX"   : ["EXCEPT",    "EXPECT"           ],
    "CEORSU"   : ["COURSE",    "SOURCE"           ],
    "DGO"      : ["DOG",       "GOD"              ],
    "EEHST"    : ["SHEET",     "THESE"            ],
    "EFIL"     : ["FILE",      "LIFE"             ],
    "EFMORR"   : ["FORMER",    "REFORM"           ],
    "EGINOR"   : ["IGNORE",    "REGION"           ],
    "EIMT"     : ["ITEM",      "TIME"             ],
    "EIQTU"    : ["QUIET",     "QUITE"            ],
    "ENOT"     : ["NOTE",      "TONE"             ],
    "ERSU"     : ["SURE",      "USER"             ],
    "FMOR"     : ["FORM",      "FROM"             ],
    "GHINT"    : ["NIGHT",     "THING"            ],
    "GINS"     : ["SIGN",      "SING"             ],
    "HORTW"    : ["THROW",     "WORTH"            ],
    "HOSTU"    : ["SHOUT",     "SOUTH"            ],
    "HOW"      : ["HOW",       "WHO"              ],
    "HSTU"     : ["SHUT",      "THUS"             ],
    "IST"      : ["ITS",       "SIT"              ],
    "NO"       : ["NO",        "ON"               ],
    "NOW"      : ["NOW",       "OWN"              ],
    "OPST"     : ["POST",      "SPOT",      "STOP"]
}
```

# Square classes

We can do a similar thing for the squares and build anagram classes out of square numbers. When forming the class key, we need to make sure that we track the zeros by using strings of digits instead of plain integers.

This gives us anagram squares classes like these:

```json
{
    "22334689" : [ 32239684,  33269824,  42823936                                            ],
    "1225789"  : [  2758921,   8791225                                                       ],
    "112556689": [125865961, 126855169, 158659216, 215619856, 526518916, 552861169, 652598116],
    "244458999": [458944929, 995844249                                                       ],
    "122346778": [163277284, 216737284, 226773481, 271326784                                 ],
    "112247899": [114982729, 212197489, 749281129                                            ]
}
```

# Mappings

The mappings between letters and digits is not unique, we can freely chose those. Hence we need to normalize one step further to get the structure. We count how many of the same letter or digit is in there. Then we take these counts and sort them, to normalize all permutations. Then we join these counts with a separator such that they give unique keys.

For instance for `11123` this would be come `1-2-3` because there is one group of length 1, one group of length 2 and another of length 3. For `222559` we get the same key. Something like `13578` would just become `1-1-1-1-1`.

We then get another mapping that groups anagram squares classes by their digits. We get the following:

```json
{
    "1-1-1-1": ["1369",      "1269",      "1467"                               ],
    "1-1-1-6": ["111111268"                                                    ],
    "1-1-2-5": ["113444449", "467777799", "166666779", "111114667", "111115669"]
}
```

You can see for instance that `113444449` and `467777799` differ in their digit content but not in the structure.

# Matching

The matching now works like this:

- We iterate through all the word classes with their key (like `ABDOR`).
- We normalize that key to `1-1-1-1-1`, there are five distinct letters in there, each occurring once. `EEHST` would become `1-1-1-2` (number ordered for uniqueness).
- We take a look into the square classes that have the same pattern. There we have `["34569", "13468", "12679", "24579", "14589", "24589", "12367", "16789"]`.
- We go though all of these classes, let's pick `16789` as an example.
- Inside that class we find the squares `[17689, 18769, 78961, 81796]`.
- We go through all the pairs of words in our word class, "SHEET" and "THESE".
- We go through all the pairs of primes in the chosen square class.
- We see how the first word maps its letters onto the first prime and the second word onto the second prime.
- If these mappings match, then we have found square anagrams!

Doing this for all possible combinations gives the following pairs:

| Anagram Pair | Square Pair | Letter to Digit Mapping |
| --- | --- | --- |
| BROAD - BOARD | **18769** - 17689 | `{'A': '6', 'B': '1', 'D': '9', 'O': '7', 'R': '8'}` |
| RACE - CARE | 9216 - 1296 | `{'A': '2', 'C': '1', 'E': '6', 'R': '9'}` |
| LEAD - DEAL | 4761 - 1764 | `{'A': '6', 'D': '1', 'E': '7', 'L': '4'}` |
| HEAT - HATE | 1936 - 1369 | `{'A': '3', 'E': '9', 'H': '1', 'T': '6'}` |
| MEAL - MALE | 1936 - 1369 | `{'A': '3', 'E': '9', 'L': '6', 'M': '1'}` |
| TEA - EAT | 625 - 256 | `{'A': '5', 'E': '2', 'T': '6'}` |
| GOD - DOG | 961 - 169 | `{'D': '1', 'G': '9', 'O': '6'}` |
| LIFE - FILE | 9216 - 1296 | `{'E': '6', 'F': '1', 'I': '2', 'L': '9'}` |
| TONE - NOTE | 9216 - 1296 | `{'E': '6', 'N': '1', 'O': '2', 'T': '9'}` |
| WHO - HOW | 625 - 256 | `{'H': '2', 'O': '5', 'W': '6'}` |
| THUS - SHUT | 4761 - 1764 | `{'H': '7', 'S': '1', 'T': '4', 'U': '6'}` |
| SIT - ITS | 625 - 256 | `{'I': '2', 'S': '6', 'T': '5'}` |
| OWN - NOW | 961 - 196 | `{'N': '1', 'O': '9', 'W': '6'}` |

The desired answer is the largest square number that occurs in any of these pairs.