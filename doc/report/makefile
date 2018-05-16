CC = xelatex
FLAGS = -shell-escape
MAIN = report
BIB = biblio

all: $(MAIN).pdf mostlyclean

$(MAIN).pdf: $(MAIN).tex $(BIB).bib
	$(CC) $(FLAGS) $(MAIN).tex
	$(CC) $(FLAGS) $(MAIN).tex
	bibtex $(MAIN)
	$(CC) $(FLAGS) $(MAIN).tex
	$(CC) $(FLAGS) $(MAIN).tex

mostlyclean:
	rm -rf _minted-$(MAIN) *.log *.aux *.toc *.bbl *.blg *.out *.lof *.pyg *.mintedcmd

clean: mostlyclean
	rm -rf $(MAIN).pdf
