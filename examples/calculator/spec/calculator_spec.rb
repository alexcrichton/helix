require "spec_helper"

describe "Calculator" do
  let(:calculator) { Calculator.new }

  it "can multiply floats" do
    expect(calculator.multiply_floats(1.23, -4.56)).to eq(-5.6088)
  end

end
