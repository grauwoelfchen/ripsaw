NR == 1 { header = $0; next }
NR % l == 2 { close(x); N++; x = p N s; print header > x }
{ print > x }
