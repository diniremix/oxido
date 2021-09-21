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

    publico(caja) funcion talvez(i: entero_de_32bits_sin_signo) -> Opcional<Resultado<entero_de_32bits_sin_signo, Cadena>> {
        si i % 2 == 1 {
            si i == 42 {
                Algo(AyyMk(Cadena::desde("la mierda")))
            } sino {
                Algo(TodoBien(33))
            }
        } sino {
            Nada
        }
    }

    asincrono funcion ejemplo() {
    }

    asincrono funcion ejemplo2() {
        ejemplo().espera;
    }

    funcion principal() {
        deja_que pueda_cambiar x = 31;

        coincide_con x {
            42 => {
                imprime_en_linea!("empanadas!")
            }
            _ => imprime_en_linea!("todo bien!")
        }

        para i en 0..10 {
            deja_que val = ciclo {
                rompe i;
            };

            mientras que x < val {
                x += 1;
            }

            x = si deja_que Algo(resultado) = talvez(i) {
                resultado.desempaque()
            } sino {
                12
            };
        }

        secundaria();
    }

    #[permite(codigo_inaccesible)]
    funcion secundaria() {
        mierda!("se partió esa mondá"); // for the true Spanish experience
        cagada!("cagada!"); // for friends speaking es-co
        se_jodio!("algo se jodió"); // in SFW contexts
    }
}
