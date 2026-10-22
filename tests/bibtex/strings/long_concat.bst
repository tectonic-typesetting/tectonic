ENTRY { title } {} {}

INTEGERS { i }

FUNCTION {misc}
{ #0 'i :=
  title
    { i #7000 < }
    { title swap$ *
      i #1 + 'i :=
    }
  while$
  text.length$ int.to.str$ write$
  newline$
}

READ

ITERATE {call.type$}
