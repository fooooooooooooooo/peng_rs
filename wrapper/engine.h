#pragma once

#include "../PengEngine/src/core/peng_engine.h"
#include "core/asset.h"
#include "rendering/window_icon.h"
#include "scene/scene_loader.h"
#include "rendering/window_subsystem.h"

#pragma clang diagnostic push
#pragma ide diagnostic ignored "OCUnusedGlobalDeclarationInspection"

extern "C" {
PengEngine *create_engine();

void engine_run(PengEngine *engine);

void engine_set_target_frametime(PengEngine *engine, float frametime_ms);

void on_engine_initialized_once(PengEngine *engine, void (listener)(void));

Asset<rendering::WindowIcon> *create_icon(const char *path);

const rendering::WindowIcon *load_icon(Asset<rendering::WindowIcon> *icon);

void use_icon(rendering::WindowIcon *icon);

scene::SceneLoader load_scene_from_file(const char *path);

rendering::WindowSubsystem *get_window_subsystem();

void window_set_resolution(rendering::WindowSubsystem *window, math::Vector2i resolution);

void window_set_vsync(rendering::WindowSubsystem *window, bool vsync);

math::Vector2i vector_2i(int x, int y);
}

#pragma clang diagnostic pop
