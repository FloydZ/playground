%define name cln
%define version 1.3.7
%define release 1

Summary: Class Library for Numbers
Name: %{name}
Version: %{version}
Release: %{release}
License: GPL
Group: System Environment/Libraries
Source0: %{name}-%{version}.tar.bz2
URL: http://www.ginac.de/CLN/
BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root-%(%{__id_u} -n)
Prefix: %{_prefix}

Requires(post): /sbin/install-info
Requires(preun): /sbin/install-info
BuildRequires: gcc-c++

%description
A GPLed collection of math classes and functions, that will bring
efficiency, type safety, algebraic syntax to everyone in a memory
and speed efficient library.

%package devel
Summary: Development files for programs using the CLN library
Group: Development/Libraries
Requires: %{name} = %{version}-%{release}

%description devel
This package is necessary if you wish to develop software based on
the CLN library.

%prep
%setup -q

%build
%configure
make

%install
rm -rf ${RPM_BUILD_ROOT}
%makeinstall
mkdir -p ${RPM_BUILD_ROOT}%{_defaultdocdir}/%{name}-devel-%{version}
mv ${RPM_BUILD_ROOT}%{_datadir}/dvi/cln.dvi ${RPM_BUILD_ROOT}%{_defaultdocdir}/%{name}-devel-%{version}
rmdir ${RPM_BUILD_ROOT}%{_datadir}/dvi
mv ${RPM_BUILD_ROOT}%{_datadir}/html ${RPM_BUILD_ROOT}%{_defaultdocdir}/%{name}-devel-%{version}

%clean
rm -rf ${RPM_BUILD_ROOT}

%post -p /sbin/ldconfig

%postun -p /sbin/ldconfig

%post devel
/sbin/install-info --section="Math" --info-dir=%{_infodir} %{_infodir}/cln.info.gz %{_infodir}/dir 2>/dev/null || :

%preun devel
if [ "$1" = 0 ]; then
  /sbin/install-info --delete --info-dir=%{_infodir} %{_infodir}/cln.info.gz %{_infodir}/dir 2>/dev/null || :
fi

%files
%defattr(-,root,root)
%doc COPYING ChangeLog FILES NEWS README TODO*
%{_libdir}/*.so.*

%files devel
%defattr(-,root,root)
%{_defaultdocdir}/%{name}-devel-%{version}
%{_libdir}/*.a
%{_libdir}/*.la
%{_libdir}/*.so
%{_libdir}/pkgconfig/cln.pc
%{_includedir}/cln
%{_infodir}/*.info*
%{_mandir}/man1/cln-config.1*
%{_bindir}/cln-config
%{_datadir}/aclocal/cln.m4

%changelog
* Wed Jun 20 2007 Markus Grabner <grabner@icg.tugraz.at>
  Source0 is bzip2-compressed
* Tue Oct 25 2005 Christian Bauer <Christian.Bauer@uni-mainz.de>
  - "Copyright:" -> "License:"
  - Fixed broken install-info command
  - Added missing BuildRequires
  - Added release to Requires for devel
  - Remove processing of info files (this is supposed to be automatic)
* Thu Nov 20 2003 Christian Bauer <Christian.Bauer@uni-mainz.de>
  Added pkg-config metadata file to devel package
* Wed Nov  6 2002 Christian Bauer <Christian.Bauer@uni-mainz.de>
  Added HTML and DVI docs to devel package
* Tue Nov  5 2001 Christian Bauer <Christian.Bauer@uni-mainz.de>
  Added Packager
