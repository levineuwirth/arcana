//! Arco-Flagellant — `{2}{B}` 3/1 Human.
//!
//! Oracle text:
//! * Squad {2} (additional-cost copy mechanic on cast).
//! * This creature can't block.
//! * Endurant — Pay 3 life: This creature gains indestructible until end of turn.
//!
//! Squad and Endurant are not usable `KeywordAbility` variants (not in the
//! supported keyword surface), so the keyword line is empty and Squad is
//! GAP'd. The "Endurant" ability is a plain "Pay 3 life: …" activated ability,
//! which IS expressible (the keyword name is just flavor). The static
//! "can't block" has no trigger/activation hook and is GAP'd.

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
    let name = reg.interner_mut().intern("Arco-Flagellant");
    let human = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Squad {2} keyword (additional-cost copy-on-ETB) not a usable
        // KeywordAbility variant. GAP: Endurant keyword marker not usable.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static "This creature can't block." — pure static restriction with no
    // trigger/activation hook to apply an effect.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Pay 3 life: This creature gains indestructible until end of turn.".into(),
            cost: ActivationCost {
                life: 3,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: gain_indestructible,
        }),
    )
}

fn gain_indestructible(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GrantKeyword {
        target: ctx.source,
        keyword: KeywordAbility::Indestructible,
        duration: Duration::EndOfTurn,
    }]
}
