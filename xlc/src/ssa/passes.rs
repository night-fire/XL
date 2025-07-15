use super::*;
use std::collections::{HashMap, HashSet};

fn const_int(func:&Function, id: ValueId) -> Option<i64> {
    match func.values.get(&id)?.kind {
        ValueKind::Const(c) => Some(c),
        _ => None,
    }
}

pub struct ConstPropPass;
impl SSAPass for ConstPropPass {
    fn name(&self) -> &'static str { "constprop" }
    fn run(&self, func: &mut Function) -> bool {
        let mut changed = false;
        // very local const folding
        for block in func.blocks.values_mut() {
            let mut const_map = HashMap::new();
            for val_id in &block.values {
                if let Some(val) = func.values.get(val_id).cloned() {
                    match val.kind {
                        ValueKind::Add(a,b) | ValueKind::Sub(a,b) | ValueKind::Mul(a,b) | ValueKind::Div(a,b) => {
                            if let (Some(va), Some(vb)) = (const_int(func, a), const_int(func, b)) {
                                let res = match val.kind {
                                    ValueKind::Add(_,_) => va + vb,
                                    ValueKind::Sub(_,_) => va - vb,
                                    ValueKind::Mul(_,_) => va * vb,
                                    ValueKind::Div(_,_) => if vb!=0 { va / vb } else { continue },
                                    _ => unreachable!(),
                                };
                                *func.values.get_mut(val_id).unwrap() = Value { id:*val_id, kind: ValueKind::Const(res) };
                                changed = true;
                            }
                        }
                        _ => {}
                    }
                }
            }
            // TODO implement once Value storage exists
        }
        changed
    }
}

pub struct DcePass;
impl SSAPass for DcePass {
    fn name(&self) -> &'static str { "dce" }
    fn run(&self, func: &mut Function) -> bool {
        // mark values used in terminators only because we don't store use lists yet
        let mut used: HashSet<ValueId> = HashSet::new();
        for bb in func.blocks.values() {
            match &bb.terminator {
                Terminator::Ret(v) => { used.insert(*v); },
                Terminator::Br(_) => {},
                Terminator::CondBr { cond, .. } => { used.insert(*cond); },
            }
        }
        // If Value storage existed we'd iterate and remove dead ones.
        let mut changed = false;
        for block in func.blocks.values_mut() {
            block.values.retain(|vid| used.contains(vid));
        }
        let before = func.values.len();
        func.values.retain(|vid, _| used.contains(vid));
        if func.values.len()!=before { changed=true; }
        changed
    }
}