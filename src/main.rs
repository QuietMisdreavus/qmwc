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

fn main() {
    let args = args().get_matches();

    if let Some(wall_dir) = args.value_of("set-dir") {
        println!("Setting wallpaper directory to {}", wall_dir);
    } else {
        println!("TODO");
    }
}
