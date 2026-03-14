(use-modules
 ;; Rust
 (gnu packages rust)
 ;; Just
 (gnu packages rust-apps)
 ;; Sqlite
 (gnu packages sqlite)
 ;; Digikam
 (gnu packages kde-graphics)
 ;; git, git-lfs
 (gnu packages version-control)
 ;; Dist (cargo-dist)
 (afistfullofash packages package-management))

(manifest
 (list
  ;; For setting up reference files
  (package->manifest-entry digikam)
  ;; For manageing the repo
  (package->manifest-entry git)
  (package->manifest-entry git-lfs)
  ;; Task Runner
  (package->manifest-entry just)
  ;; Build Dependencies
  (package->manifest-entry sqlite)
  ;; Build Requirements
  (package->manifest-entry rust)
  (package->manifest-entry rust "tools")
  (package->manifest-entry rust "cargo")
  ;; Distrobution
  (package->manifest-entry cargo-dist)))
