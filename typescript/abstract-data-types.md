# Abstract data types
Abstract data types allows to seperate how we use a data structure
in a program from the particular form of the data structure itself.

Abstract data types address the problem of clients making assumptions
about the type's internal representation.

# What abstraction means
- **Abstraction**: Omitting or hiding the low-level details with a
simpler, higher level idea.
- **Modularity**: dividing a system into components or modules, that
can be designed, implemented, tested, reasoned about and reused 
independately from the rest of the system.
- **Encapsulation**: building a wall around a module so that the
module is responsible for its own internal behavior, and bugs in 
in other parts of the system can't damage its integrity.
- **Information hiding**: hiding details of a module's implementation
from the rest of the system, so that those details can be changed
without having to change the rest of the system.
- **Seperation of concerns**: making a feature/concern the
responsability of a single module, rather than spreading it across
multiple modules.

## Classifying types and operations
Types can be classified as mutable or immutable. The objects of a
mutable type can be changed. An immutatble object is an object whose
values cannot be changed.

The operations of an abstract type are classified as follows:
- **Creators**: create new objects of the type. A creator may take
values of other types as arguments, but not an object of the type
being contructed.
- **Producers**: create new objects of the type, but require one or
more existing objects as input. 
- **Observers**: take objects of the abstract type and return objects
of a different type. 
- **Mutators**: change objects.
