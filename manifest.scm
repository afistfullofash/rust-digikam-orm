(use-modules (gnu packages rust)
	     (afistfullofash packages package-management))

(manifest
 (list
  (package->manifest-entry rust)
  (package->manifest-entry rust "tools")
  (package->manifest-entry rust "cargo")
  (package->manifest-entry cargo-dist)))
