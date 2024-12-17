# Jour 17 AOC

## Analyse
- extraction des valeurs
- extraction des registres

## Recherche
- je regarde ce qu'il se passe juste sur le premier output
- Je cherche à obtenir un output == 2
- Donc je peux trouver les A potentiels pour lesquels le output est 2


![recherche](./assets/day_17_2.png)

![recherche](./assets/day_17.png)

## Optimisations apprises

- Remplacer ``x % 8`` par ``x & 7``
- Remplacer ``a // (2 ** b2)`` par ``a >> b2``
- Remplacer ``b_val % 8`` par ``b_val & 7``

