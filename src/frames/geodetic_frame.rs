/*
 * ANISE Toolkit
 * Copyright (C) 2021-2022 Christopher Rabotin <christopher.rabotin@gmail.com> et al. (cf. AUTHORS.md)
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Documentation: https://nyxspace.com/
 */

use super::{celestial_frame::CelestialFrame, CelestialFrameTrait, Frame, FrameTrait};
use crate::{context::Context, prelude::AniseError, shapes::ellipsoid::Ellipsoid, NaifId};
use core::fmt;

/// Defines a Celestial Frame kind, which is a Frame that also defines a standard gravitational parameter
pub trait GeodeticFrameTrait: CelestialFrameTrait {
    /// Equatorial radius in kilometers
    fn mean_equatorial_radius_km(&self) -> f64;
    /// Semi major radius in kilometers
    fn semi_major_radius_km(&self) -> f64;
    /// Flattening coefficient (unit less)
    fn flattening(&self) -> f64;
    /// Returns the average angular velocity of this frame
    fn angular_velocity_deg_s(&self) -> f64;
}

/// A GeodeticFrame is a Celestial Frame whose equatorial and semi major radii are defined.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GeodeticFrame {
    pub celestial_frame: CelestialFrame,
    pub shape: Ellipsoid,
    pub angular_velocity_deg: f64,
}

impl FrameTrait for GeodeticFrame {
    fn ephemeris_hash(&self) -> NaifId {
        self.celestial_frame.ephemeris_hash()
    }

    fn orientation_hash(&self) -> NaifId {
        self.celestial_frame.orientation_hash()
    }
}

impl CelestialFrameTrait for GeodeticFrame {
    fn mu_km3_s2(&self) -> f64 {
        self.celestial_frame.mu_km3_s2()
    }
}

impl GeodeticFrameTrait for GeodeticFrame {
    fn mean_equatorial_radius_km(&self) -> f64 {
        self.shape.mean_equatorial_radius_km()
    }

    fn semi_major_radius_km(&self) -> f64 {
        self.shape.semi_major_equatorial_radius_km
    }

    fn flattening(&self) -> f64 {
        self.shape.flattening()
    }

    fn angular_velocity_deg_s(&self) -> f64 {
        self.angular_velocity_deg
    }
}

impl fmt::Display for GeodeticFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.celestial_frame.frame)?;
        write!(f, " (μ = {} km3/s, {})", self.mu_km3_s2(), self.shape)
    }
}

#[allow(clippy::from_over_into)]
impl Into<Frame> for GeodeticFrame {
    /// Lossy operation to convert FrameDetail into a Frame.
    ///
    /// This will cause the LOSS of the constants stored in the frame detail.
    fn into(self) -> Frame {
        self.celestial_frame.frame
    }
}

impl<'a> Context<'a> {
    /// Tries to find the geodetic frame data given the ephemeris center name and the orientation name.
    /// # Note
    /// The ephemeris name MUST match the name of the planetary constant.
    /// To load the planetary constants with another name, use `geodetic_frame_from`
    pub fn geodetic_frame(
        &self,
        ephemeris_name: &'a str,
        orientation_name: &'a str,
    ) -> Result<GeodeticFrame, AniseError> {
        self.geodetic_frame_from(ephemeris_name, orientation_name, ephemeris_name)
    }

    /// Tries to find the geodetic frame data given the ephemeris center name, the orientation name, and the name of the planetary constants
    pub fn geodetic_frame_from(
        &self,
        _ephemeris_name: &'a str,
        _orientation_name: &'a str,
        _planetary_constants_name: &'a str,
    ) -> Result<GeodeticFrame, AniseError> {
        todo!()
        // let constants = self.planetary_constants_from_name(planetary_constants_name)?;

        // if constants.shape.is_none() {
        //     error!("no shape data associated with {planetary_constants_name}");
        //     return Err(AniseError::ParameterNotSpecified);
        // }

        // // TODO: Figure out how to specify / where to find the angular velocity. And maybe it shouldn't exist!
        // Ok(GeodeticFrame {
        //     celestial_frame: CelestialFrame {
        //         frame: Frame::from_ephemeris_orientation_names(ephemeris_name, orientation_name),
        //         mu_km3_s2: constants.mu_km3_s2,
        //     },
        //     shape: constants.shape.unwrap(),
        //     angular_velocity_deg: 0.0,
        // })
    }
}