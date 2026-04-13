class OrderStatus 
  def initialize(order_no)
    @order_no = order_no
    @status = "initiated"
  end
  STATUSES = ["initiated","dispatched","out_for_delivery","delivered"]
  STATUSES.each do |status|
    define_method("is_#{status}?") do
      @status == status
    end
  end
  STATUSES.each do |status|
    define_method("to_#{status}") do
      @status = status
    end
  end
  attr_accessor(:order_no,:status)
end

def main
  order_123 = OrderStatus.new(123)
  pp(order_123.is_initiated?())
  order_123.to_dispatched()
  pp(order_123.is_dispatched?())
  pp(order_123.is_delivered?())
end

if __FILE__ == $0
  _ = main()
end