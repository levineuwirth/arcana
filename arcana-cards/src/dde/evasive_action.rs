//! Evasive Action — `{1}{U}` instant. "Domain — Counter target spell
//! unless its controller pays {1} for each basic land type among
//! lands you control." We need a tax that varies with Domain. The
//! catalog's `CounterUnlessPays` takes a fixed ManaCost — best
//! approximation: build "{N}" generic where N = Domain count.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Evasive Action");
    let _ = reg.interner_mut().intern("Plains");
    let _ = reg.interner_mut().intern("Island");
    let _ = reg.interner_mut().intern("Swamp");
    let _ = reg.interner_mut().intern("Mountain");
    let _ = reg.interner_mut().intern("Forest");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Domain — Counter target spell unless its controller pays {1} for each basic land type among lands you control.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Spell(ObjectFilter::default()),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let mut domain: u32 = 0;
    for ty in ["Plains", "Island", "Swamp", "Mountain", "Forest"] {
        let f = script::subtype_filter(reg, ty).controlled_by(ControllerConstraint::You);
        if script::count_matching(state, &f, entry.controller) > 0 {
            domain += 1;
        }
    }
    let cost_str = format!("{{{}}}", domain);
    let cost = ManaCost::parse(&cost_str).expect("valid generic cost");
    vec![Effect::CounterUnlessPays { target: *id, cost }]
}
