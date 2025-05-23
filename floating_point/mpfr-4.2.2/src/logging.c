/* MPFR Logging functions.

Copyright 2005-2025 Free Software Foundation, Inc.
Contributed by the Pascaline and Caramba projects, INRIA.

This file is part of the GNU MPFR Library.

The GNU MPFR Library is free software; you can redistribute it and/or modify
it under the terms of the GNU Lesser General Public License as published by
the Free Software Foundation; either version 3 of the License, or (at your
option) any later version.

The GNU MPFR Library is distributed in the hope that it will be useful, but
WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public
License for more details.

You should have received a copy of the GNU Lesser General Public License
along with the GNU MPFR Library; see the file COPYING.LESSER.
If not, see <https://www.gnu.org/licenses/>. */

#include "mpfr-impl.h"

/* Logging MPFR needs GCC >= 3.0 and GLIBC >= 2.0. */

#ifdef MPFR_USE_LOGGING

/* The <time.h> header might not be available everywhere (it is standard,
   but not even required by freestanding C implementations); thus it isn't
   included unconditionally. */
#include <time.h>

/* Define LOGGING variables */

FILE *mpfr_log_file;
int   mpfr_log_flush;
int   mpfr_log_type;
int   mpfr_log_level;
int   mpfr_log_current;
int   mpfr_log_worstcase_limit;
mpfr_prec_t mpfr_log_prec;

static void mpfr_log_begin (void) __attribute__((constructor));

/* We let the system close the LOG itself
   (Otherwise functions called by destructor can't use LOG File */
static void
mpfr_log_begin (void)
{
  const char *var;

  /* Grab some information */
  var = getenv ("MPFR_LOG_LEVEL");
  mpfr_log_level = var == NULL || *var == 0 ? 7 : atoi (var);
  mpfr_log_current = 0;

  var = getenv ("MPFR_LOG_PREC");
  mpfr_log_prec = var == NULL ? 6 : atol (var);

  /* Get what we need to log */
  mpfr_log_type = 0;
  if (getenv ("MPFR_LOG_INPUT") != NULL)
    mpfr_log_type |= MPFR_LOG_INPUT_F;
  if (getenv ("MPFR_LOG_OUTPUT") != NULL)
    mpfr_log_type |= MPFR_LOG_OUTPUT_F;
  if (getenv ("MPFR_LOG_TIME") != NULL)
    mpfr_log_type |= MPFR_LOG_TIME_F;
  if (getenv ("MPFR_LOG_INTERNAL") != NULL)
    mpfr_log_type |= MPFR_LOG_INTERNAL_F;
  if (getenv ("MPFR_LOG_MSG") != NULL)
    mpfr_log_type |= MPFR_LOG_MSG_F;
  if (getenv ("MPFR_LOG_ZIV") != NULL)
    mpfr_log_type |= MPFR_LOG_BADCASE_F;
  if (getenv ("MPFR_LOG_STAT") != NULL)
    mpfr_log_type |= MPFR_LOG_STAT_F;
  if (getenv ("MPFR_LOG_ALL") != NULL)
    mpfr_log_type = MPFR_LOG_INPUT_F|MPFR_LOG_OUTPUT_F|MPFR_LOG_TIME_F
      |MPFR_LOG_INTERNAL_F|MPFR_LOG_MSG_F|MPFR_LOG_BADCASE_F|MPFR_LOG_STAT_F;

  mpfr_log_flush = getenv ("MPFR_LOG_FLUSH") != NULL;

  /* Open filename if needed */
  var = getenv ("MPFR_LOG_FILE");
  if (var == NULL || *var == 0)
    var = "mpfr.log";
  if (mpfr_log_type != 0)
    {
      time_t tt;
      struct tm *tm_p;
      char s[32];  /* a bit more than needed, just in case */
      size_t r;

      mpfr_log_file = fopen (var, "w");
      if (mpfr_log_file == NULL)
        {
          fprintf (stderr, "MPFR LOG: Can't open '%s' with w.\n", var);
          abort ();
        }
      time (&tt);
      tm_p = localtime (&tt);
      r = strftime (s, sizeof s, "%Y-%m-%d %H:%M:%S", tm_p);

      fprintf (mpfr_log_file, "MPFR LOG FILE %s\n",
               r != 0 ? s : "[strftime failed]");
      fflush (mpfr_log_file);  /* always done */
    }
}

/* Return user CPU time measured in milliseconds. Thanks to Torbjorn. */

#if defined (ANSIONLY) || defined (USG) || defined (__SVR4) \
 || defined (_UNICOS) || defined(__hpux)

int
mpfr_get_cputime (void)
{
  return (int) ((unsigned long long) clock () * 1000 / CLOCKS_PER_SEC);
}

#else /* Use getrusage for cputime */

#include <sys/types.h>
#include <sys/resource.h>

int
mpfr_get_cputime (void)
{
  struct rusage rus;
  getrusage (0, &rus);
  return rus.ru_utime.tv_sec * 1000 + rus.ru_utime.tv_usec / 1000;
}

#endif /* cputime */

#endif /* MPFR_USE_LOGGING */
