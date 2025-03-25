require 'sinatra/base'
require 'sinatra/cross_origin'
require 'logger'
require 'json'

require_relative 'endpoints/health'

class App < Sinatra::Base
  error_log_file = File.open(File.expand_path(ENV.fetch('ERROR_LOGFILE_PATH', 'error.log')))

  configure do
    enable :cross_origin
    enable :logging
    set :public_folder, File.expand_path('./public', __dir__)

    set :app_config, {
      environment: ENV.fetch('RACK_ENV', 'development'),
      frontend_dev_url: ENV['FRONTEND_DEV_URL'] || 'http://localhost:5173',
      request_log_path: ENV.fetch('REQUEST_LOGFILE_PATH', 'request.log'),
      erro_log_path: ENV.fetch('ERROR_LOGFILE_PATH', 'error.log'),
    }
  end

  before do
    response.headers['Access-Control-Allow-Origin'] = '*'
    response.headers['Access-Control-Allow-Methods'] = 'GET, POST, PUT, DELETE, OPTIONS'
    response.headers['Access-Control-Allow-Headers'] = 'Content-Type'
  end

  before do
    env['rack.errors'] = error_log_file
  end

  options '*' do
    200
  end
end
