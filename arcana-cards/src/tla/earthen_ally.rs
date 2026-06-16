//! Earthen Ally — `{G}` 0/2 Human Soldier Ally.
//!
//! * "This creature gets +1/+0 for each color among Allies you
//!   control." — a pure static continuous ability (no trigger word, no
//!   cost). Not expressible as a triggered/activated ability → GAP.
//! * Scryfall keyword "Earthbend" is not in the usable keyword surface
//!   (and the Earthbend mechanic is unmodeled) → `keywords: vec![]`.
//! * "{2}{W}{U}{B}{R}{G}: Earthbend 5." — an activated ability whose
//!   effect (Earthbend N: animate a target land into a 0/0 with haste,
//!   put five +1/+1 counters, with a dies/exile return trigger) has no
//!   `Effect` primitive → the ability is emitted with a GAP'd effect.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP (static): "This creature gets +1/+0 for each color among Allies
// you control." — pure continuous static, no trigger/cost to attach.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Earthen Ally");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    subtypes.0.insert(ally);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{W}{U}{B}{R}{G}: Earthbend 5.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{W}{U}{B}{R}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: earthbend_five,
            }),
    )
}

fn earthbend_five(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Earthbend N has no Effect primitive (animate target land into
    // a 0/0 haste creature-land, put N +1/+1 counters, dies/exile-return).
    Vec::new()
}
