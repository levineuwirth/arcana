//! Two-Headed Dragon — `{4}{R}{R}` 4/4 Dragon with Flying and Menace.
//! "This creature can block an additional creature each combat."
//! "{1}{R}: This creature gets +2/+0 until end of turn."
//!
//! Flying and Menace are usable keywords. The "can block an additional
//! creature" static has no expressible primitive (GAP). The pump
//! activation is wired.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Two-Headed Dragon");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Menace],
        ..Default::default()
    };
    // GAP: "can block an additional creature each combat" — static, no
    // expressible primitive.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}: This creature gets +2/+0 until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_self,
            }),
    )
}

fn pump_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 2,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
