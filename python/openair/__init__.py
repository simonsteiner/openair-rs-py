"""
Python bindings for OpenAir airspace file parser.

This module provides a Python interface to the Rust-based OpenAir parser.
"""

import json
from typing import List, Dict, Any, Union
from pathlib import Path

try:
    from .openair import parse_openair_string, parse_openair_file
except ImportError:
    # Fallback for development - the Rust module needs to be built
    def parse_openair_string(data: str) -> str:
        raise ImportError("OpenAir Rust module not built. Run 'maturin develop' to build it.")
    
    def parse_openair_file(filepath: str) -> str:
        raise ImportError("OpenAir Rust module not built. Run 'maturin develop' to build it.")


class OpenAirParser:
    """
    A Python wrapper for the OpenAir airspace file parser.
    
    This parser can handle airspace files in OpenAir format, which is used
    by various flight instruments and aviation software.
    """
    
    @staticmethod
    def parse_string(data: str) -> List[Dict[str, Any]]:
        """
        Parse OpenAir airspace data from a string.
        
        Args:
            data: OpenAir format string to parse
            
        Returns:
            List of airspace dictionaries
            
        Raises:
            ValueError: If the data cannot be parsed
        """
        json_result = parse_openair_string(data)
        return json.loads(json_result)
    
    @staticmethod
    def parse_file(filepath: Union[str, Path]) -> List[Dict[str, Any]]:
        """
        Parse OpenAir airspace data from a file.
        
        Args:
            filepath: Path to the OpenAir file to parse
            
        Returns:
            List of airspace dictionaries
            
        Raises:
            ValueError: If the file cannot be parsed
            IOError: If the file cannot be read
        """
        filepath_str = str(filepath)
        json_result = parse_openair_file(filepath_str)
        return json.loads(json_result)


# Convenience functions for direct usage
def parse_string(data: str) -> List[Dict[str, Any]]:
    """Parse OpenAir data from string. Convenience function."""
    return OpenAirParser.parse_string(data)


def parse_file(filepath: Union[str, Path]) -> List[Dict[str, Any]]:
    """Parse OpenAir data from file. Convenience function."""
    return OpenAirParser.parse_file(filepath)


__all__ = ['OpenAirParser', 'parse_string', 'parse_file']
