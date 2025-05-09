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
  crate_names = Dir['lib/known-types*'].sort.map { |s| Pathname::new(s) }.map(&:basename).map(&:to_s)

  task :cargo_add do
    crate_names.each { |crate_name| puts "cargo add #{crate_name}" }
  end

  task :cargo_toml do
    crate_names.each { |crate_name| puts "#{crate_name} = \"0.1\"" }
  end

  task :rust_use do
    crate_names.each { |crate_name| puts "use #{crate_name.gsub('-', '_')};" }
  end

  task :rust_use_custom do
    crate_names.each { |crate_name| puts "#{crate_name} = { version = \"0.1\", default-features = false, features = [\"serde\"] }" }
  end

  task :table do
    crate_names.each do |crate_name|
      crate_dir = Pathname('lib') / crate_name
      crate_toml = TomlRB.load_file(crate_dir / 'Cargo.toml')
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
