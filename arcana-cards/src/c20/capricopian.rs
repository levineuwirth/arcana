//! Capricopian — `{X}{G}` 0/0 Goat Hydra.
//! "This creature enters with X +1/+1 counters on it."
//! "{2}: Put a +1/+1 counter on this creature, then you may reselect which player
//!  this creature is attacking. Only the player this creature is attacking may
//!  activate this ability and only during the declare attackers step."
//!
//! GAP: "enters with X +1/+1 counters" (X from the cost) is engine-handled
//! enters-with machinery, not a triggered/activated ability or effect, and is not
//! expressible here.
//! GAP: "then you may reselect which player this creature is attacking" and the
//! "only the attacked player may activate, only during declare attackers"
//! restriction are not expressible (no reselect-defender effect, no
//! attacked-player activation gate). Only the +1/+1 counter half is implemented.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Capricopian");
    let goat = reg.interner_mut().intern("Goat");
    let hydra = reg.interner_mut().intern("Hydra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goat);
    subtypes.0.insert(hydra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}: Put a +1/+1 counter on this creature, then you may reselect which \
                   player this creature is attacking."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: add_counter_self,
        }),
    )
}

fn add_counter_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: attacking-player reselection rider not expressible.
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
