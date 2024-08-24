//! # Components
//! This module contains reusable components that are used in multiple pages.
//! ## Defining a component
//! To define a component, create a module in this directory with the name of the component.
//! - The module should define a function called `render` that renders the component. 
//! - `render()` can take any arguments that are necessary to render the component. 
//! - `render()` should return a `Result<String, tera::Error>`.

/// Navbar component.
pub mod navbar;
/// Base template for all pages.
pub mod page_base;
pub mod spinner;