//! Chameleon Colossus — `{2}{G}{G}` 4/4 green Shapeshifter.
//!
//! Oracle:
//! * Changeling (this card is every creature type)
//! * Protection from black (GAP — Protection not in usable keyword surface)
//! * "{2}{G}{G}: This creature gets +X/+X until end of turn, where X is
//!   its power."
//!
//! Changeling is wired. Protection from black is GAP'd. The pump
//! activation computes X = the creature's current power at resolution.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chameleon Colossus");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);

    // GAP: "Protection from black" — Protection is not in the usable
    // keyword surface for this card class.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Changeling],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{2}{G}{G}: Chameleon Colossus gets +X/+X until end of turn, where X is its power.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{G}{G}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: pump_by_power,
        }),
    )
}

fn pump_by_power(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let x = script::power_of(state, ctx.source).max(0);
    vec![Effect::Pump {
        target: ctx.source,
        power: x,
        toughness: x,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
