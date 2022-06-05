Maximize
  obj: delta_b - delta_a

Subject To
  qc0: [ -0.997 delta_a * delta_b ] - 3.0 delta_a + 3.998 delta_b = 0
  qc1: [ -0.997 delta_b * delta_c ] - 3.0 delta_b + 3.998 delta_c = 0

Bounds
  0 <= delta_b
  0 <= delta_a
  0 <= delta_c

Generals		
	delta_a delta_b delta_c

