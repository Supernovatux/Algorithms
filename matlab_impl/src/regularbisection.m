function [root] = regularbisection(targetfunc,a,b,error)
arguments
    targetfunc,
    a double,
    b double,
    error double = 1e-7
end
%Finds output using the regular bisection method
while abs(b-a)/2 > error
    c = (a+b)/2;
    if targetfunc(c)*targetfunc(b) > 0
        b = c;
    else
        a = c;
    end
end
root = (a+b)/2
end

