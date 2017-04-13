require 'rbconfig'
require 'fileutils'

libdir = RbConfig::CONFIG['libdir'] # C:/Ruby23/lib
libruby = RbConfig::CONFIG['LIBRUBY'] # libmsvcrt-ruby230.dll.a
puts libdir
puts libruby
FileUtils.copy(File.join(libdir, libruby), 'msvcrt-ruby230.lib')

