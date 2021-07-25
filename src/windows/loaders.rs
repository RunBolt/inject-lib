//Pulled from https://github.com/UserExistsError/InjectDll/blob/master/InjectDll/loaders.h
//Comment from original file: Loader shellcode from https://github.com/UserExistsError/DllLoaderShellcode
pub(super) const LOADER_X86:&[u8] = &[
0x55, 0x8b, 0xec, 0x83, 0xec, 0x24, 0x53, 0x33, 0xdb, 0xc7, 0x45, 0xe8, 0x00, 0x00, 0x00, 0x00,
0x57, 0xc7, 0x45, 0xe4, 0x00, 0x00, 0x00, 0x00, 0x89, 0x5d, 0xf4, 0x89, 0x5d, 0xe0, 0x89, 0x5d,
0xec, 0xe8, 0x4a, 0x04, 0x00, 0x00, 0x8b, 0xf8, 0xba, 0x4d, 0x5a, 0x00, 0x00, 0x0f, 0x1f, 0x00,
0x66, 0x39, 0x17, 0x75, 0x16, 0x8b, 0x4f, 0x3c, 0x8d, 0x41, 0xc0, 0x3d, 0xbf, 0x03, 0x00, 0x00,
0x77, 0x09, 0x81, 0x3c, 0x39, 0x50, 0x45, 0x00, 0x00, 0x74, 0x03, 0x47, 0xeb, 0xe2, 0xb8, 0x4c,
0x01, 0x00, 0x00, 0x89, 0x7d, 0xf8, 0x66, 0x39, 0x44, 0x39, 0x04, 0x74, 0x08, 0x5f, 0x33, 0xc0,
0x5b, 0x8b, 0xe5, 0x5d, 0xc3, 0x64, 0xa1, 0x30, 0x00, 0x00, 0x00, 0x56, 0x8b, 0x40, 0x0c, 0x8b,
0x40, 0x14, 0x89, 0x45, 0xfc, 0x85, 0xc0, 0x0f, 0x84, 0xbd, 0x01, 0x00, 0x00, 0x0f, 0x1f, 0x00,
0x8b, 0x70, 0x28, 0x33, 0xc9, 0x0f, 0xb7, 0x50, 0x24, 0x0f, 0x1f, 0x80, 0x00, 0x00, 0x00, 0x00,
0x0f, 0xb6, 0x3e, 0xc1, 0xc9, 0x0d, 0x80, 0x3e, 0x61, 0x72, 0x03, 0x83, 0xc1, 0xe0, 0x81, 0xc2,
0xff, 0xff, 0x00, 0x00, 0x03, 0xcf, 0x46, 0x66, 0x85, 0xd2, 0x75, 0xe4, 0x81, 0xf9, 0x5b, 0xbc,
0x4a, 0x6a, 0x0f, 0x85, 0xcf, 0x00, 0x00, 0x00, 0x8b, 0x5d, 0xfc, 0xbf, 0x04, 0x00, 0x00, 0x00,
0x8b, 0x73, 0x10, 0x8b, 0x46, 0x3c, 0x8b, 0x44, 0x30, 0x78, 0x03, 0xc6, 0x89, 0x45, 0xdc, 0x8b,
0x58, 0x20, 0x8b, 0x40, 0x24, 0x03, 0xde, 0x03, 0xc6, 0x89, 0x45, 0xf0, 0x0f, 0x1f, 0x40, 0x00,
0x8b, 0x13, 0x03, 0xd6, 0x33, 0xc0, 0x8a, 0x0a, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00,
0xc1, 0xc8, 0x0d, 0x8d, 0x52, 0x01, 0x0f, 0xbe, 0xc9, 0x03, 0xc1, 0x8a, 0x0a, 0x84, 0xc9, 0x75,
0xef, 0x3d, 0x8e, 0x4e, 0x0e, 0xec, 0x74, 0x15, 0x3d, 0xaa, 0xfc, 0x0d, 0x7c, 0x74, 0x0e, 0x3d,
0x54, 0xca, 0xaf, 0x91, 0x74, 0x07, 0x3d, 0xf2, 0x32, 0xf6, 0x0e, 0x75, 0x55, 0x8b, 0x4d, 0xf0,
0x0f, 0xb7, 0x11, 0x8b, 0x4d, 0xdc, 0x8b, 0x49, 0x1c, 0x8d, 0x0c, 0x91, 0x03, 0xce, 0x3d, 0x8e,
0x4e, 0x0e, 0xec, 0x75, 0x09, 0x8b, 0x01, 0x03, 0xc6, 0x89, 0x45, 0xe8, 0xeb, 0x2e, 0x3d, 0xaa,
0xfc, 0x0d, 0x7c, 0x75, 0x09, 0x8b, 0x01, 0x03, 0xc6, 0x89, 0x45, 0xe4, 0xeb, 0x1e, 0x3d, 0x54,
0xca, 0xaf, 0x91, 0x75, 0x09, 0x8b, 0x01, 0x03, 0xc6, 0x89, 0x45, 0xf4, 0xeb, 0x0e, 0x3d, 0xf2,
0x32, 0xf6, 0x0e, 0x75, 0x07, 0x8b, 0x01, 0x03, 0xc6, 0x89, 0x45, 0xec, 0x81, 0xc7, 0xff, 0xff,
0x00, 0x00, 0x83, 0x45, 0xf0, 0x02, 0x83, 0xc3, 0x04, 0x66, 0x85, 0xff, 0x0f, 0x85, 0x5e, 0xff,
0xff, 0xff, 0xe9, 0x81, 0x00, 0x00, 0x00, 0x81, 0xf9, 0x5d, 0x68, 0xfa, 0x3c, 0x75, 0x7c, 0x8b,
0x5d, 0xfc, 0xc7, 0x45, 0xf0, 0x01, 0x00, 0x00, 0x00, 0x8b, 0x5b, 0x10, 0x8b, 0x43, 0x3c, 0x8b,
0x44, 0x18, 0x78, 0x03, 0xc3, 0x89, 0x45, 0xdc, 0x8b, 0x78, 0x20, 0x8b, 0x70, 0x24, 0x03, 0xfb,
0x03, 0xf3, 0x8b, 0x07, 0x33, 0xc9, 0x8d, 0x14, 0x18, 0x8a, 0x02, 0x0f, 0x1f, 0x44, 0x00, 0x00,
0xc1, 0xc9, 0x0d, 0x8d, 0x52, 0x01, 0x0f, 0xbe, 0xc0, 0x03, 0xc8, 0x8a, 0x02, 0x84, 0xc0, 0x75,
0xef, 0x81, 0xf9, 0xb8, 0x0a, 0x4c, 0x53, 0x75, 0x21, 0x8b, 0x45, 0xdc, 0x0f, 0xb7, 0x0e, 0x8b,
0x40, 0x1c, 0x8d, 0x04, 0x88, 0x8b, 0x04, 0x18, 0x03, 0xc3, 0x89, 0x45, 0xe0, 0x8b, 0x45, 0xf0,
0x05, 0xff, 0xff, 0x00, 0x00, 0x89, 0x45, 0xf0, 0xeb, 0x03, 0x8b, 0x45, 0xf0, 0x83, 0xc7, 0x04,
0x83, 0xc6, 0x02, 0x66, 0x85, 0xc0, 0x75, 0xaa, 0x8b, 0x5d, 0xf4, 0x83, 0x7d, 0xe8, 0x00, 0x74,
0x16, 0x83, 0x7d, 0xe4, 0x00, 0x74, 0x10, 0x85, 0xdb, 0x74, 0x0c, 0x83, 0x7d, 0xec, 0x00, 0x74,
0x06, 0x83, 0x7d, 0xe0, 0x00, 0x75, 0x10, 0x8b, 0x45, 0xfc, 0x8b, 0x00, 0x89, 0x45, 0xfc, 0x85,
0xc0, 0x0f, 0x85, 0x49, 0xfe, 0xff, 0xff, 0x8b, 0x7d, 0xf8, 0x8b, 0x5f, 0x3c, 0x6a, 0x40, 0x03,
0xdf, 0x68, 0x00, 0x30, 0x00, 0x00, 0x89, 0x5d, 0xf0, 0xff, 0x73, 0x50, 0x6a, 0x00, 0xff, 0x55,
0xf4, 0xff, 0x73, 0x50, 0x8b, 0xf0, 0x56, 0x89, 0x75, 0xfc, 0xff, 0x55, 0xec, 0x8b, 0x53, 0x54,
0x8b, 0xcf, 0x85, 0xd2, 0x74, 0x18, 0x8b, 0xfe, 0x2b, 0x7d, 0xf8, 0x0f, 0x1f, 0x44, 0x00, 0x00,
0x8a, 0x01, 0x8d, 0x49, 0x01, 0x88, 0x44, 0x0f, 0xff, 0x83, 0xea, 0x01, 0x75, 0xf2, 0x0f, 0xb7,
0x7b, 0x14, 0x03, 0xfb, 0x0f, 0xb7, 0x5b, 0x06, 0x85, 0xdb, 0x74, 0x34, 0x8b, 0x45, 0xf8, 0x83,
0xc7, 0x2c, 0x8b, 0x4f, 0xf8, 0x4b, 0x8b, 0x17, 0x03, 0xce, 0x8b, 0x77, 0xfc, 0x03, 0xd0, 0x85,
0xf6, 0x74, 0x13, 0x8a, 0x02, 0x8d, 0x49, 0x01, 0x88, 0x41, 0xff, 0x8d, 0x52, 0x01, 0x83, 0xee,
0x01, 0x75, 0xf0, 0x8b, 0x45, 0xf8, 0x8b, 0x75, 0xfc, 0x83, 0xc7, 0x28, 0x85, 0xdb, 0x75, 0xd2,
0x8b, 0x5d, 0xf0, 0x8b, 0xbb, 0x80, 0x00, 0x00, 0x00, 0x03, 0xfe, 0x89, 0x7d, 0xec, 0x83, 0x3f,
0x00, 0x74, 0x7d, 0x8b, 0x47, 0x0c, 0x03, 0xc6, 0x50, 0xff, 0x55, 0xe8, 0x8b, 0xd8, 0x85, 0xdb,
0x74, 0x60, 0x8b, 0x37, 0x8b, 0x55, 0xfc, 0x03, 0xf2, 0x8b, 0x7f, 0x10, 0x03, 0xfa, 0x83, 0x3f,
0x00, 0x74, 0x49, 0x85, 0xf6, 0x74, 0x22, 0x8b, 0x0e, 0x85, 0xc9, 0x79, 0x1c, 0x8b, 0x43, 0x3c,
0x0f, 0xb7, 0xc9, 0x8b, 0x44, 0x18, 0x78, 0x2b, 0x4c, 0x18, 0x10, 0x8b, 0x44, 0x18, 0x1c, 0x8d,
0x04, 0x88, 0x8b, 0x04, 0x18, 0x03, 0xc3, 0xeb, 0x0f, 0x8b, 0x07, 0x83, 0xc2, 0x02, 0x03, 0xc2,
0x50, 0x53, 0xff, 0x55, 0xe4, 0x8b, 0x55, 0xfc, 0x89, 0x07, 0x83, 0xc7, 0x04, 0x85, 0xf6, 0x8d,
0x46, 0x04, 0x0f, 0x44, 0xc6, 0x83, 0x3f, 0x00, 0x8b, 0xf0, 0x75, 0xb7, 0x8b, 0x75, 0xfc, 0x8b,
0x7d, 0xec, 0x83, 0xc7, 0x14, 0x89, 0x7d, 0xec, 0x83, 0x3f, 0x00, 0x75, 0x86, 0x8b, 0x5d, 0xf0,
0x8b, 0x93, 0xa4, 0x00, 0x00, 0x00, 0x8b, 0xfe, 0x2b, 0x7b, 0x34, 0x89, 0x55, 0xec, 0x85, 0xd2,
0x0f, 0x84, 0xb0, 0x00, 0x00, 0x00, 0x8b, 0x83, 0xa0, 0x00, 0x00, 0x00, 0x03, 0xc6, 0x89, 0x45,
0xe4, 0x8b, 0x48, 0x04, 0x85, 0xc9, 0x0f, 0x84, 0x97, 0x00, 0x00, 0x00, 0x8b, 0x30, 0x8d, 0x59,
0xf8, 0x03, 0x75, 0xfc, 0x83, 0xc0, 0x08, 0xd1, 0xeb, 0x89, 0x45, 0xe8, 0x74, 0x70, 0x66, 0x90,
0x0f, 0xb7, 0x10, 0x4b, 0x66, 0x8b, 0xc2, 0x8b, 0xca, 0x66, 0xc1, 0xe8, 0x0c, 0x66, 0x83, 0xf8,
0x0a, 0x75, 0x0b, 0x81, 0xe2, 0xff, 0x0f, 0x00, 0x00, 0x01, 0x3c, 0x32, 0xeb, 0x38, 0x66, 0x83,
0xf8, 0x03, 0x75, 0x0b, 0x81, 0xe1, 0xff, 0x0f, 0x00, 0x00, 0x01, 0x3c, 0x31, 0xeb, 0x27, 0x66,
0x83, 0xf8, 0x01, 0x75, 0x11, 0x81, 0xe1, 0xff, 0x0f, 0x00, 0x00, 0x8b, 0xc7, 0xc1, 0xe8, 0x10,
0x66, 0x01, 0x04, 0x31, 0xeb, 0x10, 0x66, 0x83, 0xf8, 0x02, 0x75, 0x0a, 0x81, 0xe1, 0xff, 0x0f,
0x00, 0x00, 0x66, 0x01, 0x3c, 0x31, 0x8b, 0x45, 0xe8, 0x83, 0xc0, 0x02, 0x89, 0x45, 0xe8, 0x85,
0xdb, 0x75, 0x9d, 0x8b, 0x45, 0xe4, 0x8b, 0x55, 0xec, 0x8b, 0x48, 0x04, 0xeb, 0x03, 0x8b, 0x45,
0xe4, 0x2b, 0xd1, 0x03, 0xc1, 0x89, 0x55, 0xec, 0x89, 0x45, 0xe4, 0x85, 0xd2, 0x0f, 0x85, 0x5e,
0xff, 0xff, 0xff, 0x8b, 0x5d, 0xf0, 0x8b, 0x73, 0x28, 0x8b, 0x5d, 0xfc, 0x03, 0xf3, 0x6a, 0x00,
0x6a, 0x00, 0x6a, 0xff, 0xff, 0x55, 0xe0, 0x6a, 0x00, 0x6a, 0x01, 0x53, 0xff, 0xd6, 0x8b, 0xc6,
0x5e, 0x5f, 0x5b, 0x8b, 0xe5, 0x5d, 0xc3, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc,
0xc1, 0xc9, 0x0d, 0x8b, 0xc1, 0xc3, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc,
0x8a, 0x11, 0x33, 0xc0, 0xc1, 0xc8, 0x0d, 0x8d, 0x49, 0x01, 0x0f, 0xbe, 0xd2, 0x03, 0xc2, 0x8a,
0x11, 0x84, 0xd2, 0x75, 0xef, 0xc3, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc,
0x55, 0x8b, 0xec, 0x8b, 0x45, 0x04, 0x5d, 0xc3
];

pub(super) const LOADER_X64:&[u8] = &[
0x40, 0x55, 0x56, 0x41, 0x54, 0x41, 0x55, 0x41, 0x56, 0x48, 0x83, 0xec, 0x30, 0x45, 0x33, 0xf6,
0x45, 0x33, 0xc9, 0x45, 0x33, 0xe4, 0x4c, 0x89, 0x74, 0x24, 0x68, 0x33, 0xf6, 0x4c, 0x89, 0x4c,
0x24, 0x60, 0x33, 0xed, 0xe8, 0x07, 0x05, 0x00, 0x00, 0x4c, 0x8b, 0xe8, 0xb8, 0x4d, 0x5a, 0x00,
0x00, 0x66, 0x41, 0x39, 0x45, 0x00, 0x75, 0x1b, 0x49, 0x63, 0x55, 0x3c, 0x48, 0x8d, 0x4a, 0xc0,
0x48, 0x81, 0xf9, 0xbf, 0x03, 0x00, 0x00, 0x77, 0x0a, 0x42, 0x81, 0x3c, 0x2a, 0x50, 0x45, 0x00,
0x00, 0x74, 0x05, 0x49, 0xff, 0xc5, 0xeb, 0xd9, 0xb8, 0x64, 0x86, 0x00, 0x00, 0x4c, 0x89, 0x6c,
0x24, 0x70, 0x66, 0x42, 0x39, 0x44, 0x2a, 0x04, 0x74, 0x0f, 0x33, 0xc0, 0x48, 0x83, 0xc4, 0x30,
0x41, 0x5e, 0x41, 0x5d, 0x41, 0x5c, 0x5e, 0x5d, 0xc3, 0x65, 0x48, 0x8b, 0x04, 0x25, 0x60, 0x00,
0x00, 0x00, 0x41, 0xbb, 0x01, 0x00, 0x00, 0x00, 0x48, 0x89, 0x5c, 0x24, 0x78, 0x48, 0x89, 0x7c,
0x24, 0x28, 0x4c, 0x89, 0x7c, 0x24, 0x20, 0x48, 0x8b, 0x48, 0x18, 0x4c, 0x8b, 0x79, 0x20, 0x4d,
0x85, 0xff, 0x0f, 0x84, 0xf0, 0x01, 0x00, 0x00, 0x41, 0x8d, 0x5b, 0x03, 0x41, 0xba, 0xff, 0xff,
0x00, 0x00, 0x49, 0x8b, 0x57, 0x50, 0x33, 0xc0, 0x45, 0x0f, 0xb7, 0x47, 0x48, 0x0f, 0x1f, 0x00,
0x0f, 0xb6, 0x0a, 0xc1, 0xc8, 0x0d, 0x80, 0xf9, 0x61, 0x72, 0x04, 0x48, 0x83, 0xc0, 0xe0, 0x48,
0x03, 0xc1, 0x48, 0xff, 0xc2, 0x66, 0x45, 0x03, 0xc2, 0x75, 0xe5, 0x3d, 0x5b, 0xbc, 0x4a, 0x6a,
0x0f, 0x85, 0xe5, 0x00, 0x00, 0x00, 0x4d, 0x8b, 0x4f, 0x20, 0x41, 0xbd, 0xff, 0xff, 0x00, 0x00,
0x49, 0x63, 0x41, 0x3c, 0x42, 0x8b, 0xbc, 0x08, 0x88, 0x00, 0x00, 0x00, 0x46, 0x8b, 0x54, 0x0f,
0x20, 0x46, 0x8b, 0x5c, 0x0f, 0x24, 0x4d, 0x03, 0xd1, 0x4d, 0x03, 0xd9, 0x0f, 0x1f, 0x40, 0x00,
0x41, 0x8b, 0x02, 0x45, 0x33, 0xc0, 0x4a, 0x8d, 0x0c, 0x08, 0x42, 0x0f, 0xb6, 0x04, 0x08, 0x90,
0x41, 0xc1, 0xc8, 0x0d, 0x48, 0x8d, 0x49, 0x01, 0x0f, 0xbe, 0xc0, 0x44, 0x03, 0xc0, 0x0f, 0xb6,
0x01, 0x84, 0xc0, 0x75, 0xeb, 0x41, 0x81, 0xf8, 0x8e, 0x4e, 0x0e, 0xec, 0x74, 0x1b, 0x41, 0x81,
0xf8, 0xaa, 0xfc, 0x0d, 0x7c, 0x74, 0x12, 0x41, 0x81, 0xf8, 0x54, 0xca, 0xaf, 0x91, 0x74, 0x09,
0x41, 0x81, 0xf8, 0xf2, 0x32, 0xf6, 0x0e, 0x75, 0x54, 0x42, 0x8b, 0x4c, 0x0f, 0x1c, 0x41, 0x0f,
0xb7, 0x13, 0x49, 0x03, 0xc9, 0x41, 0x81, 0xf8, 0x8e, 0x4e, 0x0e, 0xec, 0x75, 0x09, 0x44, 0x8b,
0x34, 0x91, 0x4d, 0x03, 0xf1, 0xeb, 0x32, 0x41, 0x81, 0xf8, 0xaa, 0xfc, 0x0d, 0x7c, 0x75, 0x09,
0x44, 0x8b, 0x24, 0x91, 0x4d, 0x03, 0xe1, 0xeb, 0x20, 0x41, 0x81, 0xf8, 0x54, 0xca, 0xaf, 0x91,
0x75, 0x08, 0x8b, 0x34, 0x91, 0x49, 0x03, 0xf1, 0xeb, 0x0f, 0x41, 0x81, 0xf8, 0xf2, 0x32, 0xf6,
0x0e, 0x75, 0x06, 0x8b, 0x2c, 0x91, 0x49, 0x03, 0xe9, 0x66, 0x41, 0x03, 0xdd, 0x49, 0x83, 0xc2,
0x04, 0x49, 0x83, 0xc3, 0x02, 0x66, 0x85, 0xdb, 0x0f, 0x85, 0x52, 0xff, 0xff, 0xff, 0x4c, 0x89,
0x74, 0x24, 0x68, 0x45, 0x8b, 0xd5, 0xe9, 0x94, 0x00, 0x00, 0x00, 0x3d, 0x5d, 0x68, 0xfa, 0x3c,
0x0f, 0x85, 0x98, 0x00, 0x00, 0x00, 0x4d, 0x8b, 0x57, 0x20, 0x41, 0xbe, 0xff, 0xff, 0x00, 0x00,
0x49, 0x63, 0x42, 0x3c, 0x42, 0x8b, 0x9c, 0x10, 0x88, 0x00, 0x00, 0x00, 0x46, 0x8b, 0x44, 0x13,
0x20, 0x46, 0x8b, 0x4c, 0x13, 0x24, 0x4d, 0x03, 0xc2, 0x4d, 0x03, 0xca, 0x0f, 0x1f, 0x40, 0x00,
0x41, 0x8b, 0x00, 0x33, 0xc9, 0x4a, 0x8d, 0x14, 0x10, 0x42, 0x0f, 0xb6, 0x04, 0x10, 0x66, 0x90,
0xc1, 0xc9, 0x0d, 0x48, 0x8d, 0x52, 0x01, 0x0f, 0xbe, 0xc0, 0x03, 0xc8, 0x0f, 0xb6, 0x02, 0x84,
0xc0, 0x75, 0xed, 0x81, 0xf9, 0xb8, 0x0a, 0x4c, 0x53, 0x75, 0x1b, 0x42, 0x8b, 0x4c, 0x13, 0x1c,
0x41, 0x0f, 0xb7, 0x11, 0x49, 0x03, 0xca, 0x8b, 0x3c, 0x91, 0x49, 0x03, 0xfa, 0x48, 0x89, 0x7c,
0x24, 0x60, 0x66, 0x45, 0x03, 0xde, 0x49, 0x83, 0xc0, 0x04, 0x49, 0x83, 0xc1, 0x02, 0x66, 0x45,
0x85, 0xdb, 0x75, 0xac, 0x4c, 0x8b, 0x74, 0x24, 0x68, 0x41, 0xba, 0xff, 0xff, 0x00, 0x00, 0x4c,
0x8b, 0x4c, 0x24, 0x60, 0x41, 0xbb, 0x01, 0x00, 0x00, 0x00, 0x41, 0x8d, 0x5b, 0x03, 0x4d, 0x85,
0xf6, 0x74, 0x14, 0x4d, 0x85, 0xe4, 0x74, 0x0f, 0x48, 0x85, 0xf6, 0x74, 0x0a, 0x48, 0x85, 0xed,
0x74, 0x05, 0x4d, 0x85, 0xc9, 0x75, 0x0c, 0x4d, 0x8b, 0x3f, 0x4d, 0x85, 0xff, 0x0f, 0x85, 0x1f,
0xfe, 0xff, 0xff, 0x4c, 0x8b, 0x6c, 0x24, 0x70, 0x4d, 0x63, 0x7d, 0x3c, 0x33, 0xc9, 0x4d, 0x03,
0xfd, 0x41, 0xb8, 0x00, 0x30, 0x00, 0x00, 0x44, 0x8d, 0x49, 0x40, 0x41, 0x8b, 0x57, 0x50, 0xff,
0xd6, 0x41, 0x8b, 0x57, 0x50, 0x48, 0x8b, 0xc8, 0x4c, 0x8b, 0xf0, 0xff, 0xd5, 0x41, 0x8b, 0x57,
0x54, 0x49, 0x8b, 0xcd, 0x48, 0x85, 0xd2, 0x74, 0x17, 0x4d, 0x8b, 0xc6, 0x4d, 0x2b, 0xc5, 0x90,
0x0f, 0xb6, 0x01, 0x41, 0x88, 0x04, 0x08, 0x48, 0xff, 0xc1, 0x48, 0x83, 0xea, 0x01, 0x75, 0xf0,
0x45, 0x0f, 0xb7, 0x4f, 0x14, 0x45, 0x0f, 0xb7, 0x57, 0x06, 0x49, 0x83, 0xc1, 0x2c, 0x4d, 0x85,
0xd2, 0x74, 0x47, 0x4d, 0x03, 0xcf, 0x66, 0x66, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00,
0x41, 0x8b, 0x49, 0xf8, 0x49, 0xff, 0xca, 0x41, 0x8b, 0x11, 0x49, 0x03, 0xce, 0x45, 0x8b, 0x41,
0xfc, 0x49, 0x03, 0xd5, 0x4d, 0x85, 0xc0, 0x74, 0x18, 0x0f, 0x1f, 0x80, 0x00, 0x00, 0x00, 0x00,
0x0f, 0xb6, 0x02, 0x48, 0xff, 0xc2, 0x88, 0x01, 0x48, 0xff, 0xc1, 0x49, 0x83, 0xe8, 0x01, 0x75,
0xef, 0x49, 0x83, 0xc1, 0x28, 0x4d, 0x85, 0xd2, 0x75, 0xc6, 0x41, 0x8b, 0xaf, 0x90, 0x00, 0x00,
0x00, 0x49, 0x03, 0xee, 0x83, 0x7d, 0x00, 0x00, 0x0f, 0x84, 0x92, 0x00, 0x00, 0x00, 0x4c, 0x8b,
0x6c, 0x24, 0x68, 0x8b, 0x4d, 0x0c, 0x49, 0x03, 0xce, 0x41, 0xff, 0xd5, 0x48, 0x8b, 0xf0, 0x48,
0x85, 0xc0, 0x74, 0x6e, 0x8b, 0x7d, 0x10, 0x8b, 0x5d, 0x00, 0x49, 0x03, 0xfe, 0x49, 0x03, 0xde,
0x48, 0x83, 0x3f, 0x00, 0x74, 0x5c, 0x48, 0x85, 0xdb, 0x74, 0x2c, 0x48, 0x8b, 0x13, 0x48, 0x85,
0xd2, 0x79, 0x24, 0x48, 0x63, 0x46, 0x3c, 0x0f, 0xb7, 0xd2, 0x8b, 0x8c, 0x30, 0x88, 0x00, 0x00,
0x00, 0x8b, 0x44, 0x31, 0x10, 0x8b, 0x4c, 0x31, 0x1c, 0x48, 0x2b, 0xd0, 0x48, 0x03, 0xce, 0x8b,
0x04, 0x91, 0x48, 0x03, 0xc6, 0xeb, 0x10, 0x48, 0x8b, 0x17, 0x48, 0x8b, 0xce, 0x48, 0x83, 0xc2,
0x02, 0x49, 0x03, 0xd6, 0x41, 0xff, 0xd4, 0x48, 0x89, 0x07, 0x48, 0x83, 0xc7, 0x08, 0x48, 0x85,
0xdb, 0x48, 0x8d, 0x43, 0x08, 0x48, 0x0f, 0x44, 0xc3, 0x48, 0x83, 0x3f, 0x00, 0x48, 0x8b, 0xd8,
0x75, 0xa4, 0x48, 0x83, 0xc5, 0x14, 0x83, 0x7d, 0x00, 0x00, 0x0f, 0x85, 0x73, 0xff, 0xff, 0xff,
0x41, 0x8b, 0x87, 0xb4, 0x00, 0x00, 0x00, 0x4d, 0x8b, 0xce, 0x4d, 0x2b, 0x4f, 0x30, 0x85, 0xc0,
0x0f, 0x84, 0xb9, 0x00, 0x00, 0x00, 0x41, 0x8b, 0x9f, 0xb0, 0x00, 0x00, 0x00, 0x8b, 0xf8, 0x49,
0x03, 0xde, 0x85, 0xc0, 0x0f, 0x84, 0xa5, 0x00, 0x00, 0x00, 0x66, 0x0f, 0x1f, 0x44, 0x00, 0x00,
0x8b, 0x43, 0x04, 0x85, 0xc0, 0x0f, 0x84, 0x94, 0x00, 0x00, 0x00, 0x44, 0x8b, 0x03, 0x4c, 0x8d,
0x50, 0xf8, 0x4d, 0x03, 0xc6, 0x4c, 0x8d, 0x5b, 0x08, 0x49, 0xd1, 0xea, 0x74, 0x75, 0x66, 0x90,
0x41, 0x0f, 0xb7, 0x13, 0x49, 0xff, 0xca, 0x0f, 0xb7, 0xca, 0x0f, 0xb7, 0xc2, 0x66, 0xc1, 0xe9,
0x0c, 0x66, 0x83, 0xf9, 0x0a, 0x75, 0x14, 0x81, 0xe2, 0xff, 0x0f, 0x00, 0x00, 0x4a, 0x8b, 0x04,
0x02, 0x49, 0x8d, 0x0c, 0x01, 0x4a, 0x89, 0x0c, 0x02, 0xeb, 0x3c, 0x66, 0x83, 0xf9, 0x03, 0x75,
0x0b, 0x25, 0xff, 0x0f, 0x00, 0x00, 0x46, 0x01, 0x0c, 0x00, 0xeb, 0x2b, 0x66, 0x83, 0xf9, 0x01,
0x75, 0x15, 0x25, 0xff, 0x0f, 0x00, 0x00, 0x4a, 0x8d, 0x0c, 0x00, 0x49, 0x8b, 0xc1, 0x48, 0xc1,
0xe8, 0x10, 0x66, 0x01, 0x01, 0xeb, 0x10, 0x66, 0x83, 0xf9, 0x02, 0x75, 0x0a, 0x25, 0xff, 0x0f,
0x00, 0x00, 0x66, 0x46, 0x01, 0x0c, 0x00, 0x49, 0x83, 0xc3, 0x02, 0x4d, 0x85, 0xd2, 0x75, 0x90,
0x8b, 0x43, 0x04, 0x48, 0x03, 0xd8, 0x48, 0x2b, 0xf8, 0x0f, 0x85, 0x61, 0xff, 0xff, 0xff, 0x41,
0x8b, 0x5f, 0x28, 0x33, 0xd2, 0x45, 0x33, 0xc0, 0x49, 0x03, 0xde, 0x48, 0x8d, 0x4a, 0xff, 0xff,
0x54, 0x24, 0x60, 0x45, 0x33, 0xc0, 0x49, 0x8b, 0xce, 0x41, 0x8d, 0x50, 0x01, 0xff, 0xd3, 0x4c,
0x8b, 0x7c, 0x24, 0x20, 0x48, 0x8b, 0xc3, 0x48, 0x8b, 0x5c, 0x24, 0x78, 0x48, 0x8b, 0x7c, 0x24,
0x28, 0x48, 0x83, 0xc4, 0x30, 0x41, 0x5e, 0x41, 0x5d, 0x41, 0x5c, 0x5e, 0x5d, 0xc3, 0xcc, 0xcc,
0xc1, 0xc9, 0x0d, 0x8b, 0xc1, 0xc3, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc,
0x0f, 0xb6, 0x11, 0x33, 0xc0, 0x66, 0x66, 0x66, 0x0f, 0x1f, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00,
0xc1, 0xc8, 0x0d, 0x48, 0x8d, 0x49, 0x01, 0x0f, 0xbe, 0xd2, 0x03, 0xc2, 0x0f, 0xb6, 0x11, 0x84,
0xd2, 0x75, 0xed, 0xc3, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc, 0xcc,
0x48, 0x8b, 0x04, 0x24, 0xc3
];