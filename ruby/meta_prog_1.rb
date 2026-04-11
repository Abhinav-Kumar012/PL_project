class Order 
  def initialize(order_no)
    @order_no = order_no
  end

  def initated() 
    puts "initated #{@order_no}"
  end 

  def dispatched()
    puts "dispatched #{@order_no}"
  end

  def out_for_delivery()
    puts "out_for_delivery #{@order_no}"
  end

  def delivered()
    puts "delivered #{@order_no}"
  end

  attr_accessor(:order_no)

  def take_action(action)
    return if !self.respond_to?(action)

    self.send(action)
  end
end

def main
  order_123 = Order.new(123)
  order_123.take_action("initated")
  order_123.order_no = 789
  order_123.take_action("dispatched")
  order_123.take_action("out_for_delivery")
  order_123.take_action("delivered")
  order_123.take_action("bruh")
end

if __FILE__ == $0
  _ = main()
end