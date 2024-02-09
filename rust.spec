# Copyright 2024 Wong Hoi Sing Edison <hswong3i@pantarei-design.com>
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

%global debug_package %{nil}

%global source_date_epoch_from_changelog 0

%global __strip /bin/true

%global __brp_mangle_shebangs /bin/true

%define _use_internal_dependency_generator 0
%define __find_requires %{nil}
%global __spec_install_post \
    /usr/lib/rpm/check-buildroot \
    /usr/lib/rpm/brp-compress

%undefine _build_create_debug
%define __arch_install_post export NO_BRP_STRIP_DEBUG=true NO_BRP_AR=true

Name: rust
Epoch: 100
Version: 1.76.0
Release: 1%{?dist}
Summary: Rust systems programming language
License: Apache-2.0
URL: https://github.com/rust-lang/rust/tags
Source0: %{name}_%{version}.orig.tar.gz
Source99: %{name}.rpmlintrc
BuildRequires: chrpath
BuildRequires: fdupes
BuildRequires: pkgconfig
Requires: binutils >= 2.26
Requires: gcc
Requires: glibc-devel
Requires: libLLVM-17-rust-1_76_0-stable = %{epoch}:%{version}-%{release}

%description
Rust is a curly-brace, block-structured expression language. It visually
resembles the C language family, but differs significantly in syntactic
and semantic details. Its design is oriented toward concerns of
"programming in the large", that is, of creating and maintaining
boundaries - both abstract and operational - that preserve large-system
integrity, availability and concurrency.

%prep
%autosetup -T -c -n %{name}_%{version}-%{release}
tar -zx -f %{S:0} --strip-components=1 -C .

%build

%install
./install.sh \
    --destdir=%{buildroot} \
    --prefix=%{_prefix} \
    --sysconfdir=%{buildroot}%{_sysconfdir} \
    --bindir=%{buildroot}%{_bindir} \
    --libdir=%{buildroot}%{_prefix}/lib \
    --datadir=%{buildroot}%{_datadir} \
    --mandir=%{buildroot}%{_mandir} \
    --docdir=%{buildroot}%{_docdir} \
    --components=rustc,cargo,rust-std-x86_64-unknown-linux-gnu \
    --disable-ldconfig \
    --verbose
mkdir -p %{buildroot}%{_datadir}/bash-completion/completions
mv %{buildroot}/etc/bash_completion.d/* %{buildroot}%{_datadir}/bash-completion/completions/
rm -rf %{buildroot}%{_prefix}/lib/rustlib/install.log
rm -rf %{buildroot}%{_prefix}/lib/rustlib/manifest-*
rm -rf %{buildroot}%{_prefix}/lib/rustlib/uninstall.sh
rm -rf %{buildroot}%{_prefix}/lib/rustlib/rust-installer-version
rm -rf %{buildroot}%{_prefix}/libexec/rust-analyzer-proc-macro-srv
rm -rf %{buildroot}%{_docdir}/*
chrpath -d %{buildroot}%{_prefix}/lib/rustlib/*/bin/rust-lld
fdupes -qnrps %{buildroot}

%check

%package -n libLLVM-17-rust-1_76_0-stable
Summary: Library for libLLVM-17-rust-1.76.0-stable.so

%description -n libLLVM-17-rust-1_76_0-stable
Library for libLLVM-17-rust-1.76.0-stable.so.

%package -n cargo
Summary: Rust package manager
Requires: binutils
Requires: gcc
Requires: rust = %{epoch}:%{version}-%{release}

%description -n cargo
Cargo is a tool that allows Rust projects to declare their various
dependencies, and ensure that you'll always get a repeatable build.

%post -p /sbin/ldconfig
%postun -p /sbin/ldconfig

%files
%license COPYRIGHT
%dir %{_prefix}/lib/rustlib
%exclude %{_prefix}/lib/libLLVM-17-rust-1.76.0-stable.so
%{_bindir}/rust-gdb
%{_bindir}/rust-gdbgui
%{_bindir}/rust-lldb
%{_bindir}/rustc
%{_bindir}/rustdoc
%{_mandir}/man1/rust*.1*
%{_prefix}/lib/*.so
%{_prefix}/lib/rustlib/*

%files -n libLLVM-17-rust-1_76_0-stable
%{_prefix}/lib/libLLVM-17-rust-1.76.0-stable.so

%files -n cargo
%license COPYRIGHT
%dir %{_datadir}/zsh
%dir %{_datadir}/zsh/site-functions
%dir %{_prefix}/libexec
%{_bindir}/cargo
%{_datadir}/bash-completion/completions/cargo
%{_datadir}/zsh/site-functions/_cargo
%{_mandir}/man1/cargo*.1*

%changelog
