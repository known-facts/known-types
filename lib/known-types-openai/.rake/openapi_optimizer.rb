# This is free and unencumbered software released into the public domain.

module OpenAPIOptimizer
  ##
  # @param  [Hash] The flattened `openapi.components.schemas` map
  # @return [void]
  def optimize_combines!(schemas)
    raise ArgumentError, schemas.inspect unless schemas.is_a?(Hash)
    schemas.each_value do |schema|
      optimize_combine!(schema, schemas)
    end
  end # optimize_combines!

  ##
  # @param  [Hash] A JSON Schema schema type
  # @param  [Hash] The flattened `openapi.components.schemas` map
  # @return [void]
  def optimize_combine!(schema, schemas)
    raise ArgumentError, schema.inspect unless schema.is_a?(Hash)
    raise ArgumentError, schemas.inspect unless schemas.is_a?(Hash)
    case
      when schema[:oneOf], schema[:anyOf], schema[:allOf]
        variants = schema[:oneOf] || schema[:anyOf] || schema[:allOf] || []
        raise ArgumentError, schema.inspect unless variants.all? { it.is_a?(Hash) }
        variants.reject! { |variant| variant[:'$recursiveRef'] }
        if variants.size == 1
          schema.merge!(variants.first)
          %i(oneOf anyOf allOf).each { schema.delete(it) }
        end
        variants.each do |variant|
          optimize_combine!(variant, schemas)
        end
      else # nothing to do
    end
  end # optimize_combine!
end # OpenAPIOptimizer
