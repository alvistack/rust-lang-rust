%global debug_package %{nil}

Name: rustc
Version: __VERSION__
Release: 1%{?dist}
Summary: Rust systems programming language
License: Apache-2.0
URL: http://www.rust-lang.org
Source0: %{name}_%{version}.orig.tar.gz
BuildRequires: chrpath
BuildRequires: fdupes
Requires: rust-std-static = %{version}-%{release}
Requires: /usr/bin/cc
Requires: glibc

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


%package -n rust-std-static
Summary: Rust standard libraries - development files

%description -n rust-std-static
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
 .
 This package contains development files for the standard Rust libraries,
 needed to compile Rust programs. It may also be installed on a system
 of another host architecture, for cross-compiling to this architecture.


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
install -Dpm755 -d %{buildroot}%{_bindir}
install -Dpm755 -d %{buildroot}%{_datadir}/zsh
install -Dpm755 -d %{buildroot}%{_datadir}/bash-completion/completions
install -Dpm755 -d %{buildroot}%{_docdir}
install -Dpm755 -d %{buildroot}%{_libdir}
install -Dpm755 -d %{buildroot}%{_libdir}/rustlib
install -Dpm755 -d %{buildroot}%{_mandir}/man1
install -Dpm755 -d %{buildroot}%{_prefix}
install -Dpm755 -d %{buildroot}%{_prefix}/libexec
install -Dpm755 -d %{buildroot}%{_sysconfdir}
./install.sh \
    --destdir=%{buildroot} \
    --bindir=%{buildroot}%{_bindir} \
    --docdir=%{buildroot}%{_docdir}/rust \
    --libdir=%{buildroot}%{_libdir} \
    --mandir=%{buildroot}%{_mandir} \
    --prefix=%{_prefix} \
    --sysconfdir=%{buildroot}%{_sysconfdir} \
    --components=rustc \
    --disable-ldconfig \
    --verbose
./install.sh \
    --destdir=%{buildroot} \
    --libdir=%{buildroot}%{_libdir} \
    --prefix=%{_prefix} \
    --components=rust-std-x86_64-unknown-linux-gnu \
    --disable-ldconfig \
    --verbose
./install.sh \
    --destdir=%{buildroot} \
    --bindir=%{buildroot}%{_bindir} \
    --docdir=%{buildroot}%{_docdir}/cargo \
    --libdir=%{buildroot}%{_libdir} \
    --mandir=%{buildroot}%{_mandir} \
    --prefix=%{_prefix} \
    --sysconfdir=%{buildroot}%{_sysconfdir} \
    --components=cargo \
    --disable-ldconfig \
    --verbose
mv %{buildroot}%{_sysconfdir}/bash_completion.d/cargo %{buildroot}%{_datadir}/bash-completion/completions/
find %{buildroot} -type f -name "manifest-*" -exec rm -rf {} \;
find %{buildroot} -type f -name "install.log" -exec rm -rf {} \;
find %{buildroot} -type f -name "*.so" -exec chrpath -r %{_libdir} {} \;
rm -rf %{buildroot}%{_libdir}/rustlib/components
rm -rf %{buildroot}%{_libdir}/rustlib/rust-installer-version
rm -rf %{buildroot}%{_libdir}/rustlib/uninstall.sh
rm -rf %{buildroot}%{_prefix}/libexec
%fdupes -s %{buildroot}%{_libdir}/rustlib


%ldconfig_scriptlets


%files
%dir %{_docdir}/rust
%dir %{_libdir}/rustlib
%dir %{_libdir}/rustlib/etc
%dir %{_libdir}/rustlib/x86_64-unknown-linux-gnu
%dir %{_libdir}/rustlib/x86_64-unknown-linux-gnu/bin
%{_bindir}/rust-gdb
%{_bindir}/rust-gdbgui
%{_bindir}/rust-lldb
%{_bindir}/rustc
%{_bindir}/rustdoc
%{_docdir}/rust/*
%{_libdir}/*.so
%{_libdir}/rustlib/etc/*
%{_libdir}/rustlib/x86_64-unknown-linux-gnu/bin/*
%{_mandir}/man1/rust*.1*


%files -n rust-std-static
%dir %{_libdir}/rustlib/x86_64-unknown-linux-gnu/lib
%{_libdir}/rustlib/x86_64-unknown-linux-gnu/lib/*


%files -n cargo
%dir %{_datadir}/zsh
%dir %{_datadir}/zsh/site-functions
%dir %{_docdir}/cargo
%{_bindir}/cargo
%{_datadir}/bash-completion/completions/cargo
%{_datadir}/zsh/site-functions/_cargo
%{_docdir}/cargo/*
%{_mandir}/man1/cargo*.1*


%changelog
* __DATE__ Wong Hoi Sing Edison <hswong3i@gmail.com> - __VERSION__-1
- https://github.com/rust-lang/rust/blob/master/RELEASES.md
