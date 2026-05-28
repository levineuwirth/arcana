//! Dark Apostle — `{3}{R}` 3/3 red Astartes Warlock.
//! Gift of Chaos — `{3}, {T}: The next noncreature spell you cast this
//! turn has cascade.`
//!
//! GAP: "the next noncreature spell you cast this turn has cascade"
//! grants a delayed/conditional cascade effect on future spells.
//! No Effect variant for "grant cascade to next spell cast this turn".

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dark Apostle");
    let astartes = reg.interner_mut().intern("Astartes");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(astartes);
    subtypes.0.insert(warlock);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Gift of Chaos — {3}, {T}: The next noncreature spell you cast this turn has cascade.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}").unwrap(),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: grant_cascade_to_next_spell,
            }),
    )
}

fn grant_cascade_to_next_spell(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no Effect variant for "grant cascade to next noncreature spell
    // cast this turn". DelayedAction only handles known object ids.
    Vec::new()
}
