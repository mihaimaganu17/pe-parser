# Backlog
1. Add default checks in the parser
    1. Check section_alignment vs file_alignment
    2. Check entry_point is in the text section
    3. Check size_of_image
    4. Check size_of_headers
    5. Check checksum
    6. Check reserved values
2. Parsing additional enums:
    1. Subsystem
    2. DLL Characteristics
2. Add `parser` method to each header
3. Define a `Parsable` trait
4. Add tests for each file
