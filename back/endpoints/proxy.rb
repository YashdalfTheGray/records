require_relative 'base'

module Endpoints
  class Proxy < Endpoints::Base
    get '/*' do
      if @config[:environment] == 'development'
        redirect @config[:frontend_dev_url]
      else
        send_file File.join(settings.public_folder, 'index.html')
      end
    end
  end
end
