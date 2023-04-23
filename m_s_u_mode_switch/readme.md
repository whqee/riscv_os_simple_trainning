## RISC-V M/S/U Mode Switch


Progress:

    M -> S -> M -> S -> U -> S -> U -> M -> End with Panic!


M: 

Set M mode trap handler, that will handle ecall for 'switch context'(decide later) and switch back to M main.

With all prepared, 'mret' to the bellow S mode.

(There are two context at M mode: normal context and M-mode trap context.)


--> S:

Set S mode trap handler, that will handle ecall for 'switch context'(decide later) and switch back to M main.

With all prepared, 'ecall' back to M mode.

(There are two context at S mode: normal context and S-mode trap context.)


--> S:

Say hello and 'sret' switch to the bellow U mode.


--> U:

Say hello and 'ecall' switch to the bellow S mode.


--> S:

Say hello and 'sret' switch to the bellow U mode.


--> U:

Say hello and 'ecall' switch to the bellow M mode.


--> M:

Back to normal context, end with panic.
