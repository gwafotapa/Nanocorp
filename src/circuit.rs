use std::{collections::HashMap, fmt};

use crate::{
    error::Error,
    gate::Gate,
    wire::{Wire, WireId, WireInput},
};

#[derive(Debug, PartialEq)]
pub struct Circuit {
    pub wires: HashMap<WireId, Wire>,
}

impl Circuit {
    pub fn new() -> Self {
        Self {
            wires: HashMap::new(),
        }
    }

    pub fn remove(&mut self, id: &str) {
        self.wires.remove(id);
    }

    pub fn add(&mut self, wire: Wire) -> Result<(), Error> {
        if self.wires.contains_key(&wire.id) {
            Err(Error::IdAlreadyExists(wire.id))
        } else {
            self.wires.insert(wire.id.clone(), wire);
            Ok(())
        }
    }

    pub fn add_wire_with_input(
        &mut self,
        id: impl Into<String>,
        input: WireInput,
    ) -> Result<(), Error> {
        let wire = Wire::new(id, input)?;
        self.add(wire)?;
        Ok(())
    }

    pub fn add_wire_with_value(&mut self, id: impl Into<String>, value: u16) -> Result<(), Error> {
        let wire = Wire::with_value(id, value)?;
        self.add(wire)?;
        Ok(())
    }

    pub fn add_wire_from_wire(
        &mut self,
        id: impl Into<String>,
        input_id: impl Into<String>,
    ) -> Result<(), Error> {
        let wire = Wire::from_wire(id, input_id)?;
        self.add(wire)?;
        Ok(())
    }

    pub fn add_wire_from_gate(&mut self, id: impl Into<String>, gate: Gate) -> Result<(), Error> {
        let wire = Wire::from_gate(id, gate)?;
        self.add(wire)?;
        Ok(())
    }

    pub fn add_gate_and(
        &mut self,
        output: impl Into<String>,
        input1: impl Into<String>,
        input2: impl Into<String>,
    ) -> Result<(), Error> {
        let gate = Gate::and(input1, input2)?;
        let wire = Wire::from_gate(output, gate)?;
        self.add(wire)?;
        Ok(())
    }

    pub fn add_gate_or(
        &mut self,
        output: impl Into<String>,
        input1: impl Into<String>,
        input2: impl Into<String>,
    ) -> Result<(), Error> {
        let gate = Gate::or(input1, input2)?;
        let wire = Wire::from_gate(output, gate)?;
        self.add(wire)?;
        Ok(())
    }

    pub fn add_gate_sll(
        &mut self,
        output: impl Into<String>,
        input: impl Into<String>,
        shift: u8,
    ) -> Result<(), Error> {
        let gate = Gate::sll(input, shift)?;
        let wire = Wire::from_gate(output, gate)?;
        self.add(wire)?;
        Ok(())
    }

    pub fn add_gate_slr(
        &mut self,
        output: impl Into<String>,
        input: impl Into<String>,
        shift: u8,
    ) -> Result<(), Error> {
        let gate = Gate::slr(input, shift)?;
        let wire = Wire::from_gate(output, gate)?;
        self.add(wire)?;
        Ok(())
    }

    pub fn add_gate_not(
        &mut self,
        output: impl Into<String>,
        input: impl Into<String>,
    ) -> Result<(), Error> {
        let gate = Gate::not(input)?;
        let wire = Wire::from_gate(output, gate)?;
        self.add(wire)?;
        Ok(())
    }

    // TODO: rework
    // TODO: check for loops
    // TODO: add result return type
    pub fn compute_signals(&mut self) -> bool {
        let mut ids: Vec<String> = self.wires.keys().map(|id| id.into()).collect();
        while let Some(id) = ids.last() {
            if let Some(wire) = self.wires.get(id) {
                match &wire.input {
                    WireInput::Value(value) => {
                        self.wires.get_mut(id).unwrap().signal = Some(*value); // TODO: add fn
                        ids.pop();
                    }
                    WireInput::Wire(input_id) => {
                        if let Some(signal) = self.get_signal(input_id) {
                            self.wires.get_mut(id).unwrap().signal = Some(signal);
                            ids.pop();
                        } else {
                            ids.push(input_id.to_string());
                        }
                    }
                    WireInput::Gate(gate) => match gate {
                        Gate::And { input1, input2 } => {
                            if let Some(signal1) = self.get_signal(input1) {
                                if let Some(signal2) = self.get_signal(input2) {
                                    self.wires.get_mut(id).unwrap().signal =
                                        Some(signal1 & signal2);
                                    ids.pop();
                                } else {
                                    ids.push(input2.to_string());
                                }
                            } else {
                                ids.push(input1.to_string());
                                if self.get_signal(input2).is_none() {
                                    ids.push(input2.to_string());
                                }
                            }
                        }
                        Gate::Or { input1, input2 } => {
                            if let Some(signal1) = self.get_signal(input1) {
                                if let Some(signal2) = self.get_signal(input2) {
                                    self.wires.get_mut(id).unwrap().signal =
                                        Some(signal1 | signal2);
                                    ids.pop();
                                } else {
                                    ids.push(input2.to_string());
                                }
                            } else {
                                ids.push(input1.to_string());
                                if self.get_signal(input2).is_none() {
                                    ids.push(input2.to_string());
                                }
                            }
                        }
                        Gate::SLL { input, shift } => {
                            if let Some(signal) = self.get_signal(input) {
                                self.wires.get_mut(id).unwrap().signal = Some(signal << shift);
                                ids.pop();
                            } else {
                                ids.push(input.to_string());
                            }
                        }
                        Gate::SLR { input, shift } => {
                            if let Some(signal) = self.get_signal(input) {
                                self.wires.get_mut(id).unwrap().signal = Some(signal >> shift);
                                ids.pop();
                            } else {
                                ids.push(input.to_string());
                            }
                        }
                        Gate::Not { input } => {
                            if let Some(signal) = self.get_signal(input) {
                                self.wires.get_mut(id).unwrap().signal = Some(!signal);
                                ids.pop();
                            } else {
                                ids.push(input.to_string());
                            }
                        }
                    },
                }
            } else {
                // Unkwown wire id
                break;
            }
        }

        true
    }

    pub fn get_signal(&self, id: &str) -> Option<u16> {
        self.wires.get(id).and_then(|w| w.signal)
    }
}

impl fmt::Display for Circuit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for wire in self.wires.values() {
            write!(f, "{}\n", wire)?
        }
        Ok(())
    }
}

impl From<&str> for Circuit {
    fn from(s: &str) -> Self {
        let mut circuit = Circuit::new();
        for wire in s.split('\n') {
            circuit.add(wire.into()).unwrap();
        }
        circuit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_circuit() {
        let w1 = Wire::with_value("a", 1).unwrap();
        let w2 = Wire::from_wire("b", "a").unwrap();
        // let w2_ = Wire::new_wire_from_component("b", "a");

        let mut circuit = Circuit::new();
        assert!(circuit.add(w1).is_ok());
        assert!(circuit.add(w2).is_ok());
        // assert!(!circuit.add(w2_));

        assert_eq!(circuit.get_signal("z"), None);
        assert_eq!(circuit.get_signal("a"), None);
        assert_eq!(circuit.get_signal("b"), None);
        circuit.compute_signals();
        assert_eq!(circuit.get_signal("a"), Some(1));
        assert_eq!(circuit.get_signal("b"), Some(1));

        let g = Gate::not("b").unwrap();
        let c = Wire::from_gate("c", g).unwrap();
        let _ = circuit.add(c);
        assert_eq!(circuit.get_signal("c"), None);
        circuit.compute_signals();
        assert_eq!(circuit.get_signal("c"), Some(0xfffe));
        println!("{}", circuit);
    }

    #[test]
    fn instructions_example() {
        let x = Wire::with_value("x", 123).unwrap();
        let y = Wire::with_value("y", 456).unwrap();
        let gd = Gate::and("x", "y").unwrap();
        let ge = Gate::or("x", "y").unwrap();
        let gf = Gate::sll("x", 2).unwrap();
        let gg = Gate::slr("y", 2).unwrap();
        let gh = Gate::not("x").unwrap();
        let gi = Gate::not("y").unwrap();
        let d = Wire::from_gate("d", gd).unwrap();
        let e = Wire::from_gate("e", ge).unwrap();
        let f = Wire::from_gate("f", gf).unwrap();
        let g = Wire::from_gate("g", gg).unwrap();
        let h = Wire::from_gate("h", gh).unwrap();
        let i = Wire::from_gate("i", gi).unwrap();

        let mut circuit = Circuit::new();
        circuit.add(x).unwrap();
        circuit.add(y).unwrap();
        circuit.add(d).unwrap();
        circuit.add(e).unwrap();
        circuit.add(f).unwrap();
        circuit.add(g).unwrap();
        circuit.add(h).unwrap();
        circuit.add(i).unwrap();

        circuit.compute_signals();

        assert_eq!(circuit.get_signal("d"), Some(72));
        assert_eq!(circuit.get_signal("e"), Some(507));
        assert_eq!(circuit.get_signal("f"), Some(492));
        assert_eq!(circuit.get_signal("g"), Some(114));
        assert_eq!(circuit.get_signal("h"), Some(65412));
        assert_eq!(circuit.get_signal("i"), Some(65079));
        assert_eq!(circuit.get_signal("x"), Some(123));
        assert_eq!(circuit.get_signal("y"), Some(456));
    }

    #[test]
    fn instructions_example_2() {
        let mut circuit = Circuit::new();
        circuit.add_wire_with_value("x", 123).unwrap();
        circuit.add_wire_with_value("y", 456).unwrap();
        circuit.add_gate_and("d", "x", "y").unwrap();
        circuit.add_gate_or("e", "x", "y").unwrap();
        circuit.add_gate_sll("f", "x", 2).unwrap();
        circuit.add_gate_slr("g", "y", 2).unwrap();
        circuit.add_gate_not("h", "x").unwrap();
        circuit.add_gate_not("i", "y").unwrap();
        circuit.compute_signals();

        println!("{}", circuit);

        assert_eq!(circuit.get_signal("d"), Some(72));
        assert_eq!(circuit.get_signal("e"), Some(507));
        assert_eq!(circuit.get_signal("f"), Some(492));
        assert_eq!(circuit.get_signal("g"), Some(114));
        assert_eq!(circuit.get_signal("h"), Some(65412));
        assert_eq!(circuit.get_signal("i"), Some(65079));
        assert_eq!(circuit.get_signal("x"), Some(123));
        assert_eq!(circuit.get_signal("y"), Some(456));
    }

    #[test]
    fn from() {
        let s = "x AND y -> d\n\
		 NOT x -> h\n\
		 NOT y -> i\n\
		 x OR y -> e\n\
		 y RSHIFT 2 -> g\n\
		 x LSHIFT 2 -> f\n\
		 123 -> x\n\
		 456 -> y";
        let c1 = Circuit::from(s);

        let mut c2 = Circuit::new();
        c2.add_wire_with_value("x", 123).unwrap();
        c2.add_wire_with_value("y", 456).unwrap();
        c2.add_gate_and("d", "x", "y").unwrap();
        c2.add_gate_or("e", "x", "y").unwrap();
        c2.add_gate_sll("f", "x", 2).unwrap();
        c2.add_gate_slr("g", "y", 2).unwrap();
        c2.add_gate_not("h", "x").unwrap();
        c2.add_gate_not("i", "y").unwrap();

        assert_eq!(c1, c2);
    }

    #[test]
    fn input_example() {
        let s = "lf AND lq -> ls
iu RSHIFT 1 -> jn
bo OR bu -> bv
gj RSHIFT 1 -> hc
et RSHIFT 2 -> eu
bv AND bx -> by
is OR it -> iu
b OR n -> o
gf OR ge -> gg
NOT kt -> ku
ea AND eb -> ed
kl OR kr -> ks
hi AND hk -> hl
au AND av -> ax
lf RSHIFT 2 -> lg
dd RSHIFT 3 -> df
eu AND fa -> fc
df AND dg -> di
ip LSHIFT 15 -> it
NOT el -> em
et OR fe -> ff
fj LSHIFT 15 -> fn
t OR s -> u
ly OR lz -> ma
ko AND kq -> kr
NOT fx -> fy
et RSHIFT 1 -> fm
eu OR fa -> fb
dd RSHIFT 2 -> de
NOT go -> gp
kb AND kd -> ke
hg OR hh -> hi
jm LSHIFT 1 -> kg
NOT cn -> co
jp RSHIFT 2 -> jq
jp RSHIFT 5 -> js
1 AND io -> ip
eo LSHIFT 15 -> es
1 AND jj -> jk
g AND i -> j
ci RSHIFT 3 -> ck
gn AND gp -> gq
fs AND fu -> fv
lj AND ll -> lm
jk LSHIFT 15 -> jo
iu RSHIFT 3 -> iw
NOT ii -> ij
1 AND cc -> cd
bn RSHIFT 3 -> bp
NOT gw -> gx
NOT ft -> fu
jn OR jo -> jp
iv OR jb -> jc
hv OR hu -> hw
19138 -> b
gj RSHIFT 5 -> gm
hq AND hs -> ht
dy RSHIFT 1 -> er
ao OR an -> ap
ld OR le -> lf
bk LSHIFT 1 -> ce
bz AND cb -> cc
bi LSHIFT 15 -> bm
il AND in -> io
af AND ah -> ai
as RSHIFT 1 -> bl
lf RSHIFT 3 -> lh
er OR es -> et
NOT ax -> ay
ci RSHIFT 1 -> db
et AND fe -> fg
lg OR lm -> ln
k AND m -> n
hz RSHIFT 2 -> ia
kh LSHIFT 1 -> lb
NOT ey -> ez
NOT di -> dj
dz OR ef -> eg
lx -> a
NOT iz -> ja
gz LSHIFT 15 -> hd
ce OR cd -> cf
fq AND fr -> ft
at AND az -> bb
ha OR gz -> hb
fp AND fv -> fx
NOT gb -> gc
ia AND ig -> ii
gl OR gm -> gn
0 -> c
NOT ca -> cb
bn RSHIFT 1 -> cg
c LSHIFT 1 -> t
iw OR ix -> iy
kg OR kf -> kh
dy OR ej -> ek
km AND kn -> kp
NOT fc -> fd
hz RSHIFT 3 -> ib
NOT dq -> dr
NOT fg -> fh
dy RSHIFT 2 -> dz
kk RSHIFT 2 -> kl
1 AND fi -> fj
NOT hr -> hs
jp RSHIFT 1 -> ki
bl OR bm -> bn
1 AND gy -> gz
gr AND gt -> gu
db OR dc -> dd
de OR dk -> dl
as RSHIFT 5 -> av
lf RSHIFT 5 -> li
hm AND ho -> hp
cg OR ch -> ci
gj AND gu -> gw
ge LSHIFT 15 -> gi
e OR f -> g
fp OR fv -> fw
fb AND fd -> fe
cd LSHIFT 15 -> ch
b RSHIFT 1 -> v
at OR az -> ba
bn RSHIFT 2 -> bo
lh AND li -> lk
dl AND dn -> do
eg AND ei -> ej
ex AND ez -> fa
NOT kp -> kq
NOT lk -> ll
x AND ai -> ak
jp OR ka -> kb
NOT jd -> je
iy AND ja -> jb
jp RSHIFT 3 -> jr
fo OR fz -> ga
df OR dg -> dh
gj RSHIFT 2 -> gk
gj OR gu -> gv
NOT jh -> ji
ap LSHIFT 1 -> bj
NOT ls -> lt
ir LSHIFT 1 -> jl
bn AND by -> ca
lv LSHIFT 15 -> lz
ba AND bc -> bd
cy LSHIFT 15 -> dc
ln AND lp -> lq
x RSHIFT 1 -> aq
gk OR gq -> gr
NOT kx -> ky
jg AND ji -> jj
bn OR by -> bz
fl LSHIFT 1 -> gf
bp OR bq -> br
he OR hp -> hq
et RSHIFT 5 -> ew
iu RSHIFT 2 -> iv
gl AND gm -> go
x OR ai -> aj
hc OR hd -> he
lg AND lm -> lo
lh OR li -> lj
da LSHIFT 1 -> du
fo RSHIFT 2 -> fp
gk AND gq -> gs
bj OR bi -> bk
lf OR lq -> lr
cj AND cp -> cr
hu LSHIFT 15 -> hy
1 AND bh -> bi
fo RSHIFT 3 -> fq
NOT lo -> lp
hw LSHIFT 1 -> iq
dd RSHIFT 1 -> dw
dt LSHIFT 15 -> dx
dy AND ej -> el
an LSHIFT 15 -> ar
aq OR ar -> as
1 AND r -> s
fw AND fy -> fz
NOT im -> in
et RSHIFT 3 -> ev
1 AND ds -> dt
ec AND ee -> ef
NOT ak -> al
jl OR jk -> jm
1 AND en -> eo
lb OR la -> lc
iu AND jf -> jh
iu RSHIFT 5 -> ix
bo AND bu -> bw
cz OR cy -> da
iv AND jb -> jd
iw AND ix -> iz
lf RSHIFT 1 -> ly
iu OR jf -> jg
NOT dm -> dn
lw OR lv -> lx
gg LSHIFT 1 -> ha
lr AND lt -> lu
fm OR fn -> fo
he RSHIFT 3 -> hg
aj AND al -> am
1 AND kz -> la
dy RSHIFT 5 -> eb
jc AND je -> jf
cm AND co -> cp
gv AND gx -> gy
ev OR ew -> ex
jp AND ka -> kc
fk OR fj -> fl
dy RSHIFT 3 -> ea
NOT bs -> bt
NOT ag -> ah
dz AND ef -> eh
cf LSHIFT 1 -> cz
NOT cv -> cw
1 AND cx -> cy
de AND dk -> dm
ck AND cl -> cn
x RSHIFT 5 -> aa
dv LSHIFT 1 -> ep
he RSHIFT 2 -> hf
NOT bw -> bx
ck OR cl -> cm
bp AND bq -> bs
as OR bd -> be
he AND hp -> hr
ev AND ew -> ey
1 AND lu -> lv
kk RSHIFT 3 -> km
b AND n -> p
NOT kc -> kd
lc LSHIFT 1 -> lw
km OR kn -> ko
id AND if -> ig
ih AND ij -> ik
jr AND js -> ju
ci RSHIFT 5 -> cl
hz RSHIFT 1 -> is
1 AND ke -> kf
NOT gs -> gt
aw AND ay -> az
x RSHIFT 2 -> y
ab AND ad -> ae
ff AND fh -> fi
ci AND ct -> cv
eq LSHIFT 1 -> fk
gj RSHIFT 3 -> gl
u LSHIFT 1 -> ao
NOT bb -> bc
NOT hj -> hk
kw AND ky -> kz
as AND bd -> bf
dw OR dx -> dy
br AND bt -> bu
kk AND kv -> kx
ep OR eo -> eq
he RSHIFT 1 -> hx
ki OR kj -> kk
NOT ju -> jv
ek AND em -> en
kk RSHIFT 5 -> kn
NOT eh -> ei
hx OR hy -> hz
ea OR eb -> ec
s LSHIFT 15 -> w
fo RSHIFT 1 -> gh
kk OR kv -> kw
bn RSHIFT 5 -> bq
NOT ed -> ee
1 AND ht -> hu
cu AND cw -> cx
b RSHIFT 5 -> f
kl AND kr -> kt
iq OR ip -> ir
ci RSHIFT 2 -> cj
cj OR cp -> cq
o AND q -> r
dd RSHIFT 5 -> dg
b RSHIFT 2 -> d
ks AND ku -> kv
b RSHIFT 3 -> e
d OR j -> k
NOT p -> q
NOT cr -> cs
du OR dt -> dv
kf LSHIFT 15 -> kj
NOT ac -> ad
fo RSHIFT 5 -> fr
hz OR ik -> il
jx AND jz -> ka
gh OR gi -> gj
kk RSHIFT 1 -> ld
hz RSHIFT 5 -> ic
as RSHIFT 2 -> at
NOT jy -> jz
1 AND am -> an
ci OR ct -> cu
hg AND hh -> hj
jq OR jw -> jx
v OR w -> x
la LSHIFT 15 -> le
dh AND dj -> dk
dp AND dr -> ds
jq AND jw -> jy
au OR av -> aw
NOT bf -> bg
z OR aa -> ab
ga AND gc -> gd
hz AND ik -> im
jt AND jv -> jw
z AND aa -> ac
jr OR js -> jt
hb LSHIFT 1 -> hv
hf OR hl -> hm
ib OR ic -> id
fq OR fr -> fs
cq AND cs -> ct
ia OR ig -> ih
dd OR do -> dp
d AND j -> l
ib AND ic -> ie
as RSHIFT 3 -> au
be AND bg -> bh
dd AND do -> dq
NOT l -> m
1 AND gd -> ge
y AND ae -> ag
fo AND fz -> gb
NOT ie -> if
e AND f -> h
x RSHIFT 3 -> z
y OR ae -> af
hf AND hl -> hn
NOT h -> i
NOT hn -> ho
he RSHIFT 5 -> hh
";
        // let c = Circuit::from(s);
        // println!("{}", c);
    }
}
