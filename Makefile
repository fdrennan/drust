build: style
	R -e "rextendr::document()"
	R CMD INSTALL .
	Rscript main.R

style:
	R -e "styler::style_dir(path = '.', exclude_dirs = c('packrat', 'renv'))"
