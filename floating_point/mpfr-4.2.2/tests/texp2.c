/* Test file for mpfr_exp2.

Copyright 2001-2004, 2006-2025 Free Software Foundation, Inc.
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

#include "mpfr-test.h"

#define TEST_FUNCTION mpfr_exp2
#define TEST_RANDOM_EMIN -36
#define TEST_RANDOM_EMAX 36
#include "tgeneric.c"

static void
special_overflow (void)
{
  mpfr_t x, y;
  int inex;
  mpfr_exp_t emin, emax;

  emin = mpfr_get_emin ();
  emax = mpfr_get_emax ();

  set_emin (-125);
  set_emax (128);

  mpfr_init2 (x, 24);
  mpfr_init2 (y, 24);

  mpfr_set_str_binary (x, "0.101100100000000000110100E15");
  inex = mpfr_exp2 (y, x, MPFR_RNDN);
  if (!mpfr_inf_p (y) || inex <= 0)
    {
      printf ("Overflow error.\n");
      mpfr_dump (y);
      printf ("inex = %d\n", inex);
      exit (1);
    }

  mpfr_clear (y);
  mpfr_clear (x);
  set_emin (emin);
  set_emax (emax);
}

static void
emax_m_eps (void)
{
  if (mpfr_get_emax () <= LONG_MAX)
    {
      mpfr_t x, y;
      int inex, ov;

      mpfr_init2 (x, sizeof(mpfr_exp_t) * CHAR_BIT * 4);
      mpfr_init2 (y, 8);
      mpfr_set_si (x, mpfr_get_emax (), MPFR_RNDN);

      mpfr_clear_flags ();
      inex = mpfr_exp2 (y, x, MPFR_RNDN);
      ov = mpfr_overflow_p ();
      if (!ov || !mpfr_inf_p (y) || inex <= 0)
        {
          printf ("Overflow error for x = emax, MPFR_RNDN.\n");
          mpfr_dump (y);
          printf ("inex = %d, %soverflow\n", inex, ov ? "" : "no ");
          exit (1);
        }

      mpfr_nextbelow (x);

      mpfr_clear_flags ();
      inex = mpfr_exp2 (y, x, MPFR_RNDN);
      ov = mpfr_overflow_p ();
      if (!ov || !mpfr_inf_p (y) || inex <= 0)
        {
          printf ("Overflow error for x = emax - eps, MPFR_RNDN.\n");
          mpfr_dump (y);
          printf ("inex = %d, %soverflow\n", inex, ov ? "" : "no ");
          exit (1);
        }

      mpfr_clear_flags ();
      inex = mpfr_exp2 (y, x, MPFR_RNDD);
      ov = mpfr_overflow_p ();
      if (ov || mpfr_inf_p (y) || inex >= 0 ||
          (mpfr_nextabove (y), !mpfr_inf_p (y)))
        {
          printf ("Overflow error for x = emax - eps, MPFR_RNDD.\n");
          mpfr_dump (y);
          printf ("inex = %d, %soverflow\n", inex, ov ? "" : "no ");
          exit (1);
        }

      mpfr_clear (x);
      mpfr_clear (y);
    }
}

static void
exp_range (void)
{
  mpfr_t x;
  mpfr_exp_t emin;

  emin = mpfr_get_emin ();
  set_emin (3);
  mpfr_init2 (x, 8);
  mpfr_set_ui (x, 5, MPFR_RNDN);
  mpfr_exp2 (x, x, MPFR_RNDN);
  set_emin (emin);
  if (mpfr_nan_p (x) || mpfr_cmp_ui (x, 32) != 0)
    {
      printf ("Error in mpfr_exp2 for x = 5, with emin = 3\n");
      printf ("Expected 32, got ");
      mpfr_out_str (stdout, 2, 0, x, MPFR_RNDN);
      printf ("\n");
      exit (1);
    }
  mpfr_clear (x);
}

static void
overflowed_exp2_0 (void)
{
  mpfr_t x, y;
  int emax, i, inex, rnd, err = 0;
  mpfr_exp_t old_emax;

  old_emax = mpfr_get_emax ();

  mpfr_init2 (x, 8);
  mpfr_init2 (y, 8);

  for (emax = -1; emax <= 0; emax++)
    {
      mpfr_set_ui_2exp (y, 1, emax, MPFR_RNDN);
      mpfr_nextbelow (y);
      set_emax (emax);  /* 1 is not representable. */
      /* and if emax < 0, 1 - eps is not representable either. */
      for (i = -1; i <= 1; i++)
        RND_LOOP (rnd)
          {
            mpfr_set_si_2exp (x, i, -512 * ABS (i), MPFR_RNDN);
            mpfr_clear_flags ();
            inex = mpfr_exp2 (x, x, (mpfr_rnd_t) rnd);
            if ((i >= 0 || emax < 0 || rnd == MPFR_RNDN || rnd == MPFR_RNDU) &&
                ! mpfr_overflow_p ())
              {
                printf ("Error in overflowed_exp2_0 (i = %d, rnd = %s):\n"
                        "  The overflow flag is not set.\n",
                        i, mpfr_print_rnd_mode ((mpfr_rnd_t) rnd));
                err = 1;
              }
            if (rnd == MPFR_RNDZ || rnd == MPFR_RNDD)
              {
                if (inex >= 0)
                  {
                    printf ("Error in overflowed_exp2_0 (i = %d, rnd = %s):\n"
                            "  The inexact value must be negative.\n",
                            i, mpfr_print_rnd_mode ((mpfr_rnd_t) rnd));
                    err = 1;
                  }
                if (! mpfr_equal_p (x, y))
                  {
                    printf ("Error in overflowed_exp2_0 (i = %d, rnd = %s):\n"
                            "  Got        ", i,
                            mpfr_print_rnd_mode ((mpfr_rnd_t) rnd));
                    mpfr_dump (x);
                    printf ("  instead of 0.11111111E%d.\n", emax);
                    err = 1;
                  }
              }
            else if (rnd != MPFR_RNDF)
              {
                if (inex <= 0)
                  {
                    printf ("Error in overflowed_exp2_0 (i = %d, rnd = %s):\n"
                            "  The inexact value must be positive.\n",
                            i, mpfr_print_rnd_mode ((mpfr_rnd_t) rnd));
                    err = 1;
                  }
                if (! (mpfr_inf_p (x) && MPFR_IS_POS (x)))
                  {
                    printf ("Error in overflowed_exp2_0 (i = %d, rnd = %s):\n"
                            "  Got        ", i,
                            mpfr_print_rnd_mode ((mpfr_rnd_t) rnd));
                    mpfr_dump (x);
                    printf ("  instead of +Inf.\n");
                    err = 1;
                  }
              }
          }
      set_emax (old_emax);
    }

  if (err)
    exit (1);
  mpfr_clear (x);
  mpfr_clear (y);
}

static void
bug20171218 (void)
{
  mpfr_t x, y, z;
  mpfr_exp_t emin;
  int inex, i;
  mpfr_flags_t flags, ex_flags;

  emin = mpfr_get_emin ();

  mpfr_init2 (x, 228);
  mpfr_init2 (y, 11);
  mpfr_init2 (z, 11);
  mpfr_set_str_binary (x, "-0.110111010100001100000000000000111001100101011011101110101011000011011011001101111111110100110001110100111000111101010010100010001101100001010111101110100010000101011111001101011000011101000000001010001011110011110101010111000000E17");
  set_emin (-113285);
  mpfr_clear_flags ();
  inex = mpfr_exp2 (y, x, MPFR_RNDA);
  /* exact result is 0.11111111111110110000001011...E-113286, which rounded away
     gives 0.10000000000E-113285, i.e., no underflow (after rounding) */
  mpfr_set_str_binary (z, "0.10000000000E-113285");
  MPFR_ASSERTN(mpfr_equal_p (y, z));
  MPFR_ASSERTN(inex > 0);
  MPFR_ASSERTN(mpfr_inexflag_p ());
  MPFR_ASSERTN(!mpfr_underflow_p ());
  mpfr_clear (x);
  mpfr_clear (y);
  mpfr_clear (z);

  mpfr_init2 (x, 256);
  mpfr_init2 (y, 8);
  mpfr_init2 (z, 8);

  for (i = 0; i < 3; i++)
    {
      set_emin (i == 0 ? -17 : i == 1 ? emin : MPFR_EMIN_MIN);
      mpfr_set_exp_t (x, __gmpfr_emin - 2, MPFR_RNDN);
      mpfr_nextabove (x);
      mpfr_set_ui_2exp (z, 1, __gmpfr_emin - 1, MPFR_RNDN);
      ex_flags = MPFR_FLAGS_UNDERFLOW | MPFR_FLAGS_INEXACT;
      mpfr_clear_flags ();
      inex = mpfr_exp2 (y, x, MPFR_RNDN);
      flags = __gmpfr_flags;
      if (! (flags == ex_flags && SAME_SIGN (inex, 1) && mpfr_equal_p (y, z)))
        {
          printf ("Error in bug20171218 for i=%d\n", i);
          printf ("Expected ");
          mpfr_dump (z);
          printf ("  with inex = 1 and flags:");
          flags_out (ex_flags);
          printf ("Got      ");
          mpfr_dump (y);
          printf ("  with inex = %d and flags:", inex);
          flags_out (flags);
          exit (1);
        }
    }

  mpfr_clear (x);
  mpfr_clear (y);
  mpfr_clear (z);

  set_emin (emin);
}

int
main (int argc, char *argv[])
{
  mpfr_t x, y;
  mpfr_exp_t emin, emax;

  tests_start_mpfr ();

  bug20171218 ();
  special_overflow ();
  emax_m_eps ();
  exp_range ();

  mpfr_init (x);
  mpfr_init (y);

  mpfr_set_ui (x, 4, MPFR_RNDN);
  mpfr_exp2 (y, x, MPFR_RNDN);
  if (mpfr_cmp_ui (y, 16) != 0)
    {
      printf ("Error for 2^4, MPFR_RNDN\n");
      exit (1);
    }
  mpfr_exp2 (y, x, MPFR_RNDD);
  if (mpfr_cmp_ui (y, 16) != 0)
    {
      printf ("Error for 2^4, MPFR_RNDD\n");
      exit (1);
    }
  mpfr_exp2 (y, x, MPFR_RNDU);
  if (mpfr_cmp_ui (y, 16) != 0)
    {
      printf ("Error for 2^4, MPFR_RNDU\n");
      exit (1);
    }

  mpfr_set_si (x, -4, MPFR_RNDN);
  mpfr_exp2 (y, x, MPFR_RNDN);
  if (mpfr_cmp_ui_2exp (y, 1, -4) != 0)
    {
      printf ("Error for 2^(-4), MPFR_RNDN\n");
      exit (1);
    }
  mpfr_exp2 (y, x, MPFR_RNDD);
  if (mpfr_cmp_ui_2exp (y, 1, -4) != 0)
    {
      printf ("Error for 2^(-4), MPFR_RNDD\n");
      exit (1);
    }
  mpfr_exp2 (y, x, MPFR_RNDU);
  if (mpfr_cmp_ui_2exp (y, 1, -4) != 0)
    {
      printf ("Error for 2^(-4), MPFR_RNDU\n");
      exit (1);
    }

  mpfr_set_prec (x, 53);
  mpfr_set_prec (y, 53);
  mpfr_set_str (x, /*-1683977482443233.0 / 2199023255552.0*/
                "-7.6578429909351734750089235603809357e2", 10, MPFR_RNDN);
  mpfr_exp2 (y, x, MPFR_RNDN);
  if (mpfr_cmp_str1 (y, "2.991959870867646566478e-231"))
    {
      printf ("Error for x=-1683977482443233/2^41\n");
      exit (1);
    }

  mpfr_set_prec (x, 10);
  mpfr_set_prec (y, 10);
  /* save emin */
  emin = mpfr_get_emin ();
  set_emin (-10);
  mpfr_set_si (x, -12, MPFR_RNDN);
  mpfr_exp2 (y, x, MPFR_RNDN);
  if (MPFR_NOTZERO (y) || MPFR_IS_NEG (y))
    {
      printf ("Error for x=emin-2, RNDN\n");
      printf ("Expected +0\n");
      printf ("Got      "); mpfr_dump (y);
      exit (1);
    }
  /* restore emin */
  set_emin (emin);

  /* save emax */
  emax = mpfr_get_emax ();
  set_emax (10);
  mpfr_set_ui (x, 11, MPFR_RNDN);
  mpfr_exp2 (y, x, MPFR_RNDN);
  if (!mpfr_inf_p (y) || mpfr_sgn (y) < 0)
    {
      printf ("Error for x=emax+1, RNDN\n");
      exit (1);
    }
  /* restore emax */
  set_emax (emax);

  MPFR_SET_INF(x);
  MPFR_SET_POS(x);
  mpfr_exp2 (y, x, MPFR_RNDN);
  if(!MPFR_IS_INF(y))
    {
      printf ("evaluation of function in INF does not return INF\n");
      exit (1);
    }

  MPFR_CHANGE_SIGN(x);
  mpfr_exp2 (y, x, MPFR_RNDN);
  if(!MPFR_IS_ZERO(y))
    {
      printf ("evaluation of function in -INF does not return 0\n");
      exit (1);
    }

  MPFR_SET_NAN(x);
  mpfr_exp2 (y, x, MPFR_RNDN);
  if(!MPFR_IS_NAN(y))
    {
      printf ("evaluation of function in NaN does not return NaN\n");
      exit (1);
    }

  if ((mpfr_uexp_t) 8 << 31 != 0 ||
      mpfr_get_emax () <= (mpfr_uexp_t) 100000 * 100000)
    {
      /* emax <= 10000000000 */
      mpfr_set_prec (x, 40);
      mpfr_set_prec (y, 40);
      mpfr_set_str (x, "10000000000.5", 10, MPFR_RNDN);
      mpfr_clear_flags ();
      mpfr_exp2 (y, x, MPFR_RNDN);
      if (!(MPFR_IS_INF (y) && MPFR_IS_POS (y) && mpfr_overflow_p ()))
        {
          printf ("exp2(10000000000.5) should overflow.\n");
          exit (1);
        }
    }

  mpfr_set_prec (x, 2);
  mpfr_set_prec (y, 2);
  mpfr_set_str_binary (x, "-1.0E-26");
  mpfr_exp2 (y, x, MPFR_RNDD);
  mpfr_set_str_binary (x, "1.1E-1");
  if (mpfr_cmp (x, y))
    {
      printf ("Error for exp(-2^(-26)) for prec=2\n");
      exit (1);
    }

  test_generic (MPFR_PREC_MIN, 100, 100);

  mpfr_clear (x);
  mpfr_clear (y);

  overflowed_exp2_0 ();

  data_check ("data/exp2", mpfr_exp2, "mpfr_exp2");
  bad_cases (mpfr_exp2, mpfr_log2, "mpfr_exp2",
             0, -256, 255, 4, 128, 800, 50);

  tests_end_mpfr ();
  return 0;
}
