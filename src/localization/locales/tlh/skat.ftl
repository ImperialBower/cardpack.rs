# tlh/skat.ftl — Klingon (tlhIngan Hol) translation
#
# Status: DRAFT — LOW CONFIDENCE. As with tlh/french.ftl, Klingon canon has
# no playing-card vocabulary. Skat-specific terminology (Eichel/Laub/Herz/
# Schellen, Daus, Ober/Unter) compounds the gap. Suit names are coinages
# from attested roots; rank names use attested directional nouns (Dung/bIng)
# for Ober/Unter — semantically exact for the German "upper"/"lower" sense.
#
# Notation: same as tlh/french.ftl (# attested / # coinage).
#
# Specific concerns:
#   - rank-d ("Daus") is the HIGH trump in Skat. Mapped to wa'DIch
#     (first/Ace) for semantic accuracy, matching fr/ and la/ choices.
#   - rank-o (Ober) and rank-u (Unter) use Dung (above) and bIng (below) —
#     attested directional nouns that read as "upper" / "lower" rank,
#     parallel to the la/skat.ftl Superior/Inferior choice.
#
# Needs: KLI-savvy reviewer with Skat awareness.

# coinage: "tree fruit" (Sor "tree" + naH "fruit"); for Acorns (Eichel)
name-suit-skat-e = Sor naH
# coinage: "tree material" (Sor "tree" + Hap "matter"); for Leaves (Laub)
name-suit-skat-l = Sor Hap
# attested: "heart" (TKD); for Hearts (Herz)
name-suit-skat-h = tIq
# coinage: "small noise" (wab "noise" + -Hom diminutive); for Bells (Schellen)
name-suit-skat-s = wabHom
# attested: "zero, nothing" (TKD)
name-suit-skat-_ = pagh

# attested: "first" ordinal (TKD); semantic Ace for Skat's high trump Daus
name-rank-skat-d = wa'DIch
# attested: "ten" (TKD)
name-rank-skat-z = wa'maH
# attested: "emperor" (TKD); for King
name-rank-skat-k = ta'
# attested: "above, area above" (TKD); for Ober (semantic match: upper rank)
name-rank-skat-o = Dung
# attested: "below, area below" (TKD); for Unter (semantic match: lower rank)
name-rank-skat-u = bIng
# attested: "nine" (TKD)
name-rank-skat-9 = Hut
# attested: "eight" (TKD)
name-rank-skat-8 = chorgh
# attested: "seven" (TKD)
name-rank-skat-7 = Soch
