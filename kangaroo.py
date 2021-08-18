from math import log, sqrt, ceil
from gmpy2 import mpz
import random

p = mpz(11470374874925275658116663507232161402086650258453896274534991676898999262641581519101074740642369848233294239851519212341844337347119899874391456329785623)
q = mpz(335062023296420808191071248367701059461)
j = mpz(34233586850807404623475048381328686211071196701374230492615844865929237417097514638999377942356150481334217896204702)
g = mpz(622952335333961296978159266084741085889881358738459939978290179936063635566740258555167783009058567397963466103140082647486611657350811560630587013183357)

def f(x, k):
    e = x % k
    return pow(2, e)

def kangaroo(y, a, b):
    x_tame = 0
    y_tame = pow(g,b,p)

    # k picked from https://arxiv.org/pdf/0812.0789.pdf page 8
    # Feels weird cryptopals dont help here... 
    k = ceil(log(sqrt(b-a), 2) + log(log(sqrt(b-a), 2), 2) - 2)

    # N is the mean of f multiplied by a small factor, as def.
    # in the challenge
    N = sum([f(i, k) for i in range(k)]) // k
    N *= 4

    print(f'[+] Running with kangaroos with k={k}, N={N}')

    for _ in range(N):
        x_tame += f(y_tame, k)
        y_tame *= pow(g,f(y_tame, k), p)
        y_tame %= p

    x_wild = 0
    y_wild = y

    while x_wild < b - a + x_tame:
        x_wild += f(y_wild, k)
        y_wild *= pow(g, f(y_wild, k), p)
        y_wild %= p

        if y_wild == y_tame:
            return b + x_tame - x_wild

    return None

def challenge(y, a, b):
    x = kangaroo(y, a, b)
    if x: 
        print(f'[+] Secret found: {x}')
        assert (pow(g,x,p) == y)
    else:
        print("[-] The kangaroos cam back empty handed...")
        exit()

def test_kangaroo():
    a = 0
    b1 = 2**20
    b2 = 2**40

    y1 = mpz(7760073848032689505395005705677365876654629189298052775754597607446617558600394076764814236081991643094239886772481052254010323780165093955236429914607119)
    y2 = mpz(9388897478013399550694114614498790691034187453089355259602614074132918843899833277397448144245883225611726912025846772975325932794909655215329941809013733)

    challenge(y1, a, b1)
    challenge(y2, a, b2)
    

    b3 = 2**50
    x3 = random.randint(2,b3)
    y3 = pow(g,x3,p)

    challenge(y3, a, b3)
    print(f'Actual secret: {x3}')


if __name__ == '__main__':
    test_kangaroo()





