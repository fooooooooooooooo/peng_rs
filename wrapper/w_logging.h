#pragma once

#include "core/logger.h"

extern "C" {
  void w_log(LogSeverity severity, char *s);
}
