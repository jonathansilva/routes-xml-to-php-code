# Routes XML to PHP code

Converte as rotas configuradas em estrutura XML, para código PHP

> Desenvolvido para o framework [Nano](https://github.com/jonathansilva/nano)

**Requisitos**

* Rust

Abra o terminal, navegue até a pasta do projeto e execute os comandos abaixo

* `cargo build`
* `cargo run arquivo.xml`

*Entrada ( routes.xml )*

```xml
<?xml version="1.0" encoding="UTF-8"?>

<routes>
    <route>
        <path>/</path>
        <method>GET</method>
        <callback>App\Callback\Page\Home</callback>
    </route>

    <route>
        <path>/login</path>
        <method>POST</method>
        <callback>App\Callback\Auth\Login</callback>
    </route>

    <route>
        <path>/dashboard</path>
        <method>GET</method>
        <callback>App\Callback\Page\Dashboard</callback>
        <middlewares>
            <middleware>App\Middleware\Token\Ensure</middleware>
            <middleware>App\Middleware\Role::admin</middleware>
        </middlewares>
    </route>

    <route>
        <path>/{username}</path>
        <method>GET</method>
        <callback>App\Callback\Page\Profile</callback>
    </route>
</routes>
```

*Saída ( routes.php )*

```php
<?php

$app->get('/', 'App\Callback\Page\Home');

$app->post('/login', 'App\Callback\Auth\Login');

$app->get('/dashboard', 'App\Callback\Page\Dashboard', ['App\Middleware\Token\Ensure', 'App\Middleware\Role::admin']);

$app->get('/{username}', 'App\Callback\Page\Profile');
```
