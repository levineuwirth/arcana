//! Snowmelt Stag — `{3}{U}` 2/5 blue Creature — Elemental Elk.
//! Vigilance.
//! "During your turn, this creature has base power and toughness 5/2." (static)
//! "{5}{U}{U}: This creature can't be blocked this turn."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Snowmelt Stag");
    let elemental = reg.interner_mut().intern("Elemental");
    let elk = reg.interner_mut().intern("Elk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(elk);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };
    // GAP: static "During your turn, this creature has base power and toughness 5/2"
    // is a conditional continuous self-effect (no triggered/activated hook, and no
    // your-turn Duration); not expressible with the demonstrated API.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{5}{U}{U}: This creature can't be blocked this turn.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{5}{U}{U}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: cant_be_blocked,
        }),
    )
}

fn cant_be_blocked(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::CantBeBlocked {
        target: ctx.source,
        duration: Duration::EndOfTurn,
    }]
}
