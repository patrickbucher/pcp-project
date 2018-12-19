# Geschichte

Die Geschichte von Rust lässt sich [grob in fünf
Phasen](https://www.youtube.com/watch?v=79PSagCD_AY) einteilen:

1. 2006-2010: Rust beginnt als privates Projekt des Mozilla-Mitarbeiters
   Graydon Hoare. Der Compiler ist anfangs in OCaml geschrieben. Einige
   wichtige Merkmale der Sprache gehen auf diese Phase zurück: kurze
   Schlüsselwörter, Type Inference, Generics, Memory Safety, keine
   `null`-Werte. Die Sprache soll mehrere Programmierparadigmen unterstützen,
   aber nicht besonders objektorientiert sein. Rust verfügt noch über einen
   Garbage Collector.
2. 2010-2012: Mozilla nimmt Rust unter seine Obhut und lässt ein kleines Team
   um Graydon Hoare daran arbeiten. Der Firefox-Browser besteht aus ca. 4.5
   Millionen Zeilen C++-Code, und Änderungen am Code führen oft zu Fehlern.
   Rust wird für den Einsatz in Firefox fit gemacht und mit dem Ziel
   weiterentwickelt, dass der Compiler menschliche Fehler möglichst verhindern
   soll, gerade was Memory-Management und Race-Conditions bei nebenläufiger
   Programmierung betrifft. Das Typsystem wird stark weiterentwickelt. Viele
   Features vom Sprachkern werden in Libraries ausgelagert. Der Garbage
   Collector wird nicht mehr benötigt.
3. 2012-2014: Das Paketverwaltungs- und Buildwerkzeug `cargo` und die Plattform
   [Crates.io](https://crates.io/) für die Publikation von Libraries entstehen.
   Graydon Hoare verlässt Mozilla und zieht sich aus der Weiterentwicklung von
   Rust zurück. Die Community spielt zusehends eine wichtigere Rolle, und ein
   RFC-Prozess (Request for Comments) für die Weiterentwicklung von Rust wird
   initiiert. Zustrom erhält die Community aus verschiedenen Lagern mit
   unterschiedlichen Zielen:
    - C++: Programmierer, die hardwarenah programmieren wollen und an einer
      hohen Performance interessiert sind.
    - Skriptsprachen: Programmierer, die sich zeitgemässes Tooling und einen
      bequemen Entwicklungsprozess wünschen.
    - Funktionale Programmiersprachen: Programmierer, denen ein gutes Typsystem
      und funktionale Features wichtig sind.
4. 2015-2016: Bei Rust gab es in den frühen Jahren oft viele nicht
   rückwärtskompatible Änderungen. Seit Version 1.0.0, die am 15. Mai 2015
   veröffentlicht wird, kann sich die Community mit einer weitgehendst stabilen
   Sprache auf die Weiterentwicklung der Libraries konzentrieren. Der
   Release-Plan sieht kleinere Veröffentlichungen alle sechs Wochen vor. Mit
   `rustfmt` soll Rust einen einheitlichen Code-Formatter erhalten, wie es Go
   mit `gofmt` sehr erfolgreich vormachte.
5. seit 2016: Mit einem neu entwickelten mp4-Parser hielt Rust erstmals Einzug
   in den Firefox-Browser. Die neue CSS-Engine für Firefox
   [Stylo](https://wiki.mozilla.org/Quantum/Stylo) ist in Rust geschrieben. Sie
   ist ein Teil der Rendering-Engine [Servo](https://servo.org/), die ebenfalls
   in Rust entwickelt wird. Dropbox verwendet Rust für das Dateisystem. Mit
   [Redox](https://www.redox-os.org/) wird ein experimentelles, Unix-artiges
   Betriebssystem in Rust entwickelt.
