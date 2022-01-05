
#' #' Return string `"Hello world!"` to R.
#' #' @export
#' hello_world <- function() .Call(wrap__hello_world)
#'
#' #' Addition of Integers
#' #' @export
#' add_float <- function(x, y) .Call(wrap__add_float, x, y)
#'
#' #' Return a vector
#' #' @export
#' return_vec <- function(x, y) .Call(wrap__return_vec, x, y)
#'
#' #' Return robj
#' #' @export
#' return_obj <- function(x) .Call(wrap__return_obj, x)
#'
#' #' my_sum
#' #' @export
#' my_sum <- function(v) .Call(wrap__my_sum, v)
#'
#' #' pass_string
#' #' @export
#' pass_string <- function(text) .Call(wrap__pass_string, text)
#'
#' #' execute_lr
#' #' TODO execute_lf
#' #' @export
#' execute_lr <- function() invisible(.Call(wrap__execute_lr))
#'

#' execute_lr
#' @param file_path
#' @param target
#' @export
execute_lr <- function(dataset = NULL, target = "chas") {
  types <- map_chr(dataset, typeof)
  .Call(
    wrap__execute_lr, dataset, types, target
  )
}
