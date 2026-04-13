class Square
  def initialize()
    
  end
  def method_missing(method_name, *args, &block)
    method_n = method_name.to_s
    if method_n.start_with?("area_")
      length = method_n.gsub("area_", "").to_i
      return length*length
    elsif method_n.start_with?("perimeter_")
      length = method_n.gsub("perimeter_", "").to_i
      return 4*length
    else
        super
    end
  end
end


def main
  p = Square.new()
  puts p.area_50000
  puts p.perimeter_50000
end

if __FILE__ == $0
  _ = main
end