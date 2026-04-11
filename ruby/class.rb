class Greeter
  def initialize(name = "Charlie")
    @name = name
  end
  def say_hi()
    puts "Hi, #{@name}"
  end
  def say_bye
    puts "Bye, #{@name}"
  end
  attr_accessor :name
end

def main
  p = Greeter.new("pat")
  p.say_hi
  p.name = "gt"
  p.say_bye()
end

_ = main()