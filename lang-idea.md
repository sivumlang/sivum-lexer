### Language Idea

#Sivum

##### Sivum In Verbum Ultra Morte

According to google translate and translate.com. "Live in the Word beyond death"

#### Basic Ideas

1.  No operators
2.  Hanging methods
3.  Rust enums
4.  No inheritance
5.  method definitions require mut or not
6.  Errors as value. Copy rust result? Any reason not to?
7.  Only pass by reference? Does that even make sense? Don't know don't care.
8.  Spaces do not matter
9.  Do I want zero parameter functions to not need parenthesis? Yes
10. No assignment.






##### Examples

###### 1 No Operators

``` cs
string a = "Hello";
string b = " ";
string c = "World!";
string greet = a + b + c;
```

``` sivum
a.Is("Hello");
b.Is(" ");
c.Is("World!");
greet.Is(a.cat(b).cat(c));
greet.Is(a.cat(b.cat(c)));
```

###### 2 Hanging Methods

``` rust
SomeType a = SomeType::new();
int b = SomeType::get_int();
```

``` sivum
a.Is(SomeType());
b.Is(a.GetInt());
```

###### 3 Rust Enums

``` rust
enum Cup {
    Empty
    Full(Drink)
    Partial(Drink,Amount)
}
```

###### 4 No Inheritance

``` cs
class Animal {
    void Speak(){Console.WriteLine("???")}
}
class Dog : Animal {
    override void Speak(){Console.WriteLine("Bark")}
}
```

###### 5 Mutable Method Type

Every function that wants to make a mutation must be declared as mutable.
The following is a SetVolume function that has the ability to mutate and a ReadVolume function that can not mutate the object.

``` sivum
interface Speaker {
    mut void SetVolume(float at);
    float ReadVolume();
}
```

Lastly a parameter that is mutated must be declared mutable. Much like rust.
These different functions can be considered different spaces. The mutable section can access other mutable functions otherwise the mutable functions are hidden. This is just like that program color video... The bad part.

###### 6 Errors as Value

May as well do it like rust? Maybe I can figure something different out later.
Or
``` sivum
a.IsErr(SomeErrorableFunction);
```

###### 7 Pass by reference only

In order to pass by value, you will have to do a "clone" type operation. 

###### 8 Spaces do not matter

``` sivum
myvar.Is("this example");io.println(myvar);
```

###### 9 Zero Param No Paren 

``` sivum
x.Is(-55);
y.Is(x.Abs);
```

###### 10 No assignment

``` sivum
a.Is("Hello");
b.IsMut(5);
c.IsErr('c');
d.IsMutErr("idk");
```

###### 11 Function Parameters???

``` sivum
mut fun Log(s.IsParam(String)) {
    logger.Appened(s);
}
mut fun SetVolume(box.IsMutParam(SpeakerBox)) {
    box.volume.Set(55);
}
fun Double(a.IsParam(int)).IsRet(Int) {
    return a.Multiply(2);
}
```

#### Example Problem AoC 22 d1p1

``` cs
int current = 0;
int max = 0;

foreach (var line in File.ReadLines("data.txt")) {
    if (string.IsNullOrEmpty(line)) {
        max = Math.Max(max, current);
        current = 0;
    }
    else
    {
        current += int.Parse(line);
    }
}
max = Math.Max(max, current);
return max;
```

``` sivum
current.IsMut(0);
max.IsMut(0);

file.Is(io.ReadFile("data.txt"));
lines.Is(file.MutClone.SplitNewLine());

for (line.IsParam(String):lines) {
    if (line.IsWhitespace) {
        max.Set.Max(current);
    } else {
        value.IsErr(int.Parse(line));
        if (value.ok) {
            current.Set.Add(value);
        } else {
            continue;
        }
    }
}

max.Set.Max(current);
return max;
```



