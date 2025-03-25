ENV['PORT'] ||= '8000'

require './app'

request_log_path = ENV.fetch('REQUEST_LOGFILE_PATH', 'request.log')
request_log = File.new(File.expand_path(request_log_path), 'a+')
Sinatra::Application.use Rack::CommonLogger, request_log

run Sinatra::Application
