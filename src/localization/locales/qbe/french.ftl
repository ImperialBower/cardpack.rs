# qbe/french.ftl — Belter Creole (Lang Belta) translation
#
# Status: DRAFT — LOW CONFIDENCE. Lang Belta (the constructed creole from
# *The Expanse* novels and TV adaptation, designed by Nick Farmer) has zero
# published playing-card vocabulary. This file uses a loanword-first /
# coinage-fallback approach: Lang Belta's source languages (Spanish,
# English, French, Hindi, Mandarin, Russian, German, Arabic) supply
# loanwords for terms that exist in those source languages, and coinages
# follow Lang Belta phonology (Latin transcription, `x` for sh-sounds,
# `k` over `c`, no diacritics).
#
# Notation: # loanword: word adopted from a Lang Belta source language
#                       (Spanish, English, French, Hindi, etc.).
#           # coinage : compound or extension following Lang Belta phonology;
#                       back-translation given.
#
# `qbe` locale code: Lang Belta has no registered ISO/BCP-47 code. We use
# `qbe` from the qaa–qtz private-use ISO 639 range because `unic-langid`
# rejects extension forms like `art-x-belta`. See src/localization.rs for
# the full reasoning.
#
# Needs: a Lang Belta enthusiast / *Expanse* linguistics fan (Nick Farmer
# notes, fan wikis, *The Expanse* RPG glossary) for review.

# loanword: Spanish "espada" with initial-vowel elision per Lang Belta phonology
name-suit-french-s = spada
# loanword: Spanish "corazón" (heart)
name-suit-french-h = korazon
# loanword: Spanish/French "diamante"/"diamant", apocope
name-suit-french-d = diamant
# loanword: French "trèfle" simplified to Lang Belta phonology
name-suit-french-c = trefa
# loanword: English "joker" in Lang Belta orthography (k over c)
name-suit-french-j = joka
# loanword: Spanish "nada" (nothing)
name-suit-french-_ = nada

# coinage: "brother joker" (joka + beratna "brother", attested); for big-joker
name-rank-french-b = joka beratna
# coinage: "tiny joker" (joka + tikitik phonosemantic diminutive); for one-color joker
name-rank-french-l = joka tikitik
# coinage: numeral 1 used as semantic Ace; parallels la/fr Ace patterns
name-rank-french-a = wa
# loanword: English/Spanish "captain"/"capitán" → leader/King
name-rank-french-k = kapten
# loanword: Spanish "dama" (lady) → Queen
name-rank-french-q = dama
# loanword: French "valet", parallels fr/french.ftl
name-rank-french-j = valet
# coinage: numeral 10 (Hindi-influenced; Lang Belta numerals are multi-source)
name-rank-french-t = das
# coinage: numeral 9
name-rank-french-9 = nai
# coinage: numeral 8
name-rank-french-8 = ot
# coinage: numeral 7
name-rank-french-7 = set
# coinage: numeral 6
name-rank-french-6 = sik
# coinage: numeral 5
name-rank-french-5 = fi
# coinage: numeral 4
name-rank-french-4 = fo
# coinage: numeral 3 (Spanish-influenced)
name-rank-french-3 = tre
# coinage: numeral 2
name-rank-french-2 = du
# loanword: Spanish "nada" (nothing)
name-rank-french-_ = nada

# Deprecated Values
big-joker-long = joka beratna
