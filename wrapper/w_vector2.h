#pragma once

#include <cstdint>

#define Vec2(N, T)    \
  struct Vector2##N { \
    T x;              \
    T y;              \
  }

Vec2(f, float);
Vec2(d, double);
Vec2(i, int32_t);
Vec2(u, uint32_t);
Vec2(u8, uint8_t);

#define Vec2Def(N, T) Vector2##N vector_2##N(T x, T y)

extern "C" {
  Vec2Def(f, float);
  Vec2Def(d, double);
  Vec2Def(i, int32_t);
  Vec2Def(u, uint32_t);
  Vec2Def(u8, uint8_t);
}
