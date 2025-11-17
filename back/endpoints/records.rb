require 'rspotify'

require_relative 'base'
require_relative '../serializers/album'

module Endpoints
  class Records < Endpoints::Base
    def initialize(app = nil, config: {})
      super(app, config: config)
    end

    helpers do
      def ensure_spotify!
        return if defined?(@spotify_ready) && @spotify_ready
        id = @config[:spotify_client_id]
        secret = @config[:spotify_client_secret]
        halt 500, { error: "Spotify credentials missing" }.to_json if id.to_s.empty? || secret.to_s.empty?
        begin
          RSpotify.authenticate(id, secret)
          @spotify_ready = true
        rescue RestClient::ExceptionWithResponse => e
          if (io = env['rack.errors'])
            io.puts("Spotify auth failed: #{e.http_code} #{e.response}")
          end
          halt 502, { error: "Spotify auth failed", status: e.http_code }.to_json
        end
      end
    end

    before { content_type :json }

    post '/resolve' do
      ensure_spotify!
      album = params['album'].to_s.strip
      artist = params['artist'].to_s.strip
      halt 400, { error: "Missing 'album' or 'artist'\n" }.to_json if album.empty? || artist.empty?

      query = %(album:"#{album}" artist:"#{artist}")
      base = RSpotify::Album.search(query, limit: 1).first
      halt 404, { error: "No album found for album='#{album}', artist='#{artist}'" }.to_json unless base

      full = RSpotify::Album.find(base.id) rescue nil
      halt 502, { error: "Failed to fetch album details" }.to_json unless full

      ::Serializers::Album.dump(full).to_json
    end
  end
end
