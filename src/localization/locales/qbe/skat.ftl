# qbe/skat.ftl — Belter Creole (Lang Belta) translation
#
# Status: DRAFT — LOW CONFIDENCE. As with qbe/french.ftl, Lang Belta canon
# has no playing-card vocabulary. Skat-specific terminology compounds the
# gap: there is no Belter analogue of the German Skat tradition. Suits use
# Spanish-derived loanwords matching the german-meaning targets (Eichel /
# Laub / Herz / Schellen → acorn / leaf / heart / bell). Rank choices reuse
# numerals from french.ftl plus directional Spanish loans for Ober/Unter.
#
# Notation: same as qbe/french.ftl (# loanword / # coinage).
#
# Specific concerns:
#   - rank-d ("Daus") is the HIGH trump in Skat. Mapped to `wa` (numeral 1
#     used as semantic Ace), matching en-US/de/fr/la/tlh choices.
#   - rank-o (Ober) and rank-u (Unter) use `alto` / `bajo` — Spanish
#     directional loans semantically exact for "upper" / "lower" rank,
#     parallel to the la/skat.ftl Superior/Inferior choice and tlh/skat.ftl
#     Dung/bIng pair.
#
# Needs: Lang Belta enthusiast with Skat awareness.

# loanword: Spanish "bellota" (acorn); for Acorns (Eichel)
name-suit-skat-e = bellota
# loanword: Spanish "hoja" (leaf), Lang Belta phonology; for Leaves (Laub)
name-suit-skat-l = oja
# loanword: Spanish "corazón"; matches qbe/french.ftl; for Hearts (Herz)
name-suit-skat-h = korazon
# loanword: Spanish "campana" (bell), Lang Belta orthography (k over c); for Bells (Schellen)
name-suit-skat-s = kampana
# loanword: Spanish "nada"
name-suit-skat-_ = nada

# coinage: numeral 1 used as semantic Ace; for Skat's high-trump Daus
name-rank-skat-d = wa
# coinage: numeral 10 (Hindi-influenced); for Ten
name-rank-skat-z = das
# loanword: English/Spanish "captain"; for King
name-rank-skat-k = kapten
# loanword: Spanish "alto" (high); for Ober (semantic match: upper rank)
name-rank-skat-o = alto
# loanword: Spanish "bajo" (low); for Unter (semantic match: lower rank)
name-rank-skat-u = bajo
# coinage: numeral 9
name-rank-skat-9 = nai
# coinage: numeral 8
name-rank-skat-8 = ot
# coinage: numeral 7
name-rank-skat-7 = set
