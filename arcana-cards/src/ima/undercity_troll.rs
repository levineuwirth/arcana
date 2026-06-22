//! Undercity Troll — `{1}{G}` 2/2 green Troll.
//! Renown 1 (When this creature deals combat damage to a player, if it isn't
//! renowned, put a +1/+1 counter on it and it becomes renowned.)
//! {2}{G}: Regenerate this creature.
//!
//! Renown 1 is a base keyword whose trigger the engine synthesizes. The
//! regenerate ability is a mana-only activation.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Undercity Troll");
    let troll = reg.interner_mut().intern("Troll");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(troll);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Renown(1)],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{G}: Regenerate this creature.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{G}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: regenerate_self,
        }),
    )
}

fn regenerate_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Regenerate { target: ctx.source }]
}
