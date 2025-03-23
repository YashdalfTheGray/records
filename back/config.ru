ENV['PORT'] ||= '8000'

require './app'
run Sinatra::Application
