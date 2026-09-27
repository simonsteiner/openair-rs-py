"""Allow ``python -m openair``."""

import sys

from openair.cli import main

sys.exit(main())
