multiply two numbers 

calculate 7 * 11 = 55
+++++
>
+++++++++++
<

>>[-]>[-]<<< zero cells 2 and 3

[ repeat cell 0 times
    - dec cell 0

    add cell 1 to cells 2 and 3
    > 
    [ 
        -
        >+ inc cell 2
        >+ inc cell 3
        <<
    ]

    move cell 2 to cell 1
    >
    [ repeat cell 2 times
        -
        <+ inc cell 1
        >
    ]

    <<
]

the result is in cell 3
>>> . should print 7
