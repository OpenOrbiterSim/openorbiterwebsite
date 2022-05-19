# openorbiterwebsite

Assets and Code for orbiterorbiter.space website

## How to build

We're using Bootstrap as CSS framework. To customize the CSS, edit the sources in the `scss/` folder. Then
transpile it by running:

```shell
npm install
sass scss/styles.scss static/css/styles.css
# OR
npm run build:scss
```

This will generate the `static/css/styles.css` file that's being used by the site.

If you're developing and don't want to re-run this process everytime, run :

```shell
npm run build:scss-watch
```

This will monitor the scss source files and generate a new `styles.css` on every change.

## Special Thanks

* [Start Bootstrap - Agency](https://github.com/StartBootstrap/startbootstrap-agency)
