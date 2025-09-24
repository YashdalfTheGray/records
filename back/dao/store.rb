require 'json'

module DAO
  class Store
    def get(table, id)
      raise NotImplementedError
    end

    def list(table)
      raise NotImplementedError
    end

    def put(table, id, record)
      raise NotImplementedError
    end

    def delete(table, id)
      raise NotImplementedError
    end

    def find(table, &block)
      raise NotImplementedError
    end
  end

  class JsonFileStore < Store
    def initialize(path:)
      @path = path
      @mutex = Mutex.new
      ensure_file
    end

    def get(table, id)
      db = read_db
      (db[table] || {})[id]
    end

    def list(table)
      db = read_db
      h = db[table] || {}
      h.values
    end

    def put(table, id, record)
      @mutex.synchronize do
        db = read_db
        db[table] ||= {}
        db[table][id] = record
        write_db(db)
      end
      true
    end

    def delete(table, id)
      @mutex.synchronize do
        db = read_db
        return false unless db[table]&.key?(id)
        db[table].delete(id)
        write_db(db)
      end
      true
    end

    def find(table, &block)
      db = read_db
      h = db[table] || {}
      h.each_value { |rec| return rec if block.call(rec) }
      nil
    end

    private

    def ensure_file
      return if File.exist?(@path)
      write_db(default_state)
    end

    def read_db
      File.open(@path, 'r') { |f| JSON.parse(f.read, symbolize_names: true) }
    rescue Errno::ENOENT
      default_state
    end

    def write_db(db)
      tmp = "#{@path}.tmp"
      File.open(tmp, 'w') { |f| f.write(JSON.generate(db)) }
      File.rename(tmp, @path)
    end

    def default_state
      { version: 1, admins: [], readers: [], albums: {} }
    end
  end
end
