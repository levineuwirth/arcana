//! Sunstone — `{3}` artifact.
//! "{2}, Sacrifice a snow land: Prevent all combat damage that would be
//! dealt this turn." The sacrifice-a-snow-land cost uses `sacrifice_other`;
//! the prevention is modeled as a board-wide creature-sourced prevention
//! (combat damage is dealt by creatures) — the combat-only restriction is a
//! documented fidelity gap.

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
use arcana_core::types::{CardId, ColorSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sunstone");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}, Sacrifice a snow land: Prevent all combat damage that would be dealt this turn.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                sacrifice_other: Some(
                    ObjectFilter::new()
                        .with_types(TypeLine::LAND.into())
                        .with_supertypes(SupertypeSet::new().with(SupertypeSet::SNOW)),
                ),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: fog,
        }),
    )
}

fn fog(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "prevent all COMBAT damage" — `PreventDamageFrom` has no
    // combat-only flag; approximated as preventing all damage from creatures
    // to any target this turn (combat damage is always dealt by creatures;
    // creature pinger damage is over-prevented).
    vec![Effect::PreventDamageFrom {
        source_filter: ObjectFilter::creature(),
        target_filter: TargetFilter::AnyTarget,
        amount: None,
        duration: ReplacementDuration::EndOfTurn,
    }]
}
