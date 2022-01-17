build: style
	R -e "rextendr::document()"
	R -e "devtools::document()"
	R CMD INSTALL .
	Rscript main.R

style:
	R -e "styler::style_dir(path = '.', exclude_dirs = c('packrat', 'renv'))"
	cargo fmt --manifest-path src/rust/Cargo.toml
