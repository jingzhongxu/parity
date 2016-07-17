// Copyright 2015, 2016 Ethcore (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

//! Path utilities
use std::path::Path;
use std::path::PathBuf;

#[cfg(target_os = "macos")]
/// Get the config path for application `name`.
/// `name` should be capitalized, e.g. `"Ethereum"`, `"Parity"`.
pub fn config_path(name: &str) -> PathBuf {
	let mut home = ::std::env::home_dir().expect("Failed to get home dir");
	home.push("Library");
	home.push(name);
	home
}

#[cfg(windows)]
/// Get the config path for application `name`.
/// `name` should be capitalized, e.g. `"Ethereum"`, `"Parity"`.
pub fn config_path(name: &str) -> PathBuf {
	let mut home = ::std::env::home_dir().expect("Failed to get home dir");
	home.push("AppData");
	home.push("Roaming");
	home.push(name);
	home
}

#[cfg(not(any(target_os = "macos", windows)))]
/// Get the config path for application `name`.
/// `name` should be capitalized, e.g. `"Ethereum"`, `"Parity"`.
pub fn config_path(name: &str) -> PathBuf {
	let mut home = ::std::env::home_dir().expect("Failed to get home dir");
	home.push(format!(".{}", name.to_lowercase()));
	home
}

/// Get the specific folder inside a config path.
pub fn config_path_with(name: &str, then: &str) -> PathBuf {
	let mut path = config_path(name);
	path.push(then);
	path
}

/// Default ethereum paths
pub mod ethereum {
	use std::path::PathBuf;

	/// Default path for ethereum installation on Mac Os
	pub fn default() -> PathBuf { super::config_path("Ethereum") }

	/// Get the specific folder inside default ethereum installation
	pub fn with_default(s: &str) -> PathBuf {
		let mut path = default();
		path.push(s);
		path
	}

	/// Get the specific folder inside default ethereum installation configured for testnet
	pub fn with_testnet(s: &str) -> PathBuf {
		let mut path = default();
		path.push("testnet");
		path.push(s);
		path
	}
}

/// Restricts the permissions of given path only to the owner.
#[cfg(not(windows))]
pub fn restrict_permissions_owner(file_path: &Path) -> Result<(), i32>  {
	let cstr = ::std::ffi::CString::new(file_path.to_str().unwrap()).unwrap();
	match unsafe { ::libc::chmod(cstr.as_ptr(), ::libc::S_IWUSR | ::libc::S_IRUSR) } {
		0 => Ok(()),
		x => Err(x),
	}
}

/// Restricts the permissions of given path only to the owner.
#[cfg(windows)]
pub fn restrict_permissions_owner(_file_path: &Path) -> Result<(), i32>  {
	//TODO: implement me
	Ok(())
}

