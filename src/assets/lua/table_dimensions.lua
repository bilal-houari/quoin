function Table(el)
  local new_colspecs = {}
  for _, spec in ipairs(el.colspecs) do
    table.insert(new_colspecs, {spec[1], nil})
  end
  el.colspecs = new_colspecs
  return el
end
