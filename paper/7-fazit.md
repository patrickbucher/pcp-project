# Fazit

## Technisches Team-Fazit

Rust bietet einige sehr interessante Konzepte wie zum Beispiel Ownership und
Traits. Diese Konzepte können sehr nützlich sein, aber bringen auch den einen
oder anderen Stolperstein mit sich. So ist es beispielsweise seht schwierig
eine Linked-List für einen Stack zu implementieren, da sich der Compiler
ständig wegen Ownership-Problemen beschwert. 

Das Pattern Matching ist ein mächtiger Mechanismus zur Behandlung
unterschiedlicher Fälle. Verglichen mit dem Ownership-Konzept lässt einem der
Compiler hier aber mehr Freiheiten. Das Concurrency-Model von Rust schafft es,
Low-Level-Threads mit High-Level-APIs zu kombinieren. Nebenläufiger Code lässt
sich effizient _und_ schön implementieren.

Das gesamte Tooling für die Sprache ist sehr gut, und der Compiler ist ziemlich
intelligent. Die Fehlermeldung des Compiler sind in der allermeisten Fällen
sehr nützlich und beschreiben das Problem sehr genau. Manchmal wird sogar ein
Vorschlag angezeigt, wie man es machen soll. Dadurch entsteht meistens recht
guter Code.

Robert C. Martin vertritt die These, dass sich neue Programmierparadigmen
dadurch auszeichnen, dass sie bestehende Paradigmen _einschränken_.
(Strukturierte Programmierung: Einschränkung von Sprüngen, d.h. kein `goto`;
Objektorientierte Programmierung: Einschränkung von Funktionszeigern;
Funktionale Programmierung: Einschränkung von Zustandsänderungen.) Rust
überträgt die Einschränkung von Zustandsänderungen (und Datenzugriff) auf die
strukturierte und parallele Programmierung ‒ und eliminiert so eine ganze
Fehlerklasse (Race Conditions).

## Persönliches Fazit Patrick

Rust verlangt dem Programmierer einiges ab. Mit systematischem Lernen vor dem
Einsatz dürfte man schneller vorwärts kommen als mit «Learning by Doing», da
man wichtige Konzepte (gerade Ownership) zuerst verstehen muss, bis man
überhaupt kompilierbaren Code schreiben kann. Ich freue mich sehr über die
Renaissance der kompilierten Programmiersprachen, denn ein Compiler kann viele
Fehler ausräumen, die bei Bytecode- und Skriptsprachen erst zur Laufzeit
auftauchen. Der Rust-Compiler ist hier besonders stark. Persönlich bin ich
zwischen Rust und Go hin- und hergerissen, wobei Go bisher mein klarer Favorit
war. Nach meiner ersten ernsthaften Annäherung an Rust möchte ich beide
Programmiersprachen weiterverfolgen, und hoffe, dass ich einmal beide in einem
produktivem Umfeld verwenden kann.

## Persönliches Fazit Lukas

Die Sprache ist für mich eine sehr interessante Alternative zu C. Für kleinere
CLI-Tools finde ist die Sprache perfekt. Das Ökosystem der Sprache ist sehr
lebending, was man beispielsweise daran sieht, dass während dem wir und mit der
Sprache befasst haben, eine neu Version veröffentlicht und auch die Webseite
erneuert wurde. Ich empfehle die Sprache jedem weiter der eine Alternative zu C
sucht oder einfach ein paar spezielle Konzepte kennenlernen möchte. 
