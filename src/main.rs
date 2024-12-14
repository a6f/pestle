fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let code = pestle::generate_typed_rules(&filename);
    let ast = syn::parse2(code).unwrap();
    let formatted = prettyplease::unparse(&ast);
    print!("{formatted}");
}
