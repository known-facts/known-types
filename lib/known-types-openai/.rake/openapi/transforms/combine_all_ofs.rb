# This is free and unencumbered software released into the public domain.

module OpenAPI; end
module OpenAPI::Transforms; end

##
# Combines `allOf: [...]` schemas into a single schema.
module OpenAPI::Transforms::CombineAllOfs
  ##
  # @param  [Hash] The flattened `openapi.components.schemas` map
  # @return [void]
  def self.transform_schemas!(schemas)
    raise ArgumentError, schemas.inspect unless schemas.is_a?(Hash)
    schemas.each_value do |schema|
      transform_schema!(schema, schemas)
    end
  end # transform_schemas!

  protected

  ##
  # @param  [Hash] A JSON Schema schema type
  # @param  [Hash] The flattened `openapi.components.schemas` map
  # @return [void]
  def self.transform_schema!(schema, schemas)
    raise ArgumentError, schema.inspect unless schema.is_a?(Hash)
    raise ArgumentError, schemas.inspect unless schemas.is_a?(Hash)
    case
      when schema[:allOf]
        # TODO
      else # nothing to do
    end
  end # transform_schema!
end # OpenAPI::Transforms::CombineAllOfs
