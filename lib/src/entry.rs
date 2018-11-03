// Copyright 2018 Matthew Kousoulas
// This file is part of Stabacus.
//
// Stabacus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Stabacus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Stabacus.  If not, see <https://www.gnu.org/licenses/>.
//
// @license GPL-3.0-or-later <http://spdx.org/licenses/GPL-3.0-or-later>

use std::fmt;
use std::str::FromStr;
use std::string::ParseError;

use operator::*;

#[derive(Clone,Debug)]
pub enum Entry {
	Op(Operator),
	Int(i64),
	Panic(String),
	Die,
	Pop,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Entry::Op(o)    => write!(f, "{}",  o),
			Entry::Int(n)   => write!(f, "{}",  n),
			Entry::Panic(s) => write!(f, "!{}", s),
			Entry::Die      => write!(f, "Die"),
			Entry::Pop      => write!(f, "Pop"),
		}
    }
}

#[derive(Debug)]
pub enum ParseType {
	Str(String),
	Int(i64),
}

impl FromStr for ParseType {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.parse::<i64>() {
			Ok(n)  => Ok(ParseType::Int(n)),
			Err(_) => Ok(ParseType::Str(s.to_string())),
		}
	}
}

