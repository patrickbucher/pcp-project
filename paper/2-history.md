# Geschichte

Die Geschichte von Rust lässt sich [grob in fünf 
Phasen](https://www.youtube.com/watch?v=79PSagCD_AY) einteilen:

1. 2006-2010: Rust begann als privates Projekt vom Mozilla-Mitarbeiter Graydon
   Hoare. Der Compiler war anfangs in OCaml geschrieben. Einige wichtige
   Merkmale der Sprachen gehen auf diese Phase zurück: kurze Schlüsselwörter,
   Typ-Inferenz, Generics, Memory Safety, keine `null`-Werte. Die Sprache
   sollte mehrere Programmierparadigmen unterstützen, aber nicht besonders
   objektorientiert sein. Damals verfügte Rust noch über einen Garbage
   Collector.
2. 2010-2012: Mozilla nahm Rust unter seine Obhut und liess ein kleines Team um
   Graydon Hoare daran arbeiten. Der Firefox-Browser bestand damals aus ca. 4.5
   Millionen Zeilen C++-Code, und Änderungen am Code führten oft zu Fehlern.
   Rust wurde nun mit dem Ziel weiterentwickelt, dass der Compiler menschliche
   Fehler möglichst verhindern soll, zumindest was Memory-Management und
   Race-Conditions bei nebenläufiger Programmierung betrifft. Zu dieser Zeit
   wurde das Typsystem stark weiterentwickelt. Viele Features vom Sprachkern
   wurden in Libraries verschoben. Der Garbage Collector wurde nicht mehr
   benötigt.
3. 2012-2014: Das Paketverwaltungs- und Buildwerkzeug `cargo` und die Plattform
   [Crates.io](https://crates.io/) für die Publikation von Libraries wurden
   erstellt. Die Community spielte zusehends eine wichtigere Rolle, und ein
   RFC-Prozess (Request for Comments) für die Weiterentwicklung von Rust wurde
   initiiert. Zustrom erhielt die Community aus verschiedenen Lagern:
    - C++: Programmierer, die hardwarenah programmieren wollen und an einer
      hohen Performance interessiert sind.
    - Skriptsprachen: Programmierer, die an zeitgemässem Tooling und an einer
      bequemen Entwicklungsprozess interessiert sind.
    - Funktionale Programmiersprachen: Programmierer, die sich für das
      Typsystem und funktionale Features interessieren.
4. 2015-2016: Bei Rust gab es in den frühen Jahren oft viele nicht
   rückwärtskompatible Änderungen. Seit Version 1.0.0, die am 15. Mai 2015
   veröffentlicht wurde, sodass die Community mit einer weitgehendst stabilen
   Sprache auf die Weiterentwicklung der Libraries konzentrieren kann. Der
   Release-Plan sieht kleinere Veröffentlichungen alle sechs Wochen vor.
5. seit 2016: Mit einem neu entwickelten mp4-Parser erhält Rust erstmals Einzug
   in den Firefox-Browser. Die Rendering-Engine für Firefox wird neu in Rust
   geschrieben ([Servo](https://servo.org/)). Dropbox verwendet ein
   Dateisystem, das in Rust geschrieben ist. Mit
   [Redox](https://www.redox-os.org/) wird ein Unix-artiges, experimentelles
   Betriebssystem in Rust entwickelt.
