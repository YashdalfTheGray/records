module Serializers
  class Album
    def self.dump(album)
      images = album.images || []
      cover_image_url =
        images.max_by { |im| (im['width'] || im[:width]).to_i }&.dig('url') ||
        images.max_by { |im| (im[:width] || 0).to_i }&.dig(:url)

      artists = (album.artists || []).map { |a| { spotify_artist_id: a.id, name: a.name } }
      primary_artist = artists.first&.dig(:name)
      release_year = album.release_date.to_s[0, 4]
      album_url = (album.external_urls || {})['spotify'] || (album.external_urls || {})[:spotify]

      raw_tracks = album.tracks rescue []
      tracks = (raw_tracks || []).map do |t|
        url = (t.external_urls || {})['spotify'] || (t.external_urls || {})[:spotify]
        { name: t.name.to_s, url: (url.to_s.empty? ? nil : url), disc: (t.respond_to?(:disc_number) ? t.disc_number : nil) }
      end

      {
        spotify_album_id: album.id,
        type: 'album',
        title: album.name,
        primary_artist: primary_artist,
        artists: artists,
        release_year: release_year,
        cover_image_url: cover_image_url,
        spotify_url: album_url,
        track_count: tracks.length,
        tracks: tracks,
      }
    end

    def self.load(item)
      {
        spotify_album_id: fetch(item, :spotify_album_id),
        type: 'album',
        title: fetch(item, :title),
        primary_artist: fetch(item, :primary_artist),
        artists: (fetch(item, :artists) || []).map { |a| { spotify_artist_id: fetch(a, :spotify_artist_id), name: fetch(a, :name) } },
        release_year: fetch(item, :release_year),
        cover_image_url: fetch(item, :cover_image_url),
        spotify_url: fetch(item, :spotify_url),
        track_count: (fetch(item, :track_count) || (fetch(item, :tracks) || []).length).to_i,
        tracks: (fetch(item, :tracks) || []).map { |t| { name: fetch(t, :name).to_s, url: (fetch(t, :url) || nil), disc: fetch(t, :disc) } },
      }
    end

    def self.fetch(h, key)
      h.is_a?(Hash) ? (h[key] || h[key.to_s]) : nil
    end
    private_class_method :fetch
  end
end
