# qbe/tarot.ftl — Belter Creole (Lang Belta) translation
#
# Status: DRAFT — LOW CONFIDENCE. Tarot's Italian/European mystical tradition
# has no Lang Belta cultural analog (Belters are blue-collar asteroid
# miners, not occult initiates). Most entries lean on Spanish-derived
# loanwords because Lang Belta has a strong Spanish substrate; coinages
# follow the same loanword-first / coinage-fallback policy as qbe/french.ftl
# and qbe/skat.ftl.
#
# Notation: same as qbe/french.ftl (# loanword / # coinage).
#
# Schema note: follows the en-US `name-rank-tarot-special-` prefix for
# Major Arcana — this prefix was a v0.7.0 schema fix; missing it causes
# silent fallback to English. See src/localization.rs:535-547.
#
# Heavy coinage zones (most likely to need revision on review):
# Magician, High Priestess, Hierophant, Wheel of Fortune, Hanged Man,
# Temperance, Judgement.
#
# Needs: Lang Belta enthusiast with Tarot familiarity. This is the most
# speculative file in qbe/; promotion may require alternative Spanish
# loanwords or genuinely Belter-flavored coinages.

# coinage: "great magic" (Spanish "magia" + "grande"); for Major Arcana
name-suit-tarot-m = magia grande
# loanword: Spanish "palo" (stick/staff); for Wands
name-suit-tarot-w = palo
# loanword: Spanish "copa" with Lang Belta orthography (k over c); for Chalices
name-suit-tarot-c = kopa
# loanword: Spanish "espada" with initial-vowel elision; matches qbe/french.ftl; for Swords
name-suit-tarot-s = spada
# loanword: Spanish "oro" (gold); coinage in card context (Pentacles ≡ historical Denari/coins)
name-suit-tarot-p = oro
# loanword: Spanish "nada"
name-suit-tarot-_ = nada

# Major Arcana — heavy loanword + some coinages
# loanword: Spanish "tonto" (fool); for The Fool
name-rank-tarot-special-0 = tonto
# loanword: Spanish "mago" (magician); for The Magician
name-rank-tarot-special-1 = mago
# coinage: "holy mother" (Spanish "madre" + "santa"); for The High Priestess
name-rank-tarot-special-2 = madre santa
# loanword: Spanish "emperatriz", Lang Belta phonology; for The Empress
name-rank-tarot-special-3 = imperatriza
# loanword: Latin/Spanish "imperator"; for The Emperor
name-rank-tarot-special-4 = imperator
# coinage: "holy father" (Spanish "padre" + "santo"); for The Hierophant
name-rank-tarot-special-5 = padre santo
# loanword: Spanish "amantes" (lovers); for The Lovers
name-rank-tarot-special-6 = amantes
# loanword: Spanish "carro" with Lang Belta orthography; for The Chariot
name-rank-tarot-special-7 = karro
# loanword: Spanish "fuerza" (strength), Lang Belta phonology; for Strength
name-rank-tarot-special-8 = forsa
# loanword: Spanish "solo" (alone); coinage in card context; for The Hermit
name-rank-tarot-special-9 = solo
# coinage: "fortune wheel" (Spanish "rueda" + "fortuna"); for Wheel of Fortune
name-rank-tarot-special-a = rueda fortuna
# loanword: Spanish "justicia"; for Justice
name-rank-tarot-special-b = justicia
# loanword: Spanish "colgado" (hanged), Lang Belta orthography (k over c); for The Hanged Man
name-rank-tarot-special-c = kolgado
# loanword: Spanish "muerte"; for Death
name-rank-tarot-special-d = muerte
# coinage: "tempering" (Spanish "templanza" simplified to Lang Belta); for Temperance
name-rank-tarot-special-e = tempera
# loanword: Spanish "diablo"; for The Devil
name-rank-tarot-special-f = diablo
# loanword: Spanish "torre"; for The Tower
name-rank-tarot-special-g = torre
# loanword: Spanish "estrella" with initial-vowel elision; for The Star
name-rank-tarot-special-h = strella
# loanword: Spanish "luna"; for The Moon
name-rank-tarot-special-i = luna
# loanword: Spanish "sol"; for The Sun
name-rank-tarot-special-j = sol
# loanword: Spanish "juicio"; for Judgement
name-rank-tarot-special-k = juicio
# loanword: Spanish "mundo"; for The World
name-rank-tarot-special-l = mundo
# loanword: Spanish "nada"
name-rank-tarot-special-_ = nada

# Minor Arcana — court cards plus numeric ranks
# loanword: English/Spanish "captain"; matches qbe/french.ftl; for King
name-rank-tarot-k = kapten
# loanword: Spanish "dama"; matches qbe/french.ftl; for Queen
name-rank-tarot-q = dama
# loanword: Spanish "caballero" with Lang Belta orthography; for Knight
name-rank-tarot-n = kabaiero
# loanword: Spanish "paje" (page/servant); for Page
name-rank-tarot-p = paje
# coinage: numeral 10 (Hindi-influenced); for Ten
name-rank-tarot-t = das
# coinage: numeral 9
name-rank-tarot-9 = nai
# coinage: numeral 8
name-rank-tarot-8 = ot
# coinage: numeral 7
name-rank-tarot-7 = set
# coinage: numeral 6
name-rank-tarot-6 = sik
# coinage: numeral 5
name-rank-tarot-5 = fi
# coinage: numeral 4
name-rank-tarot-4 = fo
# coinage: numeral 3
name-rank-tarot-3 = tre
# coinage: numeral 2
name-rank-tarot-2 = du
# coinage: numeral 1 used as semantic Ace
name-rank-tarot-a = wa
# loanword: Spanish "nada"
name-rank-tarot-_ = nada
