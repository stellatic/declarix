/*
Copyright (C) 2024  StarlightStargaze

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/
pub struct Prog {
    pub prog: String,
    pub install: Vec<String>,
    pub uninstall: Vec<String>,
    pub checker: Vec<String>,
    pub packages: Vec<String>,
}

impl Prog {
    fn new<'a>(prog: &'a str, install: impl IntoIterator<Item = &'a str>, uninstall: impl IntoIterator<Item = &'a str>, checker: impl IntoIterator<Item = &'a str>) -> Self {
        Self {
            prog: prog.to_string(),
            install: install.into_iter().map(String::from).collect(),
            uninstall: uninstall.into_iter().map(String::from).collect(),
            checker: checker.into_iter().map(String::from).collect(),
            packages: Vec::new()
        }
    }
}

pub struct Arch {
    pub prog: Prog,
}

pub struct Vsc {
    pub prog: Prog
}

pub struct Flatpak {
    pub prog: Prog
}

pub struct Debian {
    pub prog: Prog
}

pub struct OpenSUSE {
    pub prog: Prog
}

pub trait Builder {
    fn new(prog: &str) -> Self;
}

impl Builder for Debian {
    fn new(prog: &str) -> Self {
        Self {
            prog: Prog::new("sudo",
                [prog, "-y", "install"],
                [prog, "-y", "remove"], 
                [prog, "list", "--installed"]
            )
        }
    }
}

impl Builder for OpenSUSE {
    fn new(prog: &str) -> Self {
        Self {
            prog: Prog::new("sudo",
            [prog, "-n", "install"],
            [prog, "-n", "remove"],
            [prog, "search", "-i"])
        }
    }
}

impl Builder for Arch {
    fn new(prog: &str) -> Self {
        let (mut app,mut install, mut uninstall, mut checker) = (prog, vec!["-S", "--noconfirm"], vec!["-Rns", "--noconfirm"], vec!["-Q"]); 
        match prog {
            "pacman" => { 
                app = "sudo"; install.insert(0, prog); uninstall.insert(0, prog); checker.insert(0, prog) 
            },
            &_ => {}
        }
        Self { prog: Prog::new(app, install, uninstall, checker)  }
    }
}

impl Builder for Vsc {
    fn new(prog: &str) -> Self {
        let mut _app = "";
        match prog {
            "vscodium" => { _app = prog }
            &_ => { _app = "code" }
        }
        Self {
            prog: Prog::new(
                _app, 
                ["--install-extension"],
                ["--uninstall-extension"],
                ["--list-extensions"])
        }
    }
}

impl Builder for Flatpak {
    fn new(prog: &str) -> Self {
        Self {
            prog: Prog::new(
                prog,
                ["install", "-y"],
                ["uninstall", "-y"],
            ["list"])
        }
    }
}







