# Jour 17 AOC

## Partie 2 : Recherche du chemin inverse

### Contexte
- Objectif : Remonter le cycle pour trouver une valeur initiale A qui valide une série de positions (mon programme == outputs).
- Principe : Chaque cycle est décalé de 8 bits (équivalent à une multiplication par 2^3 ou 8)
- Approche :
  - Partir de la fin du programme (ex : [5, 5, 3, 0]).
  - Rechercher des candidats A potentiels.
  - Décaler les candidats de 8 bits pour trouver les valeurs du cycle précédent. 
  - Répéter jusqu'à remonter à l'origine (0, puis 3, puis 5, ...).

### Objectif
![recherche](./assets/day_17.png)

### Approfondissement
Pour améliorer la performance utilisation d'opérateur [bitwise](https://wiki.python.org/moin/BitwiseOperators) :
- Remplacer ``x % 8`` par ``x & 7``
- Remplacer ``a // (2 ** b2)`` par ``a >> b2``
- Remplacer ``b_val % 8`` par ``b_val & 7``


### Réfléxions premières
Je ne sais pas comment faire j'essai d'analiser le cycle qui revient X * output.length
![recherche](./assets/day_17_2.png)

### Après apprfondissement
Je part de la fin du programme, et on recherche le A possible.
<br>
Sachant qu'entre chaque cycle on décale de 8 bits (multiplier environs par 2^3  ou 8)
<br>
Donc j'ajoute le A dans les candidats qui lui devras être décaler de 8 bits et de 0 à 2^3 - 1, afin de trouver le A du cycle d'avant.


![recherche](./assets/day_17_3.png)

Exemple script fonctionnel :
<br>
Mon programme : ``[2, 4, 1, 3, 7, 5, 1, 5, 0, 3, 4, 2, 5, 5, 3, 0]``
<br>
```terminal
------
Pour [0]
Candidat : 0
A valid pour [0] == [0] -> 6
------
Pour [3, 0]
Candidat : 6
A valid pour [3, 0] == [3, 0] -> 49
A valid pour [3, 0] == [3, 0] -> 53
------
Pour [5, 3, 0]
Candidat : 49
A valid pour [5, 3, 0] == [5, 3, 0] -> 393
A valid pour [5, 3, 0] == [5, 3, 0] -> 397
Candidat : 53
A valid pour [5, 3, 0] == [5, 3, 0] -> 425
A valid pour [5, 3, 0] == [5, 3, 0] -> 429
A valid pour [5, 3, 0] == [5, 3, 0] -> 430
------
Pour [5, 5, 3, 0]
Candidat : 393
A valid pour [5, 5, 3, 0] == [5, 5, 3, 0] -> 3145
A valid pour [5, 5, 3, 0] == [5, 5, 3, 0] -> 3151
Candidat : 397
A valid pour [5, 5, 3, 0] == [5, 5, 3, 0] -> 3177
Candidat : 425
A valid pour [5, 5, 3, 0] == [5, 5, 3, 0] -> 3401
A valid pour [5, 5, 3, 0] == [5, 5, 3, 0] -> 3407
Candidat : 429
A valid pour [5, 5, 3, 0] == [5, 5, 3, 0] -> 3433
Candidat : 430
A valid pour [5, 5, 3, 0] == [5, 5, 3, 0] -> 3442
------
etc...
```


# Conclusion
Approche principale : Remonter les cycles en utilisant des décalages de bits et des masques binaires.
<br>
Optimisations : Utilisation d'opérations & (AND binaire) et >> (shift) pour améliorer les performances.
<br>
A faire : Plus appronfondir sur les binaires et les décalages de bits.
