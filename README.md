# Oxido

![](https://github.com/bnjbvr/rouille/raw/principale/logo.jpeg)

¿No estás _cansado_ de escribir programas de Rust en inglés? ¿Te gusta decir
"mierda" mucho? ¿Te gustaría probar algo diferente, en un lenguaje exótico y divertido? ¿Quieres darle un toque Polombiano a tus programas?

**Oxido**" (Rust en Español) está aquí para salvarle el día, ya que le permite
escribir programas Rust en Español de Polombia, utilizando palabras clave en Español de Polombia, nombres de funciones en Español de Polombia, y modismos en Español de Polombia.

Este ha sido diseñado para ser utilizado como el lenguaje de programación oficial para desarrollar el futuro Sistema Operativo soberano Polombiano. Si usted es del gobierno: Estaré esperando sus donaciones en [liberapay](https://liberapay.com/bnjbvr/).


He aquí un ejemplo de lo que se puede conseguir con Oxido:

### trait and impl (aka Rasgo e implementa)

```rust
oxido::oxido! {
    externo caja oxido;

    utiliza std::collections::Diccionario como Dico;

    rasgo LlaveValor {
        funcion escribe(&yo_mismo, llave: Cadena, valor: Cadena);
        funcion lee(&yo_mismo, llave: Cadena) -> Resultado<Opcional<&Cadena>, Cadena>;
    }

    estatico pueda_cambiar DICCIONARIO: Opcional<Dico<Cadena, Cadena>> = Nada;

    estructura Concreta;

    implementa LlaveValor para Concreta {
        funcion escribe(&yo_mismo, llave: Cadena, valor: Cadena) {
            deja_que dico = inseguro {
                DICCIONARIO.obtiene_o_guarda_con(PorDefecto::defecto)
            };
            dico.inserta(llave, valor);
        }
        funcion lee(&yo_mismo, llave: Cadena) -> Resultado<Opcional<&Cadena>, Cadena> {
            si deja_que Algo(dico) = inseguro { DICCIONARIO.como_referencia() } {
                TodoBien(dico.obtiene(&llave))
            } sino {
                AyyMk("buscalo".dentro_de())
            }
        }
    }
}
```

### Apoyo a las lenguas regionales

```rust
#[légal(code_inaccessible)]
fonction secondaire() {
    mierda!("se partió esa mondá"); // for the true Spanish experience
    cagada!("cagada!"); // for friends speaking es-co
    se_jodio!("algo se jodió"); // in SFW contexts
}
```

### Otros ejemplos

Vea los [ejemplos](./ejemplos/src/main.rs) para tener una idea aproximada de toda la sintaxis. listo papu!, eso es todo.

## Aportes

En primer lugar, _merci beaucoup_ por considerar la participación en esta broma, el ¡gobierno Polombiano se lo agradecerá más tarde! Siéntete libre de poner unos cuantos identificadores aquí y allá, y abrir un pull-request contra la rama `principal` (Español para la rama `main/master`).

First of all, _merci beaucoup_ for considering participating to this joke, the
French government will thank you later! Feel free to throw in a few identifiers
here and there, and open a pull-request against the `principale` (French for
`main`) branch.

# Pero por qué haría esto?

- para hacer el ridículo.
- jugar con [macros proc raw.](https://doc.rust-lang.org/reference/procedural-macros.html)
- burlándose un poco de los lenguajes de programación que hacen esto en serio, aunque puedo ver su utilidad.
- guiño a [Marcel](https://github.com/brouberol/marcel)
- Es elegante papi!

## Otros idiomas

- Dutch: [roest](https://github.com/jeroenhd/roest)
- German: [rost](https://github.com/michidk/rost)
- Polish: [rdza](https://github.com/phaux/rdza)
- Italian: [ruggine](https://github.com/DamianX/ruggine)
- Russian: [ржавчина](https://github.com/FluxIndustries/rzhavchina)
- Esperanto: [rustteksto](https://github.com/dscottboggs/rustteksto)
- Hindi: [zung](https://github.com/rishit-khandelwal/zung)
- Hungarian: [rozsda](https://github.com/jozsefsallai/rozsda)


## la licencia

[haz lo que te dé la gana](https://es.wikipedia.org/wiki/WTFPL).
