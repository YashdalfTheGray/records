require 'securerandom'

module DAO
  module AlbumsApi
    def get(id)
      raise NotImplementedError
    end

    def find_by_spotify_album_id(spotify_id)
      raise NotImplementedError
    end

    def list
      raise NotImplementedError
    end

    def create!(attrs)
      raise NotImplementedError
    end

    def update!(id, attrs)
      raise NotImplementedError
    end

    def delete!(id)
      raise NotImplementedError
    end
  end

  class AlbumsFile
    include AlbumsApi

    def initialize(store:)
      @store = store
      @table = :albums
    end

    def get(id)
      @store.get(@table, id)
    end

    def find_by_spotify_album_id(spotify_id)
      @store.find(@table) { |rec| rec[:spotify_album_id] == spotify_id }
    end

    def list
      @store.list(@table)
    end

    def create!(attrs)
      id = attrs[:id] || SecureRandom.uuid
      record = attrs.merge(id: id)
      @store.put(@table, id, record)
      record
    end

    def update!(id, attrs)
      existing = get(id)
      raise KeyError, "album not found: #{id}" unless existing

      updated = existing.merge(attrs)
      @store.put(@table, id, updated)
      updated
    end

    def delete!(id)
      @store.delete(@table, id)
    end
  end
end
