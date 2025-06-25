#!/usr/bin/env python3
"""
Example usage of the OpenAir Python bindings.
"""

from openair import parse_string, parse_file

# Example OpenAir data
openair_data = """
AC D
AN EXAMPLE CTR
AL GND
AH 5000 ft
DP 46:57:13 N 008:27:52 E
DP 46:57:46 N 008:30:41 E
DP 46:57:55 N 008:28:40 E
DP 46:57:13 N 008:27:52 E
"""

def main():
    try:
        # Parse the example data
        airspaces = parse_string(openair_data)
        
        print(f"Parsed {len(airspaces)} airspace(s):")
        for i, airspace in enumerate(airspaces):
            print(f"  {i+1}. {airspace['name']} (Class {airspace['class']})")
            print(f"     Lower bound: {airspace['lowerBound']}")
            print(f"     Upper bound: {airspace['upperBound']}")
            print(f"     Geometry: {airspace['geom']['type']}")
            if airspace['geom']['type'] == 'Polygon':
                segment_count = len(airspace['geom']['segments'])
                print(f"     Segments: {segment_count}")
            elif airspace['geom']['type'] == 'Circle':
                radius = airspace['geom']['radius']
                print(f"     Radius: {radius} NM")
            print()
    
    except Exception as e:
        print(f"Error parsing OpenAir data: {e}")


if __name__ == "__main__":
    main()
