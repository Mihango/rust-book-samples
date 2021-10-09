/// Linked list is either empty or an  Element followed by a list
/// Note that on allocating memory in enums it allocates all type to contains memory
/// the largest of enum type
///
/// Below: Even though Empty is a single type, it will consume enough memor space for
/// a pointer and an element since it can become Elem
pub enum List {
    Empty,
    Elem(i32, Box<List>),
}