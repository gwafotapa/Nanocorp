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

    pub fn add_gate_and_value(
        &mut self,
        output: impl Into<String>,
        input: impl Into<String>,
        value: u16,
    ) -> Result<(), Error> {
        let gate = Gate::and_value(input, value)?;
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

    pub fn add_gate_or_value(
        &mut self,
        output: impl Into<String>,
        input: impl Into<String>,
        value: u16,
    ) -> Result<(), Error> {
        let gate = Gate::or_value(input, value)?;
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
                        self.set_signal(id, Some(*value));
                        ids.pop();
                    }
                    WireInput::Wire(input_id) => {
                        if let Some(signal) = self.get_signal(input_id) {
                            self.set_signal(id, Some(signal));
                            ids.pop();
                        } else {
                            ids.push(input_id.to_string());
                        }
                    }
                    WireInput::Gate(gate) => match gate {
                        Gate::And { input1, input2 } => {
                            if let Some(signal1) = self.get_signal(input1) {
                                if let Some(signal2) = self.get_signal(input2) {
                                    self.set_signal(id, Some(signal1 & signal2));
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
                        Gate::AndValue { input, value } => {
                            if let Some(signal1) = self.get_signal(input) {
                                self.set_signal(id, Some(signal1 & value));
                                ids.pop();
                            } else {
                                ids.push(input.to_string());
                            }
                        }
                        Gate::Or { input1, input2 } => {
                            if let Some(signal1) = self.get_signal(input1) {
                                if let Some(signal2) = self.get_signal(input2) {
                                    self.set_signal(id, Some(signal1 | signal2));
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
                        Gate::OrValue { input, value } => {
                            if let Some(signal1) = self.get_signal(input) {
                                self.set_signal(id, Some(signal1 | value));
                                ids.pop();
                            } else {
                                ids.push(input.to_string());
                            }
                        }
                        Gate::SLL { input, shift } => {
                            if let Some(signal) = self.get_signal(input) {
                                self.set_signal(id, Some(signal << shift));
                                ids.pop();
                            } else {
                                ids.push(input.to_string());
                            }
                        }
                        Gate::SLR { input, shift } => {
                            if let Some(signal) = self.get_signal(input) {
                                self.set_signal(id, Some(signal >> shift));
                                ids.pop();
                            } else {
                                ids.push(input.to_string());
                            }
                        }
                        Gate::Not { input } => {
                            if let Some(signal) = self.get_signal(input) {
                                self.set_signal(id, Some(!signal));
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

    fn set_signal(&mut self, id: &str, signal: Option<u16>) -> bool {
        // self.wires
        //     .get_mut(id)
        //     .map(|wire| wire.signal = signal)
        //     .is_some()
        if let Some(wire) = self.wires.get_mut(id) {
            wire.signal = signal;
            true
        } else {
            false
        }
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
        for wire in s.trim_end().split('\n') {
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
        let s = "lf AND lq -> ls\n\
		 iu RSHIFT 1 -> jn\n\
		 bo OR bu -> bv\n\
		 gj RSHIFT 1 -> hc\n\
		 et RSHIFT 2 -> eu\n\
		 bv AND bx -> by\n\
		 is OR it -> iu\n\
		 b OR n -> o\n\
		 gf OR ge -> gg\n\
		 NOT kt -> ku\n\
		 ea AND eb -> ed\n\
		 kl OR kr -> ks\n\
		 hi AND hk -> hl\n\
		 au AND av -> ax\n\
		 lf RSHIFT 2 -> lg\n\
		 dd RSHIFT 3 -> df\n\
		 eu AND fa -> fc\n\
		 df AND dg -> di\n\
		 ip LSHIFT 15 -> it\n\
		 NOT el -> em\n\
		 et OR fe -> ff\n\
		 fj LSHIFT 15 -> fn\n\
		 t OR s -> u\n\
		 ly OR lz -> ma\n\
		 ko AND kq -> kr\n\
		 NOT fx -> fy\n\
		 et RSHIFT 1 -> fm\n\
		 eu OR fa -> fb\n\
		 dd RSHIFT 2 -> de\n\
		 NOT go -> gp\n\
		 kb AND kd -> ke\n\
		 hg OR hh -> hi\n\
		 jm LSHIFT 1 -> kg\n\
		 NOT cn -> co\n\
		 jp RSHIFT 2 -> jq\n\
		 jp RSHIFT 5 -> js\n\
		 1 AND io -> ip\n\
		 eo LSHIFT 15 -> es\n\
		 1 AND jj -> jk\n\
		 g AND i -> j\n\
		 ci RSHIFT 3 -> ck\n\
		 gn AND gp -> gq\n\
		 fs AND fu -> fv\n\
		 lj AND ll -> lm\n\
		 jk LSHIFT 15 -> jo\n\
		 iu RSHIFT 3 -> iw\n\
		 NOT ii -> ij\n\
		 1 AND cc -> cd\n\
		 bn RSHIFT 3 -> bp\n\
		 NOT gw -> gx\n\
		 NOT ft -> fu\n\
		 jn OR jo -> jp\n\
		 iv OR jb -> jc\n\
		 hv OR hu -> hw\n\
		 19138 -> b\n\
		 gj RSHIFT 5 -> gm\n\
		 hq AND hs -> ht\n\
		 dy RSHIFT 1 -> er\n\
		 ao OR an -> ap\n\
		 ld OR le -> lf\n\
		 bk LSHIFT 1 -> ce\n\
		 bz AND cb -> cc\n\
		 bi LSHIFT 15 -> bm\n\
		 il AND in -> io\n\
		 af AND ah -> ai\n\
		 as RSHIFT 1 -> bl\n\
		 lf RSHIFT 3 -> lh\n\
		 er OR es -> et\n\
		 NOT ax -> ay\n\
		 ci RSHIFT 1 -> db\n\
		 et AND fe -> fg\n\
		 lg OR lm -> ln\n\
		 k AND m -> n\n\
		 hz RSHIFT 2 -> ia\n\
		 kh LSHIFT 1 -> lb\n\
		 NOT ey -> ez\n\
		 NOT di -> dj\n\
		 dz OR ef -> eg\n\
		 lx -> a\n\
		 NOT iz -> ja\n\
		 gz LSHIFT 15 -> hd\n\
		 ce OR cd -> cf\n\
		 fq AND fr -> ft\n\
		 at AND az -> bb\n\
		 ha OR gz -> hb\n\
		 fp AND fv -> fx\n\
		 NOT gb -> gc\n\
		 ia AND ig -> ii\n\
		 gl OR gm -> gn\n\
		 0 -> c\n\
		 NOT ca -> cb\n\
		 bn RSHIFT 1 -> cg\n\
		 c LSHIFT 1 -> t\n\
		 iw OR ix -> iy\n\
		 kg OR kf -> kh\n\
		 dy OR ej -> ek\n\
		 km AND kn -> kp\n\
		 NOT fc -> fd\n\
		 hz RSHIFT 3 -> ib\n\
		 NOT dq -> dr\n\
		 NOT fg -> fh\n\
		 dy RSHIFT 2 -> dz\n\
		 kk RSHIFT 2 -> kl\n\
		 1 AND fi -> fj\n\
		 NOT hr -> hs\n\
		 jp RSHIFT 1 -> ki\n\
		 bl OR bm -> bn\n\
		 1 AND gy -> gz\n\
		 gr AND gt -> gu\n\
		 db OR dc -> dd\n\
		 de OR dk -> dl\n\
		 as RSHIFT 5 -> av\n\
		 lf RSHIFT 5 -> li\n\
		 hm AND ho -> hp\n\
		 cg OR ch -> ci\n\
		 gj AND gu -> gw\n\
		 ge LSHIFT 15 -> gi\n\
		 e OR f -> g\n\
		 fp OR fv -> fw\n\
		 fb AND fd -> fe\n\
		 cd LSHIFT 15 -> ch\n\
		 b RSHIFT 1 -> v\n\
		 at OR az -> ba\n\
		 bn RSHIFT 2 -> bo\n\
		 lh AND li -> lk\n\
		 dl AND dn -> do\n\
		 eg AND ei -> ej\n\
		 ex AND ez -> fa\n\
		 NOT kp -> kq\n\
		 NOT lk -> ll\n\
		 x AND ai -> ak\n\
		 jp OR ka -> kb\n\
		 NOT jd -> je\n\
		 iy AND ja -> jb\n\
		 jp RSHIFT 3 -> jr\n\
		 fo OR fz -> ga\n\
		 df OR dg -> dh\n\
		 gj RSHIFT 2 -> gk\n\
		 gj OR gu -> gv\n\
		 NOT jh -> ji\n\
		 ap LSHIFT 1 -> bj\n\
		 NOT ls -> lt\n\
		 ir LSHIFT 1 -> jl\n\
		 bn AND by -> ca\n\
		 lv LSHIFT 15 -> lz\n\
		 ba AND bc -> bd\n\
		 cy LSHIFT 15 -> dc\n\
		 ln AND lp -> lq\n\
		 x RSHIFT 1 -> aq\n\
		 gk OR gq -> gr\n\
		 NOT kx -> ky\n\
		 jg AND ji -> jj\n\
		 bn OR by -> bz\n\
		 fl LSHIFT 1 -> gf\n\
		 bp OR bq -> br\n\
		 he OR hp -> hq\n\
		 et RSHIFT 5 -> ew\n\
		 iu RSHIFT 2 -> iv\n\
		 gl AND gm -> go\n\
		 x OR ai -> aj\n\
		 hc OR hd -> he\n\
		 lg AND lm -> lo\n\
		 lh OR li -> lj\n\
		 da LSHIFT 1 -> du\n\
		 fo RSHIFT 2 -> fp\n\
		 gk AND gq -> gs\n\
		 bj OR bi -> bk\n\
		 lf OR lq -> lr\n\
		 cj AND cp -> cr\n\
		 hu LSHIFT 15 -> hy\n\
		 1 AND bh -> bi\n\
		 fo RSHIFT 3 -> fq\n\
		 NOT lo -> lp\n\
		 hw LSHIFT 1 -> iq\n\
		 dd RSHIFT 1 -> dw\n\
		 dt LSHIFT 15 -> dx\n\
		 dy AND ej -> el\n\
		 an LSHIFT 15 -> ar\n\
		 aq OR ar -> as\n\
		 1 AND r -> s\n\
		 fw AND fy -> fz\n\
		 NOT im -> in\n\
		 et RSHIFT 3 -> ev\n\
		 1 AND ds -> dt\n\
		 ec AND ee -> ef\n\
		 NOT ak -> al\n\
		 jl OR jk -> jm\n\
		 1 AND en -> eo\n\
		 lb OR la -> lc\n\
		 iu AND jf -> jh\n\
		 iu RSHIFT 5 -> ix\n\
		 bo AND bu -> bw\n\
		 cz OR cy -> da\n\
		 iv AND jb -> jd\n\
		 iw AND ix -> iz\n\
		 lf RSHIFT 1 -> ly\n\
		 iu OR jf -> jg\n\
		 NOT dm -> dn\n\
		 lw OR lv -> lx\n\
		 gg LSHIFT 1 -> ha\n\
		 lr AND lt -> lu\n\
		 fm OR fn -> fo\n\
		 he RSHIFT 3 -> hg\n\
		 aj AND al -> am\n\
		 1 AND kz -> la\n\
		 dy RSHIFT 5 -> eb\n\
		 jc AND je -> jf\n\
		 cm AND co -> cp\n\
		 gv AND gx -> gy\n\
		 ev OR ew -> ex\n\
		 jp AND ka -> kc\n\
		 fk OR fj -> fl\n\
		 dy RSHIFT 3 -> ea\n\
		 NOT bs -> bt\n\
		 NOT ag -> ah\n\
		 dz AND ef -> eh\n\
		 cf LSHIFT 1 -> cz\n\
		 NOT cv -> cw\n\
		 1 AND cx -> cy\n\
		 de AND dk -> dm\n\
		 ck AND cl -> cn\n\
		 x RSHIFT 5 -> aa\n\
		 dv LSHIFT 1 -> ep\n\
		 he RSHIFT 2 -> hf\n\
		 NOT bw -> bx\n\
		 ck OR cl -> cm\n\
		 bp AND bq -> bs\n\
		 as OR bd -> be\n\
		 he AND hp -> hr\n\
		 ev AND ew -> ey\n\
		 1 AND lu -> lv\n\
		 kk RSHIFT 3 -> km\n\
		 b AND n -> p\n\
		 NOT kc -> kd\n\
		 lc LSHIFT 1 -> lw\n\
		 km OR kn -> ko\n\
		 id AND if -> ig\n\
		 ih AND ij -> ik\n\
		 jr AND js -> ju\n\
		 ci RSHIFT 5 -> cl\n\
		 hz RSHIFT 1 -> is\n\
		 1 AND ke -> kf\n\
		 NOT gs -> gt\n\
		 aw AND ay -> az\n\
		 x RSHIFT 2 -> y\n\
		 ab AND ad -> ae\n\
		 ff AND fh -> fi\n\
		 ci AND ct -> cv\n\
		 eq LSHIFT 1 -> fk\n\
		 gj RSHIFT 3 -> gl\n\
		 u LSHIFT 1 -> ao\n\
		 NOT bb -> bc\n\
		 NOT hj -> hk\n\
		 kw AND ky -> kz\n\
		 as AND bd -> bf\n\
		 dw OR dx -> dy\n\
		 br AND bt -> bu\n\
		 kk AND kv -> kx\n\
		 ep OR eo -> eq\n\
		 he RSHIFT 1 -> hx\n\
		 ki OR kj -> kk\n\
		 NOT ju -> jv\n\
		 ek AND em -> en\n\
		 kk RSHIFT 5 -> kn\n\
		 NOT eh -> ei\n\
		 hx OR hy -> hz\n\
		 ea OR eb -> ec\n\
		 s LSHIFT 15 -> w\n\
		 fo RSHIFT 1 -> gh\n\
		 kk OR kv -> kw\n\
		 bn RSHIFT 5 -> bq\n\
		 NOT ed -> ee\n\
		 1 AND ht -> hu\n\
		 cu AND cw -> cx\n\
		 b RSHIFT 5 -> f\n\
		 kl AND kr -> kt\n\
		 iq OR ip -> ir\n\
		 ci RSHIFT 2 -> cj\n\
		 cj OR cp -> cq\n\
		 o AND q -> r\n\
		 dd RSHIFT 5 -> dg\n\
		 b RSHIFT 2 -> d\n\
		 ks AND ku -> kv\n\
		 b RSHIFT 3 -> e\n\
		 d OR j -> k\n\
		 NOT p -> q\n\
		 NOT cr -> cs\n\
		 du OR dt -> dv\n\
		 kf LSHIFT 15 -> kj\n\
		 NOT ac -> ad\n\
		 fo RSHIFT 5 -> fr\n\
		 hz OR ik -> il\n\
		 jx AND jz -> ka\n\
		 gh OR gi -> gj\n\
		 kk RSHIFT 1 -> ld\n\
		 hz RSHIFT 5 -> ic\n\
		 as RSHIFT 2 -> at\n\
		 NOT jy -> jz\n\
		 1 AND am -> an\n\
		 ci OR ct -> cu\n\
		 hg AND hh -> hj\n\
		 jq OR jw -> jx\n\
		 v OR w -> x\n\
		 la LSHIFT 15 -> le\n\
		 dh AND dj -> dk\n\
		 dp AND dr -> ds\n\
		 jq AND jw -> jy\n\
		 au OR av -> aw\n\
		 NOT bf -> bg\n\
		 z OR aa -> ab\n\
		 ga AND gc -> gd\n\
		 hz AND ik -> im\n\
		 jt AND jv -> jw\n\
		 z AND aa -> ac\n\
		 jr OR js -> jt\n\
		 hb LSHIFT 1 -> hv\n\
		 hf OR hl -> hm\n\
		 ib OR ic -> id\n\
		 fq OR fr -> fs\n\
		 cq AND cs -> ct\n\
		 ia OR ig -> ih\n\
		 dd OR do -> dp\n\
		 d AND j -> l\n\
		 ib AND ic -> ie\n\
		 as RSHIFT 3 -> au\n\
		 be AND bg -> bh\n\
		 dd AND do -> dq\n\
		 NOT l -> m\n\
		 1 AND gd -> ge\n\
		 y AND ae -> ag\n\
		 fo AND fz -> gb\n\
		 NOT ie -> if\n\
		 e AND f -> h\n\
		 x RSHIFT 3 -> z\n\
		 y OR ae -> af\n\
		 hf AND hl -> hn\n\
		 NOT h -> i\n\
		 NOT hn -> ho\n\
		 he RSHIFT 5 -> hh";
        let mut c = Circuit::from(s);
        c.compute_signals();
        println!("{}", c);
    }
}
