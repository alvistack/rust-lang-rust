%global debug_package %{nil}

Name: rustc
Version: __VERSION__
Release: 1%{?dist}
Summary: Rust systems programming language
License: Apache-2.0
URL: http://www.rust-lang.org
Source0: %{name}_%{version}.orig.tar.gz
Requires: %{name}-std-static%{?_isa} = %{version}-%{release}
Requires: /usr/bin/cc

%description
Rust is a curly-brace, block-structured expression language.  It
visually resembles the C language family, but differs significantly
in syntactic and semantic details.  Its design is oriented toward
concerns of "programming in the large", that is, of creating and
maintaining boundaries - both abstract and operational - that
preserve large-system integrity, availability and concurrency.
.
It supports a mixture of imperative procedural, concurrent actor,
object-oriented and pure functional styles.  Rust also supports
generic programming and meta-programming, in both static and dynamic
styles.

%package -n cargo
Summary: Rust package manager
Requires: rustc

%description -n cargo
Cargo is a tool that allows Rust projects to declare their various
dependencies, and ensure that you'll always get a repeatable build.
.
To accomplish this goal, Cargo does four things:
* Introduces two metadata files with various bits of project information.
* Fetches and builds your project's dependencies.
* Invokes rustc or another build tool with the correct parameters to build
your project.
* Introduces conventions, making working with Rust projects easier.
.
Cargo downloads your Rust project’s dependencies and compiles your
project.

%prep
%autosetup -T -c -n rustc_%{version}-%{release}
tar -zx -f %{S:0} --strip-components=1 -C .

%build

%install
./install.sh --prefix=%{buildroot}%{_prefix} --components=rustc,cargo

%files
%dir %{_libdir}/rustlib
%{_bindir}/rust-gdb
%{_bindir}/rust-gdbgui
%{_bindir}/rust-lldb
%{_bindir}/rustc
%{_bindir}/rustdoc
%{_docdir}/rust
%{_libdir}/*.so
%{_mandir}/man1/rust*.1*

%files -n cargo
%dir %{_docdir}/cargo
%{_bindir}/cargo
%{_datadir}/zsh/site-functions/_cargo
%{_libexecdir}/cargo*
%{_mandir}/man1/cargo*.1*
%{_sysconfdir}/bash_completion.d/cargo

%changelog
* __DATE__ Wong Hoi Sing Edison <hswong3i@gmail.com> - __VERSION__-1
- https://github.com/rust-lang/rust/blob/master/RELEASES.md
