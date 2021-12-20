trait ResourceFile {
	fn open(filename: String) -> Result<String, String>;
}

truct ResourceCache {
	size_in_mb: uint32;
	file: impl ResourceFile;
}

struct File {
	filename: String,
}

// impl ResourceFile for File {
// 	fn open(filename: String) -> Result<String, String> {
// 		Ok(String::from(""))
// 	}
// }

fn oi() {
	let mut resource_cache = ResourceCache {
		size_in_mb: 0,
		file: File {
			filename: String::from(""),
		},
	};

	resource_cache.file.open(String::from(""));
}