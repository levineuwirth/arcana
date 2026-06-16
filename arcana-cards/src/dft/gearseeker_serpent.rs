//! Gearseeker Serpent — `{5}{U}{U}` 5/6 Serpent with Affinity for
//! artifacts.
//! "{5}{U}: This creature can't be blocked this turn."

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gearseeker Serpent");
    let serpent = reg.interner_mut().intern("Serpent");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(serpent);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(6)),
        // GAP: Affinity for artifacts (cost reduction) is not in the usable
        // KeywordAbility surface.
        keywords: vec![],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{5}{U}: This creature can't be blocked this turn.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{5}{U}").expect("valid cost"),
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

fn cant_be_blocked(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::CantBeBlocked {
        target: ctx.source,
        duration: Duration::EndOfTurn,
    }]
}
