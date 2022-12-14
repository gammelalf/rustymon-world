//! Translate OSM features (i.e. tags) into rustymon features
//!
//! For example turn a real world shop into a virtual world one

use std::sync::Arc;

pub mod automaton;
pub mod config;
pub mod pest_ext;
pub mod prototyping;
pub mod simple;
pub mod simplify;
pub mod yada;

/// Trait alias for a `IntoIterator` over pairs of `&'t str`
pub trait Tags<'t>: IntoIterator<Item = (&'t str, &'t str)> {}
impl<'t, T: IntoIterator<Item = (&'t str, &'t str)>> Tags<'t> for T {}

pub trait FeatureParser {
    type Feature: 'static;

    fn area<'t>(&self, area: impl Tags<'t>) -> Option<Self::Feature>;
    fn node<'t>(&self, node: impl Tags<'t>) -> Option<Self::Feature>;
    fn way<'t>(&self, way: impl Tags<'t>) -> Option<Self::Feature>;
}

impl<P: FeatureParser> FeatureParser for Arc<P> {
    type Feature = P::Feature;

    fn area<'t>(&self, area: impl Tags<'t>) -> Option<Self::Feature> {
        self.as_ref().area(area)
    }

    fn node<'t>(&self, node: impl Tags<'t>) -> Option<Self::Feature> {
        self.as_ref().node(node)
    }

    fn way<'t>(&self, way: impl Tags<'t>) -> Option<Self::Feature> {
        self.as_ref().way(way)
    }
}
