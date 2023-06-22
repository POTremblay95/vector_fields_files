# vector_fields_files

This project will be to be able to create vector fields files. Here are the supported format :

  -.fga : This stands for Fluid Grid ASCII (for Unreal Engine)
  -.vf : This stands for Volume File (For Unity)


## FGA file format

This format is defined as follows :

resolution_x, resolution_y, resolution_x  # Cell count in X,Y and Z axis (line 1)
min_x, min_y, min_z # Bounding box minimum (min_coord) (line 2)
max_x, max_y, max_z # Bounding box maximum (max_coord) (line 3)
vec1_x, vec1_y, vec1_z # Vector in first cell (line 4)
...
vecn_x, vecn_y, vecn_z # Vector in last cell (line 3+n)


## VF file format

This format is defined as follows :

