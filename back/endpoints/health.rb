require_relative 'base'

module Endpoints
  class Health < Endpoints::Base
    get '/api/health' do
      content_type :json
      status 200
      { status: 'okay' }.to_json
    end
  end
end
