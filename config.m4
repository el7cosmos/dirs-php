PHP_ARG_ENABLE([dirs],
  [whether to enable dirs support],
  [AS_HELP_STRING([--enable-dirs],
    [Enable dirs support])],
  [no])

dnl If not enable, `cargo build` run with argument `--release`.
PHP_ARG_ENABLE([cargo_debug],
	[whether to enable cargo debug mode],
	[AS_HELP_STRING([--enable-cargo-debug],
		[Enable cargo debug])],
	[no],
	[no])

if test "$PHP_DIRS" != "no"; then
  dnl Check cargo command exists or not.
  AC_PATH_PROG(CARGO, cargo, no)
  if ! test -x "$CARGO"; then
    AC_MSG_ERROR([cargo command missing, please reinstall the cargo distribution])
  fi

  AC_DEFINE([HAVE_DIRS], [1], [ Define to 1 if the PHP extension 'dirs' is available. ])

  PHP_NEW_EXTENSION(dirs, [ ])
  PHP_ADD_MAKEFILE_FRAGMENT()

  CARGO_MODE_FLAGS="--release"
  CARGO_MODE_DIR="release"

  if test "$PHP_CARGO_DEBUG" != "no"; then
    CARGO_MODE_FLAGS=""
    CARGO_MODE_DIR="debug"
  fi

  PHP_SUBST([CARGO_MODE_FLAGS])
  PHP_SUBST([CARGO_MODE_DIR])

  dnl Symbolic link the files for `cargo build`
  AC_CONFIG_LINKS([ \
    Cargo.lock:Cargo.lock \
    Cargo.toml:Cargo.toml \
    src:src \
	])
fi
