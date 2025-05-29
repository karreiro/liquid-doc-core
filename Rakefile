# frozen_string_literal: true

require "bundler/gem_tasks"
require "minitest/test_task"
require "rubocop/rake_task"
require "rb_sys/extensiontask"

Minitest::TestTask.create
RuboCop::RakeTask.new

GEMSPEC = Gem::Specification.load("liquid_doc_core.gemspec")

RbSys::ExtensionTask.new("liquid_doc_core", GEMSPEC) do |ext|
  ext.lib_dir = "lib/liquid_doc_core"
end

task build: :compile
task default: %i[compile test rubocop]
