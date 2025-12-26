#pragma once

#include <vtkFollower.h>
#include <vtkCamera.h>
#include <vtkMapper.h>
#include <cxx.h>
#include <cstdint>

vtkFollower* vtk_follower_new();
void vtk_follower_delete(vtkFollower& ptr);

void follower_set_camera(vtkFollower& follower, uintptr_t camera);

void follower_set_mapper(vtkFollower& follower, uintptr_t mapper);

void follower_set_position(vtkFollower& follower, double x, double y, double z);
void follower_get_position(vtkFollower& follower, double& x, double& y, double& z);

void follower_set_scale(vtkFollower& follower, double x, double y, double z);
void follower_get_scale(vtkFollower& follower, double& x, double& y, double& z);
