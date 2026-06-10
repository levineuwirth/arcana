//! Al-abara's Carpet — `{5}` artifact.
//! "{5}, {T}: Prevent all damage that would be dealt to you this turn
//! by attacking creatures without flying."
//! Wired with `Effect::PreventDamageFrom` (creature sources -> you,
//! all amounts, until end of turn). GAP: the source filter cannot
//! express "attacking" or "without flying" — no combat-status or
//! keyword predicates are available on `ObjectFilter` here, so the
//! prevention is broader (all creatures) than printed.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Al-abara's Carpet");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{5}, {T}: Prevent all damage that would be dealt to you this turn by attacking creatures without flying.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{5}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: prevent_creature_damage,
        }),
    )
}

fn prevent_creature_damage(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "attacking creatures without flying" — ObjectFilter here has
    // no attacking-status or keyword predicate; prevention is wired for
    // all creature sources.
    vec![Effect::PreventDamageFrom {
        source_filter: ObjectFilter::creature(),
        target_filter: TargetFilter::Player,
        amount: None,
        duration: ReplacementDuration::EndOfTurn,
    }]
}
