require_relative 'base'

module Endpoints
  class Proxy < Endpoints::Base
    get '/*' do
      if @config[:environment] == 'development'
        logger.info "Proxy redirecting to frontend dev server: #{@config[:frontend_dev_url]}"
        redirect @config[:frontend_dev_url]
      else
        logger.info "Serving index.html from #{settings.public_folder}"
        send_file File.join(settings.public_folder, 'index.html')
      end
    end
  end
end
