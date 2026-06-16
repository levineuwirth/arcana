//! Enslaved Scout — `{2}{R}` 2/2 Creature — Goblin Scout.
//! `{2}: This creature gains mountainwalk until end of turn.`

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
    let name = reg.interner_mut().intern("Enslaved Scout");
    let goblin = reg.interner_mut().intern("Goblin");
    let scout = reg.interner_mut().intern("Scout");
    let _mountain = reg.interner_mut().intern("Mountain");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(scout);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}: This creature gains mountainwalk until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_mountainwalk,
            }),
    )
}

fn gain_mountainwalk(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mountain = reg.interner().lookup("Mountain").expect("Mountain interned");
    vec![Effect::GrantKeyword {
        target: ctx.source,
        keyword: KeywordAbility::Landwalk(mountain),
        duration: Duration::EndOfTurn,
    }]
}
