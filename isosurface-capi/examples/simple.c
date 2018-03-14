#include <stdio.h>
#include "isosurface.h"

int main() {
    /* Function depending only on x variable. */
    double u[8] = {0., 0., 0., 0., 1., 1., 1., 1.};

    isosurface *iso = marching_tetrahedra(u, 2, 2, 2, 0.5);

    size_t n_verts = iso_num_verts(iso);
    size_t n_faces = iso_num_faces(iso);

    printf("Generated %lu vertices and %lu faces (triangles)\n", n_verts, n_faces);

    double (*verts)[3] = iso_verts(iso);
    double (*normals)[3] = iso_normals(iso);
    uint32_t (*faces)[3] = iso_faces(iso);

    printf("verts:\n");
    for (int i = 0; i < n_verts; ++i) {
        printf("[%f, %f, %f], ", verts[i][0], verts[i][1], verts[i][2]);
    }
    printf("\n");

    printf("normals:\n");
    for (i = 0; i < n_verts; ++i) {
        printf("[%f, %f, %f], ", normals[i][0], normals[i][1], normals[i][2]);
    }
    printf("\n");

    printf("faces:\n");
    for (i = 0; i < n_faces; ++i) {
        printf("[%u, %u, %u], ", faces[i][0], faces[i][1], faces[i][2]);
    }
    printf("\n");

    /* Don't forget to free resources after every call to marching_tetrahedra! */
    iso_free(iso);

    return 0;
}
