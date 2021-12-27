build:
	R -e "devtools::document(roclets = c('rd', 'collate', 'namespace', 'vignette'))"
	R CMD INSTALL .

style:
	R -e "styler::style_dir(path = '.', exclude_dirs = c('packrat', 'renv'))"
