#include "w_logging.h"

extern "C" {
  void w_log(LogSeverity severity, char *s) {
    Logger::get().logf(severity, s);
  }
}
