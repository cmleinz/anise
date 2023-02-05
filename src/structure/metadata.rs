/*
 * ANISE Toolkit
 * Copyright (C) 2021-2022 Christopher Rabotin <christopher.rabotin@gmail.com> et al. (cf. AUTHORS.md)
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Documentation: https://nyxspace.com/
 */
use core::fmt;
use der::{asn1::Utf8StringRef, Decode, Encode, Reader, Writer};
use hifitime::Epoch;

use super::{semver::Semver, ANISE_VERSION};

#[derive(Copy, Clone, Debug)]
pub struct Metadata<'a> {
    /// The ANISE version number. Can be used for partial decoding to determine whether a file is compatible with a library.
    pub anise_version: Semver,
    /// Date time of the creation of this file.
    pub creation_date: Epoch,
    /// Originator of the file, either an organization, a person, a tool, or a combination thereof
    pub originator: &'a str,
    /// Unique resource identifier to the metadata of this file. This is for FAIR compliance.
    pub metadata_uri: &'a str,
}

impl Default for Metadata<'_> {
    fn default() -> Self {
        Self {
            anise_version: ANISE_VERSION,
            creation_date: Epoch::now().unwrap(),
            originator: Default::default(),
            metadata_uri: Default::default(),
        }
    }
}

impl<'a> Encode for Metadata<'a> {
    fn encoded_len(&self) -> der::Result<der::Length> {
        self.anise_version.encoded_len()?
            + self.creation_date.encoded_len()?
            + Utf8StringRef::new(self.originator)?.encoded_len()?
            + Utf8StringRef::new(self.metadata_uri)?.encoded_len()?
    }

    fn encode(&self, encoder: &mut dyn Writer) -> der::Result<()> {
        self.anise_version.encode(encoder)?;
        self.creation_date.encode(encoder)?;
        Utf8StringRef::new(self.originator)?.encode(encoder)?;
        Utf8StringRef::new(self.metadata_uri)?.encode(encoder)
    }
}

impl<'a> Decode<'a> for Metadata<'a> {
    fn decode<R: Reader<'a>>(decoder: &mut R) -> der::Result<Self> {
        Ok(Self {
            anise_version: decoder.decode()?,
            creation_date: decoder.decode()?,
            originator: decoder.decode::<Utf8StringRef<'a>>()?.as_str(),
            metadata_uri: decoder.decode::<Utf8StringRef<'a>>()?.as_str(),
        })
    }
}

impl<'a> fmt::Display for Metadata<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ANISE version {}", self.anise_version)?;
        writeln!(
            f,
            "Originator: {}",
            if self.originator.is_empty() {
                "(not set)"
            } else {
                self.originator
            }
        )?;
        writeln!(f, "Creation date: {}", self.creation_date)?;
        writeln!(
            f,
            "Metadata URI: {}",
            if self.metadata_uri.is_empty() {
                "(not set)"
            } else {
                self.metadata_uri
            }
        )
    }
}