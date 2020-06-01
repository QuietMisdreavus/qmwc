// qmwc - QuietMisdreavus Wallpaper Cycler
// Copyright (C) 2020 QuietMisdreavus
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::collections::BTreeSet;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::path::PathBuf;
use std::process::Command;

fn args() -> clap::App<'static, 'static> {
    clap::App::new("QuietMisdreavus Wallpaper Cycler")
        .author("(c) 2020 QuietMisdreavus")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A script to manage my wallpaper on GNOME 3")
        .arg(clap::Arg::with_name("set-dir")
            .long("set-dir")
            .takes_value(true)
            .value_name("DIR")
            .help("Sets the directory used to source wallpaper"))
}

fn main() -> io::Result<()> {
    let args = args().get_matches();

    if let Some(wall_dir) = args.value_of("set-dir") {
        println!("Setting wallpaper directory to {}", wall_dir);
        set_wallpaper_dir(wall_dir)?;
    } else {
        let next = get_next_wallpaper()?;
        println!("Setting next wallpaper to {}", next.display());
        set_next_wallpaper(next)?;
    }

    Ok(())
}

fn get_config_file() -> io::Result<PathBuf> {
    let mut c = dirs::config_dir().expect("could not locate XDG Config folder");
    c.push("qmwc");
    fs::create_dir_all(&c)?;
    c.push("config.txt");
    Ok(c)
}

fn set_wallpaper_dir(dir: &str) -> io::Result<()> {
    let f = get_config_file()?;
    fs::write(&f, dir)?;
    Ok(())
}

fn get_wallpaper_dir() -> io::Result<PathBuf> {
    let f = get_config_file()?;
    if !f.exists() {
        eprintln!("Wallpaper directory not set. Call this program with `--set-dir` first to set the directory.");
        return Err(io::Error::new(io::ErrorKind::NotFound, "configuration not found"));
    }
    let dir = fs::read_to_string(&f)?;
    Ok(PathBuf::from(dir))
}

fn get_cache_file() -> io::Result<PathBuf> {
    let mut c = dirs::cache_dir().expect("could not locate XDG Cache folder");
    c.push("qmwc");
    fs::create_dir_all(&c)?;
    c.push("current.txt");
    Ok(c)
}

fn get_current_wallpaper() -> io::Result<Option<PathBuf>> {
    let f = get_cache_file()?;
    if f.exists() {
        let buf = fs::read(&f)?;
        let oss = OsString::from_vec(buf);
        Ok(Some(PathBuf::from(oss)))
    } else {
        Ok(None)
    }
}

fn get_next_wallpaper() -> io::Result<PathBuf> {
    let dir = get_wallpaper_dir()?;

    let mut walls = fs::read_dir(&dir)?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_file())
        .collect::<BTreeSet<_>>();

    if let Some(curr) = get_current_wallpaper()? {
        let next_walls = walls.split_off(&curr);
        if let Some(next) = next_walls.into_iter().find(|p| p != &curr) {
            // return the file after the current one
            Ok(next)
        } else if let Some(next) = walls.into_iter().next() {
            // if the current file is the last one in the folder, return the first one instead
            Ok(next)
        } else {
            eprintln!("No wallpapers found in wallpaper directory.");
            Err(io::Error::new(io::ErrorKind::NotFound, "no wallpapers in directory"))
        }
    } else if let Some(first) = walls.into_iter().next() {
        Ok(first)
    } else {
        eprintln!("No wallpapers found in wallpaper directory.");
        Err(io::Error::new(io::ErrorKind::NotFound, "no wallpapers in directory"))
    }
}

fn set_next_wallpaper(next: PathBuf) -> io::Result<()> {
    let c = get_cache_file()?;
    fs::write(&c, next.as_os_str().as_bytes())?;

    let mut path_arg = OsString::from("file://");
    path_arg.push(&next);

    let cmd = Command::new("gsettings")
        .arg("set")
        .arg("org.gnome.desktop.background")
        .arg("picture-uri")
        .arg(&path_arg)
        .status()?;

    if !cmd.success() {
        eprintln!("ERROR: the `gsettings` command exited in failure. Code: {:?}", cmd.code());
        return Err(io::Error::new(io::ErrorKind::Other, "could not set wallpaper"));
    }

    Ok(())
}
