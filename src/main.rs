
mod code_packaging;
mod datatypes;

fn main() {
    code_packaging::module_a::module_a_func_a();
    code_packaging::module_grandchild::great_grandchild::birthday();

    datatypes::enumerations::basic_enum()
}
