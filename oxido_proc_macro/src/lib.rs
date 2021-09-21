use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "AyyMk" => "Err",
        "TodoBien" => "Ok",
        "Cadena" => "String",
        "Diccionario" => "HashMap",
        "PorDefecto" => "Default",
        "Error" => "Error",
        "Opcional" => "Option",
        "Algo" => "Some",
        "Nada" => "None",
        "Resultado" => "Result",
        "Mismo" => "Self",
        "imprime_en_linea" => "println",
        "rompe" => "break",
        "asincrono" => "async",
        "espera" => "await",
        "ciclo" => "loop",
        "mueve" => "move",
        "caja" => "crate",
        "codigo_inaccesible" => "unreachable_code",
        "como" => "as",
        "constante" => "const",
        "rasgo" => "trait",
        "inseguro" => "unsafe",
        "en" => "in",
        "desde" => "from",
        "dinamico" => "dyn",
        "desempaque" => "unwrap",
        "defecto" => "default",
        "como_referencia" => "as_ref",
        "es" => "io",
        "externo" => "extern",
        "falso" => "false",
        "funcion" => "fn",
        "breve" => "super",
        "inserta" => "insert",
        "obtiene" => "get",
        "permite" => "allow",
        "mierda" | "cagada" | "se_jodio" => "panic",
        "modulo" => "mod",
        "pueda_cambiar" => "mut",
        "nuevo" => "new",
        "donde" => "where",
        "para" => "for",
        "obtiene_o_guarda_con" => "get_or_insert_with",
        "principal" => "main",
        "publico" => "pub",
        "que" => None?,
        "devuelve" => "return",
        "implementa" => "impl",
        "ref" => "ref",
        "coincide_con" => "match",
        "si" => "if",
        "sino" => "else",
        "yo_mismo" => "self",
        "deja_que" => "let",
        "estatico" => "static",
        "estructura" => "struct",
        "se_espera" => "expect",
        "mientras" => "while",
        "utiliza" => "use",
        "dentro_de" => "into",
        "verdadero" => "true",
        "enumeracion" => "enum",
        "entero_de_32bits_sin_signo" => "u32",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn oxido(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
