@0x8b59233dc96a3506;

enum Direction {
  left @0;
  right @1;
}

struct SwipeEvent {
  direction @0 :Direction;
}

interface SwipeListener {
  onSwipe @0 (event :SwipeEvent) -> ();
}


interface SwipeService {
  pushSwipe @0 (event :SwipeEvent) -> ();
  subscribe @1 (listener :SwipeListener) -> ();
}