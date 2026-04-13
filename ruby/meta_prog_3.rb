class Square
  def initialize()
    
  end
    def method_missing(method_name, *args, &block)
        if method_name.to_s.start_with?("area_")
            length = method_name.to_s.gsub("area_", "").to_i
            return length*length
        else
            super
        end
    end
end


def main
  p = Square.new()
  puts p.area_50000
end

if __FILE__ == $0
  _ = main
end