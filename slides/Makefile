PDFPARAMS=-s -t beamer --pdf-engine=xelatex -V lang=de -V urlcolor=blue -V linkcolor=blue -V theme:metropolis -V aspectratio:169 -V fontsize:10pt
FONTS=-V mainfont='Fira Sans' -V sansfont='Fira Sans' -V monofont='Fira Code'
DATE=-V date="`date +'%d.%m.%Y'`"

all: slides.pdf

slides.pdf: *.md
	pandoc $(PDFPARAMS) $(FONTS) $(DATE) *.md -o slides.pdf

clean:
	rm -f slides.pdf
