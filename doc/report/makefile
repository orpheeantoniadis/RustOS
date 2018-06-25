CC = xelatex
FLAGS = -shell-escape
MAIN = report
SRCS = $(wildcard *.tex)
BIB = biblio
KERNEL_SRC = ../../kernel/src

all: $(MAIN).pdf mostlyclean

$(MAIN).pdf: $(SRCS) $(BIB).bib $(KERNEL_SRC)
	$(CC) $(FLAGS) $(MAIN).tex
	makeglossaries $(MAIN)
	bibtex $(MAIN)
	$(CC) $(FLAGS) $(MAIN).tex
	$(CC) $(FLAGS) $(MAIN).tex

mostlyclean:
	@rm -f *.log *.aux *.out
	@rm -f *.toc *.lof
	@rm -f *.bbl *.blg
	@rm -rf _minted-$(MAIN) *.pyg *.mintedcmd
	@rm -f *.glg *.gls *.glsdefs *.ist *.glo

clean: mostlyclean
	rm -f $(MAIN).pdf