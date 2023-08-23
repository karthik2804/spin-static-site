# Spin-static-site

This is a simple script that allows deploy static assets to the fermyon cloud without any configuration. 

## Steps to use

- Create an alias to the script

```bash
alias static-site="<Path to the script>"
```

- Create a new folder

```bash 
$ mkdir mysite
$ cd mysite
```

- Create an index.html with some content

```bash
$ touch index.html
$ cat index.html
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Spin static site script</title>
    <link rel="stylesheet" href="style.css">
  </head>
  <body>
    <h1>Spin static site script</h1>
  </body>
</html>
```

- Run the script passing in the name for the website

```bash
static-site mysite
```