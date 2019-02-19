.global mutex_lock
mutex_lock:
  li t0, 1

again:
  amoswap.w.aq t0, t0, (a0)
  bnez t0, again
