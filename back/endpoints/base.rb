require 'sinatra/base'
require 'json'

module Endpoints
  class Base < Sinatra::Base
    def initialize(app = nil, config: {})
      super(app)

      @config = config
    end
  end
end
