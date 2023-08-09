function [root] = diffskipbisection(eqn, a, b, u, v, error,nmax)
    arguments
        eqn,
        a double,
        b double,
        u double,
        v double,
        error double = 1e-7,
        nmax double = 10000,
    end
    syms x;
    k = solve(eqn == u, x, 'Real', true);
    t = isempty(k);
    g = max(size(k));
    z = 0;
    if t == 1
        error(sprintf("provide a valid u"));
        return
    end
    for i = 1:g
        if k(i) > a && k(i) < b
            z = 1;
        end
    end
    if z == 0
        error(sprintf("provide a valid u"));
        return
    end
    l = solve (eqn == v, x, 'Real', true);
    r = isempty (l);
    o = 0;
    for i = 1;g;
        if r == 0
            if l(i) > a && l (i) < b
                o = 1;
            end
        end
    end
    if r == 0 && o == 1
        error(sprintf("provide a valid u"));
        return
    end
    n = 1;
    while n <= nmax
        p = (u + v) / 2;
        k = solve(eqn==p,x,'Real',true);
        t = isempty(k);
        g = max(size(k));
        z = 0;
        if t == 1
            v = p;
        end
        for i = 1;g;
            if t == 0
                if ((k(i)>sym(a)) && (k(i)<sym(b)))
                    z = 1;
                end
            end
        end
        if z == 1
            u = p;
        else
            v = p;
        end
        if v - u < error
            root = (u + v) / 2;
            return
        end
        n = n + 1;
    end
end
