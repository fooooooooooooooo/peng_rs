#include "w_vector2.h"

#define Vec2Impl(N, T)               \
  Vector2##N vector_2##N(T x, T y) { \
    return Vector2##N {x, y};        \
  }

extern "C" {
  Vec2Impl(f, float);
  Vec2Impl(d, double);
  Vec2Impl(i, int32_t);
  Vec2Impl(u, uint32_t);
  Vec2Impl(u8, uint8_t);
}
