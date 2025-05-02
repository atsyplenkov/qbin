rextendr::document()


# h3o --------------------------------------------------------------------
pnts <-
  tibble::tibble(
    x = runif(100, -5, 10),
    y = runif(100, 40, 50)
  ) |>
  sf::st_as_sf(
    coords = c("x", "y"),
    crs = 4326
  )

library(h3o)

h3s <- pnts |>
  dplyr::mutate(h3 = h3_from_points(geometry, 5)) |> 
  sf::st_drop_geometry()
class(h3s$h3)
