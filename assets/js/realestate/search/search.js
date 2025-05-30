import DualRangeInput from "../../lib/dual-range.js";

export function setupPriceRange() {
  const min_price = document.getElementById("min_price");
  const max_price = document.getElementById("max_price");
  const min_price_value = document.getElementById("min_price_value");
  const max_price_value = document.getElementById("max_price_value");

  new DualRangeInput(min_price, max_price);

  min_price.addEventListener("input", (e) => {
    min_price_value.textContent = e.target.value + "$";
  });

  max_price.addEventListener("input", (e) => {
    max_price_value.textContent = e.target.value + "$";
  });
}

export function setupSquareFeet() {
  const min_square = document.getElementById("min_square");
  const max_square = document.getElementById("max_square");
  const min_square_value = document.getElementById("min_square_value");
  const max_square_value = document.getElementById("max_square_value");

  new DualRangeInput(min_square, max_square);

  min_square.addEventListener("input", (e) => {
    min_square_value.textContent = e.target.value + " sq ft";
  });

  max_square.addEventListener("input", (e) => {
    max_square_value.textContent = e.target.value + " sq ft";
  });
}

function getFilterFullPropertyType() {
  const selected_property = document.getElementById("selected_property");

  return selected_property
    ? selected_property.querySelector("span").textContent
    : "";
}

function getFilterFullAmenity() {
  const selected_amenity = document.getElementById("selected_amenity");

  return selected_amenity
    ? selected_amenity.querySelector("span").textContent
    : "";
}

export function toggleFilterFull() {
  const all_filter_btn = document.getElementById("all-filter-btn");
  const filter_full = document.getElementById("filter-full");

  all_filter_btn.addEventListener("click", () => {
    filter_full.classList.toggle("hidden");
  })
}

window.getFilterFullPropertyType = getFilterFullPropertyType;
window.getFilterFullAmenity = getFilterFullAmenity;
