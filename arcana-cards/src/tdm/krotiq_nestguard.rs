//! Krotiq Nestguard — `{2}{G}` 4/4 Insect with Defender.
//!
//! Oracle:
//! * Defender.
//! * "{2}{G}: This creature can attack this turn as though it didn't have
//!   defender." — a mana activation; there is no Effect to grant
//!   "can attack as though it didn't have defender", so the ability shape
//!   is emitted with a GAP'd effect body.

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
    let name = reg.interner_mut().intern("Krotiq Nestguard");
    let insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{G}: This creature can attack this turn as though it didn't have defender.".into(),
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
            effect: ignore_defender,
        }),
    )
}

fn ignore_defender(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "can attack this turn as though it didn't have defender" — no
    // Effect grants attack-despite-defender permission.
    Vec::new()
}
