# This is free and unencumbered software released into the public domain.

module OpenAPIHoister
  ##
  # @param  [Hash] The flattened `openapi.components.schemas` map
  # @return [void]
  def hoist_nullables!(schemas)
    raise ArgumentError, schemas.inspect unless schemas.is_a?(Hash)
    schemas.each_value do |schema|
      hoist_nullable!(schema, schemas)
    end
  end # hoist_nullables!

  ##
  # @param  [Hash] A JSON Schema schema type
  # @param  [Hash] The flattened `openapi.components.schemas` map
  # @return [void]
  def hoist_nullable!(schema, schemas)
    raise ArgumentError, schema.inspect unless schema.is_a?(Hash)
    raise ArgumentError, schemas.inspect unless schemas.is_a?(Hash)
    case
      when schema[:'$ref']
        ref = schema[:'$ref'].split('/').last.to_sym
        #p [ref, schema[:nullable], schemas[ref][:nullable]]
        schema[:nullable] = true if schemas[ref][:nullable]
      when schema[:oneOf], schema[:anyOf]
        variants = schema[:oneOf] || schema[:anyOf] || []
        variants.each do |variant|
          hoist_nullable!(variant, schemas)
        end
      when schema[:allOf]
        variants = schema[:allOf] || []
        if variants.any? { it[:'$ref'] } && variants.any? { it[:nullable] }
          variants.each { schema.merge!(it) }
          schema.delete(:allOf)
        else
          variants.each { hoist_nullable!(it, schemas) }
        end
      when schema[:properties] || schema[:type] == 'object'
        properties = schema[:properties] || {}
        properties.each_value do |property|
          hoist_nullable!(property, schemas)
        end
      when schema[:items] || schema[:type] == 'array'
        hoist_nullable!(schema[:items], schemas) if schema[:items]
      else # nothing to do
    end
  end # hoist_nullable!
end # OpenAPIHoister
