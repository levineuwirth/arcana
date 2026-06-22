//! Niblis of the Breath — `{2}{U}` 2/1 Spirit.
//!
//! * Flying.
//! * `{U}, {T}: You may tap or untap target creature.` — the player's
//!   choice between tapping and untapping the target has no demonstrated
//!   primitive (neither `Effect::Tap` nor `Effect::Untap` alone is
//!   faithful, and there is no tap-or-untap-choice effect). The ability
//!   (cost + target) is emitted; its effect body is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Niblis of the Breath");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{U}, {T}: You may tap or untap target creature.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{U}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: tap_or_untap,
        }),
    )
}

fn tap_or_untap(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "tap or untap target creature" — the player-chosen direction
    // has no demonstrated primitive.
    Vec::new()
}
