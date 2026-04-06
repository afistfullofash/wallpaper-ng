(use-modules
 (gnu packages rust)
 (gnu packages image-viewers)
 (gnu packages version-control)
 (gnu packages commencement))

(manifest
 (list
  ;; For managing the repository
  (package->manifest-entry git)
  ;; Build Requirements
  (package->manifest-entry gcc-toolchain)
  (package->manifest-entry rust)
  (package->manifest-entry rust "tools")
  (package->manifest-entry rust "cargo")))
