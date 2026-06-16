//! Deeptread Merrow — `{1}{U}` 2/1 Merfolk Rogue.
//! `{U}: This creature gains islandwalk until end of turn.`

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
    let name = reg.interner_mut().intern("Deeptread Merrow");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let rogue = reg.interner_mut().intern("Rogue");
    let _island = reg.interner_mut().intern("Island");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{U}: This creature gains islandwalk until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_islandwalk,
            }),
    )
}

fn gain_islandwalk(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let island = reg.interner().lookup("Island").expect("Island interned");
    vec![Effect::GrantKeyword {
        target: ctx.source,
        keyword: KeywordAbility::Landwalk(island),
        duration: Duration::EndOfTurn,
    }]
}
