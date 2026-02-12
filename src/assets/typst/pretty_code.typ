#show raw.where(block: false): box.with(fill: luma(240), inset: (x: 3pt), outset: (y: 3pt), radius: 3pt)

#show raw.where(block: true): it => {
  set text(size: 0.9em)
  align(center)[
    #block(fill: luma(250), inset: 1em, radius: 5pt, stroke: luma(220))[#it]
  ]
}
