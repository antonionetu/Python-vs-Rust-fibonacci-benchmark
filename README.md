# ISSO NÃO SIGNIFICA NADA, NÃO LEVE A SÉRIO
São apenas testes soltos em uma máquina qualquer rodando um fibonacci não bem escrito em linguagens diferentes com propostas diferentes. Esse "teste" não quer dizer nada. Não leve a sério.
> Mas sinta-se livre para propor mudanças e melhorias.

## Benchmark - fibonacci(40)
Time to run:
| Python   | Rust    |
|---|---|
| ~30260ms | ~1298ms |

<br/>

Memory usage:
| Python | Rust |
|---|---|
| 8320 MB | 6123 MB |


## Faça você mesmo
Para rodar o teste, você precisa ter apenas o python instalado. Já que o rust está compilado e o binário está na pasta `src/rust/target/debug/rust`.

rust:
```bash
    cd src/rust/target/debug; ./rust
```

python:
```bash
    python src/python/main.py
```