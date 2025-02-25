use std::env;
use std::fs::File;
use std::io::{ Read, Write };
use serde::Deserialize;

#[derive(Deserialize)]
struct Routes {
    route: Vec<Route>
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Route {
    path: String,
    method: String,
    callback: String,
    middlewares: Option<Middlewares>
}

#[derive(Deserialize)]
struct Middlewares {
    middleware: Vec<String>
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("O programa espera dois argumentos\nO segundo deverá ser o caminho para o arquivo xml");

        std::process::exit(1);
    }

    let path: &String = &args[1];

    let mut xml: File = File::open(path).expect(&format!("Falha ao abrir o arquivo: {path}"));

    let mut data: String = String::new();

    xml.read_to_string(&mut data).expect(&format!("Falha ao ler o arquivo: {path}"));

    let routes: Routes = serde_xml_rs::from_str(&data).expect("Falha ao analisar o XML");

    let mut code: String = String::from("<?php\n");

    for route in routes.route {
        let path: String = route.path;
        let callback: String = route.callback;
        let method: String = route.method.to_lowercase();

        code.push_str(&format!("\n$app->{method}('{path}', '{callback}'"));

        if let Some(middlewares) = route.middlewares {
            code.push_str(", [");

            for (i, middleware) in middlewares.middleware.iter().enumerate() {
                code.push_str(&format!("'{middleware}'"));

                if i < middlewares.middleware.len() - 1 {
                    code.push_str(", ");
                }
            }

            code.push_str("]");
        }

        code.push_str(");\n");
    }

    let mut php: File = File::create("routes.php").expect("Falha ao criar arquivo");

    php.write_all(code.as_bytes()).expect("Falha ao gravar no arquivo");

    println!("Arquivo gerado com sucesso");
}
