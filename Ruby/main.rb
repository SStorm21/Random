var1="5"
var2=10
new_var=4<1

puts "hello world"
print "hello world"
5.times{puts"hello world1"}

result = var1.to_i + var2


if new_var==true
    puts"this is ture"
elsif new_var==false
    puts"this is false"
else
    puts"Error"
end

if 4==4 || 4!=5
    puts" this is or"

elsif 5==2 && 5==2
    puts" 5==2 "
else
    puts "idk"
end


puts "this is --> "+result.to_s
puts new_var

for x in 1..20
    puts "this is --> "+x.to_s

end

x=100
while x<1000
    x+=10
    puts "( + ) 10! --> "+x.to_s
    if x==1000
        puts"var x == 1000! quit.."
    end
end

#just like python
names = ["ali","ahmed","soran","price"]
nums=[1,2,3,4,5]
data=["ahmed",15,"price",17]
studens = Array.new(5)
studens[0] = "storm"
studens[5] = "hello"
print names
puts names

for name in names
    for num in nums
        puts "student numbers 1-5 --> "+name + num.to_s
    end
end

for x in data
    puts x.to_s 
end

puts studens



studentss_grade=Hash[
    'ahmed' => 87.1,
    'hasan' => 97.5,
    'ali' => 99.9
]

for x in studentss_grade
    print x
end