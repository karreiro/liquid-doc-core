# frozen_string_literal: true

require_relative "liquid_doc_core/version"

begin
  # Load the extension from a precompiled gem that as cross-compiled
  # for multiple versions
  # (https://github.com/rake-compiler/rake-compiler#but-wait-theres-more)
  RUBY_VERSION =~ /(\d+\.\d+)/
  require "liquid_doc_core/#{Regexp.last_match(1)}/liquid_doc_core"
rescue LoadError
  # Loads locally compiled extension
  require_relative "liquid_doc_core/liquid_doc_core"
end

module LiquidDocCore
  class Error < StandardError; end
end
