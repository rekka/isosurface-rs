#ifndef _ISOSURFACE_H
#define _ISOSURFACE_H

#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif

/*
 * Type returned by marching_tetrahedra.
 */
typedef struct isosurface isosurface;

/*
 * marching_tetrahedra constructs an isosurface (level set) of a function u
 * given by values on a regular grid at a given level using the marching
 * tetrahedra algorithm.
 *
 * u is a three-dimensional array in C-order of function values on a regular grid
 * of ni * nj * nk nodes located at integer coordinates
 *      x = 0, .., ni - 1,
 *      y = 0, .., nj - 1,
 *      z = 0, .., nk - 1.
 * The x coordinate is changing the slowest and z the fastest.
 *
 * level chooses the level set of u.
 */
isosurface *marching_tetrahedra(const double *u, size_t ni, size_t nj, size_t nk, double level);

/*
 * iso_num_verts returns the number of generated vertices.
 */
size_t iso_num_verts(const isosurface *iso);

/*
 * iso_num_faces returns the number of generated faces.
 */
size_t iso_num_faces(const isosurface *iso);

/*
 * iso_verts returns a pointer to the array of generated verteces.
 *
 * Length of the array is returned by iso_num_verts.
 *
 * Each vertex is a double[3] array of (x, y, z) coordinates.
 */
double (*iso_verts(const isosurface *iso))[3];

/*
 * iso_normals returns a pointer to the array of generated normals.
 *
 * Length of the array is returned by iso_num_verts.
 *
 * Every vertex has exactly one normal.
 *
 * Each normal is a double[3] array of (x, y, z) coordinates.
 */
double (*iso_normals(const isosurface *iso))[3];

/*
 * iso_faces returns a pointer to the array of generated triangular faces.
 *
 * Length of the array is returned by iso_num_faces.
 *
 * Each face is a uint32_t[3] array of 3 indexes of vertices in the vertex array.
 */
uint32_t (*iso_faces(const isosurface *iso))[3];

/*
 * Must be called at most once for any isosurface.
 */
void iso_free(isosurface *iso);

#ifdef __cplusplus
}
#endif

#endif
