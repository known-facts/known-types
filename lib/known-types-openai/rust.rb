# This is free and unencumbered software released into the public domain.

module Rust; end

module Rust::Type; end

module Rust::Types
  Val = ::Struct.new('Val', :t) do |type|
    include Rust::Type
    type.define_method(:to_s) { t.to_s }
  end

  Bool = Val.new(:bool).freeze
  I64 = Val.new(:i64).freeze
  F64 = Val.new(:f64).freeze
  String = Val.new(:String).freeze

  Ref = ::Struct.new('Ref', :t) do |type|
    include Rust::Type
    type.define_method(:to_s) { "&#{t}" }
  end

  Unit = ::Struct.new('Unit', :m) do |type|
    include Rust::Type
    type.define_method(:to_s) { "(/*#{m}*/)" }
  end

  Vec = ::Struct.new('Vec', :t) do |type|
    include Rust::Type
    type.define_method(:to_s) { "Vec<#{t}>" }
  end

  Option = ::Struct.new('Option', :t) do |type|
    include Rust::Type
    type.define_method(:to_s) { "Option<#{t}>" }
  end

  Result = ::Struct.new('Result', :t, :e) do |type|
    include Rust::Type
    type.define_method(:to_s) { "Result<#{t}, #{e}>" }
  end
end # Rust::Types

module Rust
  class Definition
    include Rust::Type

    attr_reader :name
    attr_reader :derives
    attr_accessor :comment

    ##
    # @param [String, #to_s] name
    # @param [Array<Symbol, #to_sym>, #to_a] derives
    # @param [String, #to_s] comment
    def initialize(name, derives: nil, comment: nil)
      @name = name.to_s
      @derives = (derives || []).to_a.dup.uniq.map!(&:to_sym).sort
      @comment = comment ? comment.to_s.strip : nil
    end

    ##
    # @return [Boolean]
    def comment?() self.comment && !self.comment.empty? end

    ##
    # @return [String]
    def to_s() @name end

    ##
    # @param [IO] out
    # @return [void]
    def write(out)
      out.puts wrap_text(self.comment, 80-4).map { |s| s.prepend("/// ") }.join("\n") if self.comment?
      out.puts "#[derive(#{@derives.sort.join(", ")})]" unless @derives.empty?
      out.puts "#[cfg_attr(feature = \"serde\", derive(serde::Serialize, serde::Deserialize))]"
    end
  end # Definition

  class Newtype < Definition
    ##
    # @param [String, #to_s] name
    # @param [Type] type
    # @param [Array<Symbol, #to_sym>, #to_a] derives
    # @param [String, #to_s] comment
    # @param [Proc] block
    def initialize(name, type, derives: %i(Clone Debug), comment: nil, &block)
      super(name, derives:, comment:)
      raise ArgumentError, "#{type.inspect}" unless type.is_a?(Rust::Type)
      @type = type
      block.call(self) if block_given?
    end

    ##
    # @param [IO] out
    # @return [void]
    def write(out)
      super(out)
      out.puts "pub struct #{@name}(pub #{@type});"
    end
  end # Struct

  class Struct < Definition
    attr_reader :fields

    ##
    # @param [String, #to_s] name
    # @param [Array<Field>, #to_a] fields
    # @param [Array<Symbol, #to_sym>, #to_a] derives
    # @param [String, #to_s] comment
    # @param [Proc] block
    def initialize(name, fields: nil, derives: nil, comment: nil, &block)
      super(name, derives:, comment:)
      @fields = (fields || []).to_a.dup
      block.call(self) if block_given?
    end

    ##
    # @param [IO] out
    # @return [void]
    def write(out)
      super(out)
      if self.fields.empty?
        out.puts "pub struct #{@name};"
      else
        out.puts "pub struct #{@name} {"
        @fields.each_with_index do |field, i|
          out.puts if i > 0
          out.puts wrap_text(field.comment, 80-8).map { |s| s.prepend("    /// ") }.join("\n") if field.comment?
          out.puts "    #[cfg_attr(feature = \"serde\", serde(rename = \"#{k.id}\"))]" if false
          field.write(out)
        end
        out.puts "}"
      end
    end
  end # Struct

  class Enum < Definition
    attr_reader :variants

    ##
    # @param [String, #to_s] name
    # @param [Array<Variant>, #to_a] variants
    # @param [Array<Symbol, #to_sym>, #to_a] derives
    # @param [String, #to_s] comment
    # @param [Proc] block
    def initialize(name, variants: nil, derives: nil, comment: nil, &block)
      super(name, derives:, comment:)
      @variants = (variants || []).to_a.dup
      block.call(self) if block_given?
    end

    ##
    # @param [IO] out
    # @return [void]
    def write(out)
      super(out)
      if self.variants.empty?
        out.puts "pub struct #{@name};"
      else
        out.puts "pub enum #{@name} {"
        @variants.each_with_index do |variant, i|
          out.puts if i > 0
          out.puts wrap_text(variant.comment, 80-8).map { |s| s.prepend("    /// ") }.join("\n") if variant.comment?
          variant.write(out)
        end
        out.puts "}"
      end
    end
  end # Enum

  class Field
    attr_reader :name, :type, :summary
    attr_accessor :comment, :rename

    def initialize(name, type, &block)
      @name = name.to_s.gsub('[]', '').gsub('.', '_')
      @type = type
      raise ArgumentError, "#{type.inspect}" unless type.is_a?(Rust::Type)
      block.call(self) if block_given?
    end

    ##
    # @return [Boolean]
    def comment?() self.comment && !self.comment.empty? end

    ##
    # @param [IO] out
    # @return [void]
    def write(out)
      out.puts "    #[cfg_attr(feature = \"serde\", serde(rename = \"#{self.rename}\"))]" if self.rename
      out.puts "    pub r##{@name}: #{@type},"
    end
  end # Field

  class Variant
    attr_reader :name, :type, :summary
    attr_accessor :comment

    def initialize(name, type = nil, &block)
      @name = name.to_sym
      @type = type
      raise ArgumentError, "#{type.inspect}" unless type.nil? || type.is_a?(Rust::Type)
      block.call(self) if block_given?
    end

    ##
    # @return [Boolean]
    def comment?() self.comment && !self.comment.empty? end

    ##
    # @param [IO] out
    # @return [void]
    def write(out)
      if !@type
        out.puts "    #{@name},"
      else
        out.puts "    #{@name}(#{@type}),"
      end
    end
  end # Variant
end # Rust
