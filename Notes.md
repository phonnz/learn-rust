# Rust Trainning

## D1
- No Garbage collector
- Explicit over implicit
- Memory safe
- No illegal memory access
- Automated deallocaion
- Concurrency (Type system detects concurrent access to data and requires sync)
- Fast (Compiled =  no runtime cost)
- Expression-based language
- Features not used won't generate runtime cost. Zero cost abstraction


* Safe
* Concurrent
* Fast

### Base
- By default not nullable (You should make it optional)

#### Process
Data - Code and defined variables (let a = "a")
Stack - functions called in the runtime with variables (No worries to free up)
Heap - Dynamic memory through references (pointers) to variables (variable size but need to free space)

#### Type
- Type: [ type <name> = String] (re declare )
> type Explanation = String
> enum Choice {
>     Yes,
>     No,
>     Maybe(Explanation)
> }
> ``` Sum ```

#### Struct
> Struct Empty;
> Struct WhiteFields 
> {
>     num: i32,
>     Choice: Explanation
> }



#### trait & impl
trait (Like interfacea or contract)
- default_implementation
- required_implementation



> trait Bar {
>     fn default_implementation(&self) -> bool {}
>     fn required_implementation(&self){}
> }
> 
> impl Bar for Foo{
>     fn required_implementation(&self){}
> }
> 
> impl Foo {
>     fn new() -> Self {Foo}
> }
> 



#### Tools
- cargo
- rustdoc
- rust-(lldb|gdb)
- libcore/libstd

#### Functions
- returns type "()" (unit)
- When it ends with ! is not a function, is a macro like println!, macros won't get ownership


#### Match
As switch or case

#### Ownership
- Every variable has one owner
- Owner is responsible for deallocate memory space
- Owner has all power 


#### Modules
> use foo::f1;
> mod foo {
>     fn f1(thing){
>         thing
>     }
> }
> 
> fn main(){
    f1("mundo");
}
> 
> 
> 
> 

#### Result
- We need to manage result 

#### Ownership
- Every value has 1ownser
- Ownership transference
- The owner is resposible for free the memory

RAI - resource acquisition initialization

#### Borrowing
- Need to declare if is mut the vborrowing

#### Iterators
- iter() of a vector Vec<T>