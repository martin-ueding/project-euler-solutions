# Anagramic Squares (98)

In Problem 98, we're asked to work anagrams and substitute letters for digits to convert words into numbers such that all of them are perfect squares.

## Looking at the words

The file that we're given starts like this:

```csv
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