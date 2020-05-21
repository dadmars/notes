# 查看数据

```bash
curl localhost:8080/query -XPOST -d '
{
    me(func: uid(0x1)) {
        name
        uid
        equip {
            name
            uid
        }
        node {
            expand(_all_)
        }
    }
}' | python -m json.tool | less
```

# 备份

## 备份数据库

```bash
container id: xxxxx

docker exec -it xxxxx /bin/bash
curl localhost:8080/admin/export
ls export
exit

docker cp xxxxx:/dgraph/export/dgraph-1-2019-08-13-09-52.rdf.gz .
docker cp xxxxx:/dgraph/export/dgraph-1-2019-08-13-09-52.schema.gz .
```

## 还原数据库

```bash
docker cp dgraph-1-2019-08-13-09-52.rdf.gz xxxxx:/dgraph/
docker cp dgraph-1-2019-08-13-09-52.schema.gz xxxxx:/dgraph/

# get zeroip
docker network ls
docker network inspect xxx

docker exec -it xxxxx /bin/bash
dgraph live -r <path-to-rdf-gzipped-file> -s <path-to-schema-file> -z zeroip:5080
rm -f *.gz
exit
```

# 主从同步

```bash
--replicas
    we recommend setting --replicas to 1, 3 or 5 (not 2 or 4). This allows 0, 1, or 2 nodes serving the same group to be down, respectively without affecting the overall health of that group.
```

# 查询语法

## eq

Matches strings that have all specified terms in any order; 不分大小写
* Schema Types: string
* Index Required: term

```bash
me(func: eq(name, "Steven Spielberg")) @filter(has(director.film)) {
  name
  director.film @filter((name, "jones indiana"))  {
    name
  }
}
```
              
## anyofterms

Matches strings that have any of the specified terms in any order; case insensitive.
* Schema Types: string
* Index Required: term

Query Example: All nodes that have a name containing either poison or peacock. 

```bash
me(func:anyofterms(name@en, "poison peacock")) {
    name@en
    genre {
      name@en
    }
}
              
me(func: eq(name@en, "Steven Spielberg")) @filter(has(director.film)) {
    name@en
    director.film @filter(anyofterms(name@en, "war spies"))  {
      name@en
    }
}
```
              
## Regular Expressions

regexp(predicate, /regular-expression/) or case insensitive regexp(predicate, /regular-expression/i)
* Schema Types: string
* Index Required: trigram

```bash
curl localhost:8080/query -XPOST -d '
{
  directors(func: regexp(name@en, /^Steven Sp.*$/)) {
    name@en
    director.film @filter(regexp(name@en, /ryan/i)) {
      name@en
    }
  }
}' | python -m json.tool | less
```
              
A Trigram is a substring of three continuous runes. For example, Dgraph has trigrams Dgr, gra, rap, aph.

At least one trigram must be matched by the regular expression (patterns shorter than 3 runes are not supported). That is, Dgraph requires regular expressions that can be converted to a trigram query.

The number of alternative trigrams matched by the regular expression should be as small as possible ([a-zA-Z][a-zA-Z][0-9] is not a good idea). Many possible matches means the full regular expression is checked against many strings; where as, if the expression enforces more trigrams to match, Dgraph can make better use of the index and check the full regular expression against a smaller set of possible matches.

Thus, the regular expression should be as precise as possible. Matching longer strings means more required trigrams, which helps to effectively use the index.

If repeat specifications (*, +, ?, {n,m}) are used, the entire regular expression must not match the empty string or any string: for example, * may be used like [Aa]bcd* but not like (abcd)* or (abcd)|((defg)*)

Repeat specifications after bracket expressions (e.g. [fgh]{7}, [0-9]+ or [a-z]{3,5}) are often considered as matching any string because they match too many trigrams.

If the partial result (for subset of trigrams) exceeds 1000000 uids during index scan, the query is stopped to prohibit expensive queries.

## Full Text Search

alloftext(predicate, "space-separated text") and anyoftext(predicate, "space-separated text")
* Schema Types: string
* Index Required: fulltext

```bash
{ movie(func:alloftext(name@en, “the man maybe runs”)) { name@en } }
```

equal to

```bash
eq(predicate, value)
eq(val(varName), value)
eq(predicate, val(varName))
eq(count(predicate), value)
eq(predicate, [val1, val2, ..., valN])
```

## Schema Types: int, float, bool, string, dateTime

Type 	Index Options
int 	int
float 	float
bool 	bool
string 	exact, hash
dateTime 	dateTime

```bash
curl localhost:8080/query -XPOST -d '
{
  me(func: eq(count(genre), 13)) {
    name@en
    genre {
    	name@en
    }
  }
}' | python -m json.tool | less

curl localhost:8080/query -XPOST -d '
{
  steve as var(func: (name@en, "Steven")) {
    films as count(director.film)
  } 

  stevens(func: uid(steve)) @filter(eq(val(films), [1,2,3])) {
    name@en
    numFilms : val(films)
  }
}' | python -m json.tool | less
```

## less than, less than or equal to, greater than and greater than or equal to

Syntax Examples: for inequality IE

    IE(predicate, value)
    IE(val(varName), value)
    IE(predicate, val(varName))
    IE(count(predicate), value)

With IE replaced by

    le less than or equal to
    lt less than
    ge greater than or equal to
    gt greather than

```bash
curl localhost:8080/query -XPOST -d '
{
  me(func: eq(name@en, "Ridley Scott")) {
    name@en
    director.film @filter(lt(initial_release_date, "1980-01-01"))  {
      initial_release_date
      name@en
    }
  }
}' | python -m json.tool | less
              
curl localhost:8080/query -XPOST -d '
{
  ID as var(func: (name@en, "Steven")) {
    director.film {
      num_actors as count(starring)
    }
    total as sum(val(num_actors))
  }

  dirs(func: uid(ID)) @filter(gt(val(total), 100)) {
    name@en
    total_actors : val(total)
  }
}' | python -m json.tool | less
              
curl localhost:8080/query -XPOST -d '
{
  genre(func: gt(count(~genre), 30000)){
    name@en
    ~genre (first:1) {
      name@en
    }
  }
}' | python -m json.tool | less
              
curl localhost:8080/query -XPOST -d '
{
  var(func: eq(name@en,"Minority Report")) {
    d as initial_release_date
  }

  me(func: eq(name@en, "Steven Spielberg")) {
    name@en
    director.film @filter(ge(initial_release_date, val(d))) {
      initial_release_date
      name@en
    }
  }
}' | python -m json.tool | less
```

## uid

    q(func: uid(<uid>))
    predicate @filter(uid(<uid1>, ..., <uidn>))
    predicate @filter(uid(a)) for variable a
    q(func: uid(a,b)) for variables a and b

```bash
curl localhost:8080/query -XPOST -d '
{
  films(func: uid(0xcceb)) {
    name@hi
    actor.film {
      performance.film {
        name@hi
      }
    }
  }
}' | python -m json.tool | less

curl localhost:8080/query -XPOST -d '
{
  var(func: (name@en, "Taraji Henson")) {
    actor.film {
      F as performance.film {
        G as genre
      }
    }
  }

  Taraji_films_by_genre(func: uid(G)) {
    genre_name : name@en
    films : ~genre @filter(uid(F)) {
      film_name : name@en
    }
  }
}' | python -m json.tool | less
              
curl localhost:8080/query -XPOST -d '
{
  var(func: (name@en, "Taraji Henson")) {
    actor.film {
      F as performance.film {
        G as count(genre)
        genre {
          C as count(~genre @filter(uid(F)))
        }
      }
    }
  }

  Taraji_films_by_genre_count(func: uid(G), orderdesc: val(G)) {
    film_name : name@en
    genres : genre (orderdesc: val(C)) {
      genre_name : name@en
    }
  }
}' | python -m json.tool | less
```
              
## uid_in

    q(func: ...) @filter(uid_in(predicate, <uid>)
    predicate1 @filter(uid_in(predicate2, <uid>)

Schema Types: UID
Index Required: none

While the uid function filters nodes at the current level based on UID, function uid_in allows looking ahead along an edge to check that it leads to a particular UID. This can often save an extra query block and avoids returning the edge.

uid_in cannot be used at root, it accepts one UID constant as it’s argument (not a variable).

Query Example: The collaborations of Marc Caro and Jean-Pierre Jeunet (UID 597046). If the UID of Jean-Pierre Jeunet is known, querying this way removes the need to have a block extracting his UID into a variable and the extra edge traversal and filter for ~director.film.

```bash
curl localhost:8080/query -XPOST -d '
{
  caro(func: eq(name@en, "Marc Caro")) {
    name@en
    director.film @filter(uid_in(~director.film, 597046)){
      name@en
    }
  }
}' | python -m json.tool | less
```

## has

Determines if a node has a particular predicate.

```bash
curl localhost:8080/query -XPOST -d '
{
  me(func: has(director.film), first: 5) {
    name@en
    director.film @filter(has(initial_release_date))  {
      initial_release_date
      name@en
    }
  }
}' | python -m json.tool | less
```
              
## AND, OR and NOT

(NOT A OR B) AND (C AND NOT (D OR E)). Note that, NOT binds more tightly than AND which binds more tightly than OR.

```bash
curl localhost:8080/query -XPOST -d '
{
  me(func: eq(name@en, "Steven Spielberg")) @filter(has(director.film)) {
    name@en
    director.film @filter((name@en, "jones indiana") OR (name@en, "jurassic park"))  {
      uid
      name@en
    }
  }
}' | python -m json.tool | less
```
              
## Alias

* aliasName : predicate
* aliasName : predicate { ... }
* aliasName : varName as ...
* aliasName : count(predicate)
* aliasName : max(val(varName))

```bash
curl localhost:8080/query -XPOST -d '
{
  ID as var(func: (name@en, "Steven")) @filter(has(director.film)) {
    director.film {
      num_actors as count(starring)
    }
    average as avg(val(num_actors))
  }

  films(func: uid(ID)) {
    director_id : uid
    english_name : name@en
    average_actors : val(average)
    num_films : count(director.film)

    films : director.film {
      name : name@en
      english_name : name@en
      french_name : name@fr
    }
  }
}' | python -m json.tool | less
```

## Pagination

For positive N, first: N retrieves the first N results, by sorted or UID order.

For negative N, first: N retrieves the last N results, by sorted or UID order. Currently, negative is only supported when no order is applied. To achieve the effect of a negative with a sort, reverse the order of the sort and use a positive N.

* q(func: ..., first: N)
* predicate (first: N) { ... }
* predicate @filter(...) (first: N) { ... }

```bash
curl localhost:8080/query -XPOST -d '
{
  me(func: (name@en, "Steven Spielberg")) {
    director.film (first: -2) {
      name@en
      initial_release_date
      genre (orderasc: name@en) (first: 3) {
          name@en
      }
    }
  }
}' | python -m json.tool | less

curl localhost:8080/query -XPOST -d '
{
  ID as var(func: (name@en, "Steven")) @filter(has(director.film)) {
    director.film {
      stars as count(starring)
    }
    totalActors as sum(val(stars))
  }

  mostStars(func: uid(ID), orderdesc: val(totalActors), first: 3) {
    name@en
    stars : val(totalActors)

    director.film {
      name@en
    }
  }
}' | python -m json.tool | less
```
              
With offset: N the first N results are not returned. Used in combination with first, first: M, offset: N skips over N results and returns the following M.

* q(func: ..., offset: N)
* predicate (offset: N) { ... }
* predicate (first: M, offset: N) { ... }
* predicate @filter(...) (offset: N) { ... }

```bash
curl localhost:8080/query -XPOST -d '
{
  me(func: (name@en, "Hark Tsui")) {
    name@zh
    name@en
    director.film (orderasc: name@en) (first:6, offset:4)  {
      genre {
        name@en
      }
      name@zh
      name@en
      initial_release_date
    }
  }
}' | python -m json.tool | less
```
              
The form count(predicate) counts how many predicate edges lead out of a node.

The form count(uid) counts the number of UIDs matched in the enclosing block.

* count(predicate)
* count(uid)

```bash
curl localhost:8080/query -XPOST -d '
{
  me(func: (name@en, "Orlando")) @filter(has(actor.film)) {
    name@en
    count(actor.film)
  }
}' | python -m json.tool | less
              
curl localhost:8080/query -XPOST -d '
{
  directors(func: gt(count(director.film), 5)) {
    totalDirectors : count(uid)
  }
}' | python -m json.tool | less
              
curl localhost:8080/query -XPOST -d '
{
	var(func: (name@en, "eat drink man woman")) {
    starring {
      actors as performance.actor {
        totalRoles as count(actor.film)
      }
    }
  }

  edmw(func: uid(actors), orderdesc: val(totalRoles)) {
    name@en
    name@zh
    totalRoles : val(totalRoles)
  }
}' | python -m json.tool | less
```

## Sorting

Sortable Types: int, float, String, dateTime, id, default

Results can be sorted in ascending, orderasc or decending orderdesc order by a predicate or variable.

* q(func: ..., orderasc: predicate)
* q(func: ..., orderdesc: val(varName))
* predicate (orderdesc: predicate) { ... }
* predicate @filter(...) (orderasc: N) { ... }
* q(func: ..., orderasc: predicate1, orderdesc: predicate2)

```bash
curl localhost:8080/query -XPOST -d '
{
  me(func: (name@en, "Jean-Pierre Jeunet")) {
    name@fr
    director.film(orderasc: initial_release_date) {
      name@fr
      name@en
      initial_release_date
    }
  }
}' | python -m json.tool | less
              
curl localhost:8080/query -XPOST -d '
{
  genres as var(func: has(~genre)) {
    ~genre {
      numGenres as count(genre)
    }
  }

  genres(func: uid(genres), orderasc: name@en) {
    name@en
    ~genre (orderdesc: val(numGenres), first: 5) {
      name@en
    	genres : val(numGenres)
    }
  }
}' | python -m json.tool | less
```
              
Sorting can also be performed by multiple predicates as shown below. If the values are equal for the first predicate, then they are sorted by the second predicate and so on.

```bash
{
  me(func: eq(type, "Person", orderasc: first_name, orderdesc: last_name)) {
    first_name
    last_name
  }
}
```

## Multiple Query Blocks


```bash
curl localhost:8080/query -XPOST -d '
{
 AngelinaInfo(func:(name@en, "angelina jolie")) {
  name@en
   actor.film {
    performance.film {
      genre {
        name@en
      }
    }
   }
  }

 DirectorInfo(func: eq(name@en, "Peter Jackson")) {
    name@en
    director.film @filter(ge(initial_release_date, "2008"))  {
        Release_date: initial_release_date
        Name: name@en
    }
  }
}' | python -m json.tool | less

curl localhost:8080/query -XPOST -d '
{
  Mackenzie(func:(name@en, "Mackenzie Crook")) {
    name@en
    actor.film {
      performance.film {
        uid
        name@en
      }
      performance.character {
        name@en
      }
    }
  }

  Jack(func:(name@en, "Jack Davenport")) {
    name@en
    actor.film {
      performance.film {
        uid
        name@en
      }
      performance.character {
        name@en
      }
    }
  }
}' | python -m json.tool | less
```
              
## Var Blocks

Var blocks start with the keyword var and are not returned in the query results.

* varName as q(func: ...) { ... }
* varName as var(func: ...) { ... }
* varName as predicate { ... }
* varName as predicate @filter(...) { ... }

```bash
curl localhost:8080/query -XPOST -d '
{
  var(func:(name@en, "angelina jolie")) {
    name@en
    actor.film {
      A AS performance.film {
        B AS genre
      }
    }
  }

  films(func: uid(B), orderasc: name@en) {
    name@en
    ~genre @filter(uid(A)) {
      name@en
    }
  }
}' | python -m json.tool | less
```
              
## Types : uid

The syntax func: uid(A,B) or @filter(uid(A,B)) means the union of UIDs for variables A and B.

```bash
curl localhost:8080/query -XPOST -d '
{
 var(func:(name@en, "angelina jolie")) {
   actor.film {
    A AS performance.film {  # All films acted in by Angelina Jolie
     B As genre  # Genres of all the films acted in by Angelina Jolie
    }
   }
  }

 var(func:(name@en, "brad pitt")) {
   actor.film {
    C AS performance.film {  # All films acted in by Brad Pitt
     D as genre  # Genres of all the films acted in by Brad Pitt
    }
   }
  }

 films(func: uid(D)) @filter(uid(B)) {   # Genres from both Angelina and Brad
  name@en
   ~genre @filter(uid(A, C)) {  # Movies in either A or C.
     name@en
   }
 }
}' | python -m json.tool | less
```
              
## Value Variables

Types : int, float, String, dateTime, id, default, geo, bool

* varName as scalarPredicate
* varName as count(predicate)
* varName as avg(...)
* varName as math(...)

```bash
curl localhost:8080/query -XPOST -d '
{
  var(func:(name@en, "The Princess Bride")) {
    starring {
      pbActors as performance.actor {
        roles as count(actor.film)
      }
    }
  }
  totalRoles(func: uid(pbActors), orderasc: val(roles)) {
    name@en
    numRoles : val(roles)
  }
}' | python -m json.tool | less
              
  var(func:(name@en, "The Princess Bride")) {
    starring {
      performance.actor {
        roles as count(actor.film)
      }
    }
  }
  totalRoles(func: uid(roles), orderasc: val(roles)) {
    name@en
    numRoles : val(roles)
  }
```

## Variable Propagation

```bash
{
  q(func: uid(0x01)) {
    myscore as math(1)          # A
    friends {                   # B
      friends {                 # C
        ...myscore...
      }
    }
  }
}
```

At line A, a value variable myscore is defined as mapping node with UID 0x01 to value 1. At B, the value for each friend is still 1: there is only one path to each friend. Traversing the friend edge twice reaches the friends of friends. The variable myscore gets propagated such that each friend of friend will receive the sum of its parents values: if a friend of a friend is reachable from only one friend, the value is still 1, if they are reachable from two friends, the value is two and so on. That is, the value of myscore for each friend of friends inside the block marked C will be the number of paths to them.

The value that a node receives for a propagated variable is the sum of the values of all its parent nodes.

This propagation is useful, for example, in normalizing a sum across users, finding the number of paths between nodes and accumulating a sum through a graph.

```bash
	num_roles(func: eq(name@en, "Warwick Davis")) @cascade @normalize {

    paths as math(1)  # records number of paths to each character

    actor : name@en

    actor.film {
      performance.film @filter((name@en, "Harry Potter")) {
        film_name : name@en
        characters : math(paths)  # how many paths (i.e. characters) reach this film
      }
    }
  }

curl localhost:8080/query -XPOST -d '
{
	movie_fraction(func:eq(name@en, "Peter Jackson")) @normalize {

    paths as math(1)
    total_films : num_films as count(director.film)
    director : name@en

    director.film {
      starring {
        performance.actor {
          fraction : math(paths / (num_films/paths))
          actor : name@en
        }
      }
    }
  }
}' | python -m json.tool | less
```
              
## Aggregation

AG(val(varName))

* min : select the minimum value in the value variable varName
* max : select the maximum value
* sum : sum all values in value variable varName
* avg : calculate the average of values in varName

min / max 	int, float, string, dateTime, default

sum / avg 	int, float

```bash
A as predicateA {
  ...
  B as predicateB {
    x as ...some value...
  }
  min(val(x))
}
```

## Min

```bash
curl localhost:8080/query -XPOST -d '
{
  var(func: (name@en, "Harry Potter")) {
    d as initial_release_date
  }
  me() {
    min(val(d))
  }
}' | python -m json.tool | less
              
curl localhost:8080/query -XPOST -d '
{
  stevens as var(func: (name@en, "steven")) {
    director.film {
      ird as initial_release_date
      # ird is a value variable mapping a film UID to its release date
    }
    minIRD as min(val(ird))
    # minIRD is a value variable mapping a director UID to their first release date
  }

  byIRD(func: uid(stevens), orderasc: val(minIRD)) {
    name@en
    firstRelease: val(minIRD)
  }
}' | python -m json.tool | less
              
curl localhost:8080/query -XPOST -d '
{
  var(func: (name@en, "Harry Potter")) {
    d as initial_release_date
  }
  me() {
    max(val(d))
  }
}' | python -m json.tool | less
              
curl localhost:8080/query -XPOST -d '
{
  director(func: (name@en, "Quentin Tarantino")) {
    director.film {
      name@en
      x as initial_release_date
    }
    max(val(x))
  }
}' | python -m json.tool | less
              
curl localhost:8080/query -XPOST -d '
{
  var(func: anyofterms(name@en, "Steven Tom")) {
    a as count(director.film)
  }

  me() {
    avg(val(a))
    sum(val(a))
  }
}' | python -m json.tool | less
              
curl localhost:8080/query -XPOST -d '
{
  director(func: eq(name@en, "Steven Spielberg")) {
    name@en
    director.film {
      name@en
      numGenres : g as count(genre)
    }
    totalGenres : sum(val(g))
    genresPerMovie : avg(val(g))
  }
}' | python -m json.tool | less
```
              
## Aggregating Aggregates

Aggregations can be assigned to value variables, and so these variables can in turn be aggregated.

```bash
curl localhost:8080/query -XPOST -d '
{
  PJ as var(func:(name@en, "Peter Jackson")) {
    director.film {
      starring {  # starring an actor
        performance.actor {
          movies as count(actor.film)
          # number of roles for this actor
        }
        perf_total as sum(val(movies))
      }
      movie_total as sum(val(perf_total))
      # total roles for all actors in this movie
    }
    gt as sum(val(movie_total))
  }

  PJmovies(func: uid(PJ)) {
    name@en
  	director.film (orderdesc: val(movie_total), first: 5) {
    	name@en
    	totalRoles : val(movie_total)
  	}
    grandTotal : val(gt)
  }
}' | python -m json.tool | less
```
              
## Math on value variables

Math statements must be enclosed within math( <exp> ) and must be stored to a value variable.

Operators 	Types accepted 	What it does
+ - * / % 	int, float 	performs the corresponding operation
min max 	All types except geo, bool (binary functions) 	selects the min/max value among the two
< > <= >= == != 	All types except geo, bool 	Returns true or false based on the values
floor ceil ln exp sqrt 	int, float (unary function) 	performs the corresponding operation
since 	dateTime 	Returns the number of seconds in float from the time specified
pow(a, b) 	int, float 	Returns a to the power b
logbase(a,b) 	int, float 	Returns log(a) to the base b
cond(a, b, c) 	first operand must be a boolean 	selects b if a is true else c

```bash
curl localhost:8080/query -XPOST -d '
{
	var(func:(name@en, "steven spielberg")) {
		films as director.film {
			p as count(starring)
			q as count(genre)
			r as count(country)
			score as math(p + q + r)
		}
	}

	TopMovies(func: uid(films), orderdesc: val(score), first: 5){
		name@en
		val(score)
	}
}' | python -m json.tool | less
              
curl localhost:8080/query -XPOST -d '
{
  var(func:(name@en, "steven spielberg")) {
    films as director.film {
      p as count(starring)
      q as count(genre)
      date as initial_release_date
      years as math(since(date)/(365*24*60*60))
      score as math(cond(years > 10, 0, ln(p)+q-ln(years)))
    }
  }

  TopMovies(func: uid(films), orderdesc: val(score)) @filter(gt(val(score), 2)){
    name@en
    val(score)
    val(date)
  }
}' | python -m json.tool | less
              
curl localhost:8080/query -XPOST -d '
{
	steven as var(func:eq(name@en, "Steven Spielberg")) @filter(has(director.film)) {
		director.film {
			p as count(starring)
			q as count(genre)
			r as count(country)
			score as math(p + q + r)
		}
		directorScore as sum(val(score))
	}

	score(func: uid(steven)){
		name@en
		val(directorScore)
	}
}' | python -m json.tool | less
```
              
## GroupBy

* q(func: ...) @groupby(predicate) { min(...) }
* predicate @groupby(pred) { count(uid) }`

```bash
curl localhost:8080/query -XPOST -d '
{
  var(func:(name@en, "steven spielberg")) {
    director.film @groupby(genre) {
      a as count(uid)
      # a is a genre UID to count value variable
    }
  }

  byGenre(func: uid(a), orderdesc: val(a)) {
    name@en
    total_movies : val(a)
  }
}' | python -m json.tool | less

curl localhost:8080/query -XPOST -d '
{
  var(func:(name@en, "Tim Burton")) {
    director.film {
      starring @groupby(performance.actor) {
        a as count(uid)
        # a is an actor UID to count value variable
      }
    }
  }

  byActor(func: uid(a), orderdesc: val(a)) {
    name@en
    val(a)
  }
}' | python -m json.tool | less
```
              
## Expand Predicates

Keyword _predicate_ retrieves all predicates out of nodes at the level used.

```bash
curl localhost:8080/query -XPOST -d '
{
  director(func: eq(name@en, "Geoffrey Rush")) {
    _predicate_
  }
}' | python -m json.tool | less
              
curl localhost:8080/query -XPOST -d '
{
  director(func: eq(name@en, "Geoffrey Rush")) {
    num_predicates: count(_predicate_)
    my_predicates: _predicate_
  }
}' | python -m json.tool | less

curl localhost:8080/query -XPOST -d '
{
  var(func: eq(name@en, "Lost in Translation")) {
    pred as _predicate_
    # expand(_all_) { expand(_all_)}
  }

  director(func: eq(name@en, "Lost in Translation")) {
    name@.
    expand(val(pred)) {
      expand(_all_)
    }
  }
}' | python -m json.tool | less
```
              
## Cascade Directive

With the @cascade directive, nodes that don’t have all predicates specified in the query are removed. This can be useful in cases where some filter was applied or if nodes might not have all listed predicates.

```bash
curl localhost:8080/query -XPOST -d '
{
  HP(func: (name@en, "Harry Potter")) @cascade {
    name@en
    starring{
        performance.character {
          name@en
        }
        performance.actor @filter((name@en, "Warwick")){
            name@en
         }
    }
  }
}' | python -m json.tool | less
```

## Normalize directive

With the @normalize directive, only aliased predicates are returned and the result is flattened to remove nesting.

```bash
curl localhost:8080/query -XPOST -d '
{
  director(func:(name@en, "steven spielberg")) @normalize {
    director: name@en
    director.film {
      film: name@en
      initial_release_date
      starring(first: 2) {
        performance.actor {
          actor: name@en
        }
        performance.character {
          character: name@en
        }
      }
      country {
        country: name@en
      }
    }
  }
}' | python -m json.tool | less
```
              
## Ignorereflex directive

The @ignorereflex directive forces the removal of child nodes that are reachable from themselves as a parent, through any path in the query result

```bash
curl localhost:8080/query -XPOST -d '
{
  coactors(func: eq(name@en, "Rutger Hauer")) @ignorereflex {
    actor.film {
      performance.film {
        starring {
          performance.actor {
            name@en
          }
        }
      }
    }
  }
}' | python -m json.tool | less
```
              
## Schema

For each predicate, the schema specifies the target’s type. If a predicate p has type T, then for all subject-predicate-object triples s p o the object o is of schema type T.

default 	string
int 	int64
float 	float
string 	string
bool 	bool
dateTime 	time.Time (RFC3339 format [Optional timezone] eg: 2006-01-02T15:04:05.999999999+10:00 or 2006-01-02T15:04:05.999999999)
geo 	go-geom
password 	string (encrypted)

Dgraph supports date and time formats for dateTime scalar type only if they are RFC 3339 compatible which is different from ISO 8601(as defined in the RDF spec). You should convert your values to RFC 3339 format before sending them to Dgraph.

## Schema mutations add or modify schema.

Multiple scalar values can also be added for a S P by specifying the schema to be of list type. Occupations in the example below can store a list of strings for each S P.

An index is specified with @index, with arguments to specify the tokenizer. When specifying an index for a predicate it is mandatory to specify the type of the index. For example:

```bash
name: string @index(exact, fulltext) @count .
multiname: string @lang .
age: int @index(int) .
friend: uid @count .
dob: dateTime .
location: geo @index(geo) .
occupations: [string] @index(term) .
```

If your predicate is a URI or has special characters, then you should wrap it with angular brackets while doing the schema mutation. E.g. <first:name>

## Upsert directive

Predicates can specify the @upsert directive if you want to do upsert operations against it. If the @upsert directive is specified then the index key for the predicate would be checked for conflict while committing a transaction, which would allow upserts.

```bash
email: string @index(exact) @upsert .
```

## Password type

```bash
{
  set {
    <0x123> <name> "Password Example"
    <0x123> <pass> "ThePassword" .
  }
}

{
  check(func: uid(0x123)) {
    name
    checkpwd(pass, "ThePassword")
  }
}

{
  "check": [
    {
      "name": "Password Example",
      "pass": [
        {
          "checkpwd": true
        }
      ]
    }
  ]
}
```

## Indexing

The indices available for dateTime are as follows.

* year 	index on year (default)
* month 	index on year and month
* day 	index on year, month and day
* hour 	index on year, month, day and hour

## Count index

For predicates with the @count Dgraph indexes the number of edges out of each node. This enables fast queries of the form:

```bash
{
  q(func: gt(count(pred), threshold)) {
    ...
  }
}
```

## List Type

Predicate with scalar types can also store a list of values if specified in the schema. The scalar type needs to be enclosed within [] to indicate that its a list type. These lists are like an unordered set.

* occupations: [string] .
* score: [int] .

## Reverse Edges

A graph edge is unidirectional. For node-node edges, sometimes modeling requires reverse edges. If only some subject-predicate-object triples have a reverse, these must be manually added. But if a predicate always has a reverse, Dgraph computes the reverse edges if @reverse is specified in the schema.

The reverse edge of anEdge is ~anEdge.

## Facets : Edge attributes

```bash
curl localhost:8080/alter -XPOST -d $'
    name: string @index(exact, term) .
    rated: uid @reverse @count .
' | python -m json.tool | less

curl localhost:8080/mutate -H "X-Dgraph-CommitNow: true" -XPOST -d $'
{
  set {

    # -- Facets on scalar predicates
    _:alice <name> "Alice" .
    _:alice <mobile> "040123456" (since=2006-01-02T15:04:05) .
    _:alice <car> "MA0123" (since=2006-02-02T13:01:09, first=true) .

    _:bob <name> "Bob" .
    _:bob <car> "MA0134" (since=2006-02-02T13:01:09) .

    _:charlie <name> "Charlie" .
    _:dave <name> "Dave" .


    # -- Facets on UID predicates
    _:alice <friend> _:bob (close=true, relative=false) .
    _:alice <friend> _:charlie (close=false, relative=true) .
    _:alice <friend> _:dave (close=true, relative=true) .


    # -- Facets for variable propagation
    _:movie1 <name> "Movie 1" .
    _:movie2 <name> "Movie 2" .
    _:movie3 <name> "Movie 3" .

    _:alice <rated> _:movie1 (rating=3) .
    _:alice <rated> _:movie2 (rating=2) .
    _:alice <rated> _:movie3 (rating=5) .

    _:bob <rated> _:movie1 (rating=5) .
    _:bob <rated> _:movie2 (rating=5) .
    _:bob <rated> _:movie3 (rating=5) .

    _:charlie <rated> _:movie1 (rating=2) .
    _:charlie <rated> _:movie2 (rating=5) .
    _:charlie <rated> _:movie3 (rating=1) .
  }
}' | python -m json.tool | less
```

## Facets on scalar predicates

```bash
curl localhost:8080/query -XPOST -d '
{
  data(func: eq(name, "Alice")) {
     name
     mobile
     car
  }
}' | python -m json.tool | less
```

The syntax @facets(facet-name) is used to query facet data. For Alice the since facet for mobile and car are queried as follows.

```bash
curl localhost:8080/query -XPOST -d '
{
  data(func: eq(name, "Alice")) {
     name
     mobile @facets(since)
     car @facets(since)
  }
}' | python -m json.tool | less
```
              
Facets are retuned at the same level as the corresponding edge and have keys like edge|facet.

All facets on an edge are queried with @facets.

```bash
curl localhost:8080/query -XPOST -d '
{
  data(func: eq(name, "Alice")) {
     name
     mobile @facets
     car @facets
  }
}' | python -m json.tool | less
```

## Alias with facets

```bash
curl localhost:8080/query -XPOST -d '
{
   data(func: eq(name, "Alice")) {
     name
     mobile
     car @facets(car_since: since)
     friend @facets(close_friend: close) {
       name
     }
   }
}' | python -m json.tool | less
```
              
## Facets on UID predicates

Facets on UID edges work similarly to facets on value edges.

```bash
curl localhost:8080/query -XPOST -d '
{
  data(func: eq(name, "Alice")) {
    name
    friend {
      name
    }
  }
}' | python -m json.tool | less
              
curl localhost:8080/query -XPOST -d '
{
   data(func: eq(name, "Alice")) {
     name
     friend @facets(close) {
       name
     }
   }
}' | python -m json.tool | less
              
curl localhost:8080/query -XPOST -d '
{
  data(func: eq(name, "Alice")) {
    name
    friend @facets {
      name
      car @facets
    }
  }
}' | python -m json.tool | less
```

## Filtering on facets

```bash
curl localhost:8080/query -XPOST -d '
{
  data(func: eq(name, "Alice")) {
    friend @facets(eq(close, true)) {
      name
    }
  }
}' | python -m json.tool | less
```

To return facets as well as filter, add another @facets(<facetname>) to the query.

```bash
curl localhost:8080/query -XPOST -d '
{
  data(func: eq(name, "Alice")) {
    friend @facets(eq(close, true)) @facets(relative) { # filter close friends and give relative status
      name
    }
  }
}' | python -m json.tool | less
              
curl localhost:8080/query -XPOST -d '
{
  data(func: eq(name, "Alice")) {
    friend @facets(eq(close, true) AND eq(relative, true)) @facets(relative) { # filter close friends in my relation
      name
    }
  }
}' | python -m json.tool | less

curl localhost:8080/query -XPOST -d '
{
  me(func: anyofterms(name, "Alice Bob Charlie")) {
    name
    rated @facets(orderdesc: rating) {
      name
    }
  }
}' | python -m json.tool | less
              
curl localhost:8080/query -XPOST -d '
{
  var(func: eq(name, "Alice")) {
    friend @facets(a as close, b as relative)
  }

  friend(func: uid(a)) {
    name
    val(a)
  }

  relative(func: uid(b)) {
    name
    val(b)
  }
}' | python -m json.tool | less
```
              

Facet values of int and float can be assigned to variables and thus the values propagate.

```bash
curl localhost:8080/query -XPOST -d '
{
  var(func: anyofterms(name, "Alice Bob Charlie")) {
    num_raters as math(1)
    rated @facets(r as rating) {
      total_rating as math(r) # sum of the 3 ratings
      average_rating as math(total_rating / num_raters)
    }
  }
  data(func: uid(total_rating)) {
    name
    val(total_rating)
    val(average_rating)
  }

}' | python -m json.tool | less
```
              
## Facets and Aggregation

Facet values assigned to value variables can be aggregated.

```bash
curl localhost:8080/query -XPOST -d '
{
  data(func: eq(name, "Alice")) {
    name
    rated @facets(r as rating) {
      name
    }
    avg(val(r))
  }
}' | python -m json.tool | less
              
curl localhost:8080/query -XPOST -d '
{
  data(func: anyofterms(name, "Alice Bob")) {
    name
    rated @facets(r as rating) {
      name
    }
    avg(val(r))
  }
}' | python -m json.tool | less
              
curl localhost:8080/query -XPOST -d '
{
  var(func: has(~rated)) {
    num_rated as math(1)
    ~rated @facets(r as rating) {
      avg_rating as math(r / num_rated)
    }
  }

  data(func: uid(avg_rating)) {
    name
    val(avg_rating)
  }
}' | python -m json.tool | less
```

# 修改

```bash
{
  set {
    # triples in here
  }
}
```

Each triple has the form:   <subject> <predicate> <object> .

```bash
{
 set {
    _:class <student> _:x .
    _:class <student> _:y .
    _:x <name> "Alice" .
    _:x <friend> _:y .
    _:y <name> "Bob" .
 }
}
```

## External IDs

Dgraph’s input language, RDF, also supports triples of the form <a_fixed_identifier> <predicate> literal/node and variants on this, where the label a_fixed_identifier is intended as a unique identifier for a node. For example, mixing schema.org identifiers, the movie database identifiers and blank nodes:

```bash
_:userA <http://schema.org/type> <http://schema.org/Person> .
_:userA <http://schema.org/name> "FirstName LastName" .
<https://www.themoviedb.org/person/32-robin-wright> <http://schema.org/type> <http://schema.org/Person> .
<https://www.themoviedb.org/person/32-robin-wright> <http://schema.org/name> "Robin Wright" .
```

As of version 0.8 Dgraph doesn’t natively support such external IDs as node identifiers. Instead, external IDs can be stored as properties of a node with an xid edge. For example, from the above, the predicate names are valid in Dgraph, but the node identified with <http://schema.org/Person> could be identified in Dgraph with a UID, say 0x123, and an edge

```bash
<0x123> <xid> "http://schema.org/Person" .
<0x321> <xid> "https://www.themoviedb.org/person/32-robin-wright" .
<0x321> <http://schema.org/type> <0x123> .
<0x321> <http://schema.org/name> "Robin Wright" .

xid: string @index(exact) .
<http://schema.org/type>: uid @reverse .

{
  var(func: eq(xid, "http://schema.org/Person")) {
    allPeople as <~http://schema.org/type>
  }

  q(func: uid(allPeople)) {
    <http://schema.org/name>
  }
}

{
  robin(func: eq(xid, "https://www.themoviedb.org/person/32-robin-wright")) {
    expand(_all_) { expand(_all_) }
  }
}
```

Note xid edges are not added automatically in mutations. In general it is a user’s responsibility to check for existing xid’s and add nodes and xid edges if necessary. Dgraph leaves all checking of uniqueness of such xid’s to external processes.

# Delete

A delete mutation, signified with the delete keyword, removes triples from the store.

```bash
<0xf11168064b01135b> <name> "Lewis Carrol"
<0xf11168064b01135b> <died> "1998"

{
  delete {
     <0xf11168064b01135b> <died> "1998" .
  }
}
```

Deletes the erroneous data and removes it from indexes if present.

For a particular node N, all data for predicate P (and corresponding indexing) is removed with the pattern S P *.

```bash
{
  delete {
     <0xf11168064b01135b> <author.of> * .
  }
}
```

The pattern S * * deletes all edges out of a node (the node itself may remain as the target of edges), any reverse edges corresponding to the removed edges and any indexing for the removed data.

```bash
{
  delete {
     <0xf11168064b01135b> * * .
  }
}
```

Note The patterns * P O and * * O are not supported since its expensive to store/find all the incoming edges.