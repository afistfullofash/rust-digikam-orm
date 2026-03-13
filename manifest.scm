(use-modules (gnu packages rust)
	     (gnu packages sqlite)
	     (afistfullofash packages package-management))

(manifest
 (list
  (package->manifest-entry sqlite)
  (package->manifest-entry rust)
  (package->manifest-entry rust "tools")
  (package->manifest-entry rust "cargo")
  (package->manifest-entry cargo-dist)))
