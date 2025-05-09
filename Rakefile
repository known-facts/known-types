require 'pathname'
require 'toml-rb' # https://rubygems.org/gems/toml-rb

namespace :version do
  desc "Bump the version number"
  task :bump do
    old_version = File.read('VERSION').strip
    new_version = old_version.gsub(/\.\d+$/, &:succ)
    warn `git grep -l #{old_version} | xargs sd -F #{old_version} #{new_version}`.chomp
  end
end

namespace :readme do
  task :crates do
    Dir['lib/known-types*'].sort.map { |s| Pathname::new(s) }.each do |crate_path|
      crate_name = crate_path.basename.to_s
      crate_toml = TomlRB.load_file(crate_path / 'Cargo.toml')
      crate_summary = crate_toml['package']['description']
      crate_summary = nil unless crate_summary.is_a?(String)
      puts [
        "[#{crate_name}]",
        "[![#{crate_name}](https://img.shields.io/crates/v/#{crate_name})](https://crates.io/crates/#{crate_name})",
        "[![#{crate_name}](https://docs.rs/#{crate_name}/badge.svg)](https://docs.rs/#{crate_name}/)",
        crate_summary,
      ].join(' | ')
    end
  end
end
